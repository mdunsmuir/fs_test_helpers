//! Some helpers (including some re-exports of other crates) for testing
//! stuff that touches the filesystem.
//!
//! # Features
//!
//! * Assertions for files and directories like `assert_exists`,
//!   `assert_file_has_contents`, and so on.
//! * `fake` module for quick-n-easy creation of directories and files.

extern crate tempdir;
pub use tempdir::TempDir;

pub mod fake;
pub use fake::Fake;

pub mod assertions;
pub use assertions::*;
