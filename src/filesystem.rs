use std::path::{Path, PathBuf};

use mockall::automock;

#[automock]
pub trait FsInt {
    fn exists(&self, path: &Path) -> bool;
    fn is_directory(&self, path: &Path) -> bool;
}

#[derive(Clone, Default)]
pub struct DefaultFsInt;

impl FsInt for DefaultFsInt {
    fn exists(&self, path: &Path) -> bool {
        std::fs::exists(path).unwrap()
    }

    fn is_directory(&self, path: &Path) -> bool {
        std::fs::metadata(path).unwrap().is_dir()
    }
}

impl MockFsInt {
    pub fn expect_and_rig_exists(&mut self, expected_path: PathBuf, rigged: bool) -> &mut __mock_MockFsInt_FsInt::__exists::Expectation {
        self.expect_exists()
            .with(mockall::predicate::eq(expected_path))
            .return_once(move |_| rigged)
    }

    pub fn expect_and_rig_is_directory(&mut self, expected_path: PathBuf, rigged: bool) -> &mut __mock_MockFsInt_FsInt::__is_directory::Expectation {
        self.expect_is_directory()
            .with(mockall::predicate::eq(expected_path))
            .return_once(move |_| rigged)
    }
}