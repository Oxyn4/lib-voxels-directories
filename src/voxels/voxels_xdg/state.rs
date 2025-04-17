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
use crate::voxels::voxels_xdg::xdg::{state as base};

use super::{VoxelsDirectoryError};

use std::path::{PathBuf};

#[derive(Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum StateDirectoryResolutionMethods {
    FromXDG,
    FromDBus,
}

#[mockall::automock]
pub trait StateDirectoryResolver {
    fn resolve(&self) -> Result<PathBuf, VoxelsDirectoryError>;

    fn is_resolved(&self) -> bool;
}

pub struct StateDirectory<BaseT: base::StateDirectoryResolver> {
    path: Option<PathBuf>,
    base: BaseT,
}

impl<BaseT: base::StateDirectoryResolver> StateDirectory<BaseT> {
    pub fn new(base: BaseT) -> Self {
        Self {
            path: None,
            base
        }
    }
}

impl<BaseT: base::StateDirectoryResolver> StateDirectoryResolver for StateDirectory<BaseT> {
    fn resolve(&self) -> Result<PathBuf, VoxelsDirectoryError> {
        // if resolve has been called previously we update this objects path
        if self.is_resolved() {
            return Ok(self.path.clone().unwrap());
        }

        let (base, _how) = self.base.resolve()?;

        Ok(base.join("voxels"))
    }

    fn is_resolved(&self) -> bool {
        self.path.is_some()
    }
}

impl<BaseT: base::StateDirectoryResolver> Into<Option<PathBuf>> for StateDirectory<BaseT> {
    fn into(self) -> Option<PathBuf> {
        self.path
    }
}
