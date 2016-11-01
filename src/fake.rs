//! Create "fake" directories and files for testing purposes.

extern crate std;
extern crate uuid;

use std::fs;
use std::path::Path;
use std::string::ToString;
use std::io::Write;

/// This enum is used to represent a tree of directories and files that
/// can be created on the filesystem. The goal is quick setup and teardown
/// of files and directories for the purposes of testing.
///
/// # Basic Use
///
/// To create this structure:
///
/// ```text`
/// foo
/// ├── bar.file
/// └── baz
///     ├── fobe.file
///     └── quux.file
/// ```
///
/// You might do something like this:
///
/// ```
/// use self::fs_test_helpers::*;
///
/// let dir = Fake::dir(
///     "foo",
///     vec![
///         Fake::file("bar.file"),
///         Fake::dir(
///             "baz",
///             vec![
///                 Fake::file("fobe.file"),
///                 Fake::file("quux.file"),
///             ],
///         ),
///     ],
/// );
///
/// let base = TempDir::new("happy_little_prefix").unwrap();
/// dir.create(&base);
///
/// assert_is_dir(base.as_ref().join("foo"));
/// assert_is_file(base.as_ref().join("foo/bar.file"));
/// assert_is_dir(base.as_ref().join("foo/baz"));
/// assert_is_file(base.as_ref().join("foo/baz/fobe.file"));
/// assert_is_file(base.as_ref().join("foo/baz/quux.file"));
/// ```
///
/// # File Contents
///
/// You can include data in files like so (note the availability of the
/// more general `fill_with` method in addition to `fill_with_uuid`):
///
/// ```
/// use self::fs_test_helpers::*;
///
/// let file = Fake::file("foo.file").fill_with_uuid();
/// let base = TempDir::new("happy_little_prefix").unwrap();
/// file.create(&base);
///
/// assert_file_has_contents(
///     base.as_ref().join("foo.file"),
///
///     // this just retrieves whatever data we wrote to the file
///     file.contents().unwrap().as_bytes()
/// )
/// ```
///
/// # TODO
///
/// * Assertion that a `Fake` matches the filesystem object at a given path
/// * More builder stuff: permissions, timestamps, ...
pub enum Fake {
    Dir(String, Vec<Fake>),
    File(String, Option<String>),
}

use self::Fake::*;

impl Fake {

    pub fn dir<T: ToString>(name: T, contents: Vec<Self>) -> Self {
        Dir(name.to_string(), contents)
    }

    pub fn file<T: ToString>(name: T) -> Self {
        File(name.to_string(), None)
    }

    /// Create the filesystem objects described by this `Fake` under
    /// the path `base`.
    pub fn create<P: AsRef<Path>>(&self, base: P) -> std::io::Result<()> {
        match *self {
            Dir(ref name, ref contents) => {
                let path = base.as_ref().join(name);
                try!(fs::create_dir(&path));

                for thing in contents {
                    try!(thing.create(&path));
                }
            },

            File(ref name, ref maybe_contents) => {
                let path = base.as_ref().join(name);
                try!(fs::File::create(path).and_then(|mut file| {
                    match *maybe_contents {
                        None => Ok(()),
                        Some(ref contents) =>
                            file.write_all(contents.as_bytes())
                    }
                }));
            }
        }

        Ok(())
    }

    pub fn name(&self) -> &str {
        match *self {
            Dir(ref name, _) => name.as_str(),
            File(ref name, _) => name.as_str(),
        }
    }

    pub fn contents(&self) -> Option<&str> {
        match *self {
            Dir(..) => None,
            File(_, ref mcs) => mcs.as_ref().map(|ref cs| cs.as_str() )
        }
    }

    /// Write some data into a `Fake::File` when it gets created.
    ///
    /// # Panics
    ///
    /// * If called on a `Fake::Dir`
    ///   (only files can have contents written to them)
    pub fn fill_with<T: ToString>(self, contents: T) -> Self {
        match self {
            Dir(..) => panic!("cannot add contents to Dir"),
            File(name, _) => File(name, Some(contents.to_string())),
        }
    }

    /// Write a randomly generated uuid into a `Fake::File` when it
    /// gets created. Useful e.g. if you need to verify that a file
    /// has been copied intact.
    ///
    /// # Panics
    ///
    /// * If called on a `Fake::Dir`
    ///   (only files can have contents written to them)
    pub fn fill_with_uuid(self) -> Self {
        let uuid = uuid::Uuid::new_v4().simple().to_string();
        self.fill_with(uuid)
    }
}
