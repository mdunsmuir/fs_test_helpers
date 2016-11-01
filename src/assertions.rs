//! `assert!` macros attached to filesystem queries.

use std::path::Path;
use std::fs;
use std::io::{Read, BufReader};

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

pub fn assert_file_has_contents<P: AsRef<Path>>(path: P, contents: &[u8]) {
    assert_is_file(&path);
    let file = BufReader::new(fs::File::open(path).unwrap());
    for (exp, act) in contents.iter().zip(file.bytes()) {
        assert_eq!(exp, &act.unwrap());
    }
}
