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

use std::path::PathBuf;
use crate::voxels::VoxelsDirectoryError;

use super::voxels_xdg::data as base;

use lib_voxels_application::application::application::Application;

#[cfg(feature = "dbus")]
pub const DBUS_STANDARD_APPS_DATA_METHOD_NAME: &str = "data";

#[mockall::automock]
pub trait DataDirectoryResolver {
    async fn resolve(&self, application: Application) -> Result<PathBuf, VoxelsDirectoryError>;

    async fn resolve_and_create(&self, application: Application) -> Result<PathBuf, VoxelsDirectoryError>;

    fn is_resolved(&self) -> bool;
}

pub struct DataDirectory<BaseT: base::DataDirectoryResolver> {
    data_path: Option<PathBuf>,
    base: BaseT,
}

impl<BaseT: base::DataDirectoryResolver> DataDirectory<BaseT> {
    pub fn new(base: BaseT) -> Self {
        Self {
            data_path: None,
            base
        }
    }
}

impl<BaseT: base::DataDirectoryResolver> DataDirectoryResolver for DataDirectory<BaseT> {
    async fn resolve(&self, application: Application) -> Result<PathBuf, VoxelsDirectoryError> {
        // if resolve has been called previously we update this objects path
        if self.is_resolved() {
            return Ok(self.data_path.clone().unwrap());
        }

        let base = self.base.resolve().await?;

        Ok(base.join(application.rdn().as_path()))
    }

    async fn resolve_and_create(&self, application: Application) -> Result<PathBuf, VoxelsDirectoryError> {
        let resolved = self.resolve(application).await?;

        std::fs::create_dir_all(resolved.as_path()).expect("Failed to create directory");

        Ok(resolved)
    }

    fn is_resolved(&self) -> bool {
        self.data_path.is_some()
    }
}