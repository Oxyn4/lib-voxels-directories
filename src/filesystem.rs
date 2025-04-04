/*
Copyright (C) 2025  Jacob Evans

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use std::path::{Path, PathBuf};

use mockall::automock;

#[automock]
pub trait FsInt {
    fn exists(&self, path: &Path) -> bool;
    fn is_directory(&self, path: &Path) -> bool;
    fn is_absolute(&self, path: &Path) -> bool;
    fn read_to_string(&self, path: &Path) -> std::io::Result<String>;
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

    fn is_absolute(&self, path: &Path) -> bool {
        path.is_absolute()
    }

    fn read_to_string(&self, path: &Path) -> std::io::Result<String> {
        std::fs::read_to_string(path)
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