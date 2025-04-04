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
use crate::base::{config as base};

use super::{VoxelsDirectoryError};

use std::path::{PathBuf};

#[mockall::automock]
pub trait ConfigDirectoryResolver {
    fn resolve(&self) -> Result<PathBuf, VoxelsDirectoryError>;

    fn is_resolved(&self) -> bool;
}

struct ConfigDirectory<BaseT: base::ConfigDirectoryResolver> {
    config_path: Option<PathBuf>,
    base: BaseT,
}

impl<BaseT: base::ConfigDirectoryResolver> ConfigDirectory<BaseT> {
    fn new(base: BaseT) -> Self {
        Self {
            config_path: None,
            base
        }
    }
}

impl<BaseT: base::ConfigDirectoryResolver> ConfigDirectoryResolver for ConfigDirectory<BaseT> {
    fn resolve(&self) -> Result<PathBuf, VoxelsDirectoryError> {
        // if resolve has been called previously we update this objects path
        if self.is_resolved() {
            return Ok(self.config_path.clone().unwrap());
        }

        let (base, _how) = self.base.resolve()?;

        Ok(base.join("voxels"))
    }

    fn is_resolved(&self) -> bool {
        self.config_path.is_some()
    }
}

impl<BaseT: base::ConfigDirectoryResolver> Into<Option<PathBuf>> for ConfigDirectory<BaseT> {
    fn into(self) -> Option<PathBuf> {
        self.config_path
    }
}