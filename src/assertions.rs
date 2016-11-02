//! `assert!` macros attached to filesystem queries.

use std::path::Path;
use std::fs;
use std::io::{Read, BufReader};

extern crate itertools;
use self::itertools::zip_eq;

macro_rules! path_method_assertion {
    (
        $our_method_name:ident,
        $path_method_name:ident,
        $message:expr
    ) => {
        pub fn $our_method_name<P: AsRef<Path>>(path: P) {
            assert!(
                path.as_ref().$path_method_name(),
                format!(
                    "path {:?} {}",
                    path.as_ref(),
                    $message
                )
            )
        }
    };
}

path_method_assertion!(assert_exists, exists, "does not exist");
path_method_assertion!(assert_is_dir, is_dir, "is not a directory");
path_method_assertion!(assert_is_file, is_file, "is not a file");

macro_rules! file_bytes {
    ($path:expr) => {
        BufReader::new(fs::File::open($path).unwrap()).bytes()
    }
}

pub fn assert_file_has_contents<P: AsRef<Path>>(path: P, contents: &[u8]) {
    assert_is_file(&path);
    let file_bytes = file_bytes!(&path);

    for (exp, act) in zip_eq(contents.iter(), file_bytes) {
        assert_eq!(exp, &act.unwrap());
    }
}

pub fn assert_files_have_same_contents<P: AsRef<Path>>(path_1: P, path_2: P) {
    assert_is_file(&path_1);
    assert_is_file(&path_2);

    for (b1, b2) in zip_eq(file_bytes!(&path_1), file_bytes!(&path_2)) {
        assert_eq!(b1.unwrap(), b2.unwrap());
    }
}

#[cfg(test)]
mod tests {

    extern crate std;
    use super::super::{Fake, TempDir};

    #[test]
    fn files_have_same_contents() {
        let temp_dir = TempDir::new("fth_test").unwrap();
        let file1 = Fake::file("one.file").fill_with_uuid();
        file1.create(&temp_dir).unwrap();

        std::fs::copy(
            temp_dir.as_ref().join("one.file"),
            temp_dir.as_ref().join("two.file"),
        ).unwrap();

        super::assert_files_have_same_contents(
            temp_dir.as_ref().join("one.file"),
            temp_dir.as_ref().join("two.file"),
        )
    }

    #[test]
    #[should_panic]
    fn files_have_different_contents() {
        let temp_dir = TempDir::new("fth_test").unwrap();
        let file1 = Fake::file("one.file").fill_with_uuid();
        let file2 = Fake::file("two.file").fill_with_uuid();

        file1.create(&temp_dir).unwrap();
        file2.create(&temp_dir).unwrap();

        super::assert_files_have_same_contents(
            temp_dir.as_ref().join("one.file"),
            temp_dir.as_ref().join("two.file"),
        )
    }

    #[test]
    #[should_panic]
    fn files_have_mostly_same_but_different_length_contents() {
        let temp_dir = TempDir::new("fth_test").unwrap();
        let file1 = Fake::file("one.file").fill_with("foobarbaz");
        let file2 = Fake::file("two.file").fill_with("foobar");

        file1.create(&temp_dir).unwrap();
        file2.create(&temp_dir).unwrap();

        super::assert_files_have_same_contents(
            temp_dir.as_ref().join("one.file"),
            temp_dir.as_ref().join("two.file"),
        )
    }
}
