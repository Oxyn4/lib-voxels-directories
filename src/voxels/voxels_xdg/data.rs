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
use crate::voxels::voxels_xdg::xdg::{data as base};

use super::{VoxelsDirectoryError};

use std::path::{PathBuf};
use tracing::trace;
use crate::voxels::voxels_xdg::runtime::{RuntimeDirectoryPriority, RuntimeDirectoryResolutionMethods};
use crate::voxels::voxels_xdg::xdg::config::ConfigDirectoryResolutionMethods;

#[derive(Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum DataDirectoryResolutionMethods {
    FromXDG,
    #[cfg(feature = "dbus")]
    FromDBus,
}

pub struct DataDirectoryPriority {
    order: std::collections::BTreeMap<usize, DataDirectoryResolutionMethods>,
}

impl Default for DataDirectoryPriority {
    #[cfg(not(feature = "dbus"))]
    fn default() -> Self {
        let mut order = std::collections::BTreeMap::new();

        order.insert(0, DataDirectoryResolutionMethods::FromXDG);

        Self {
            order
        }
    }

    #[cfg(feature = "dbus")]
    fn default() -> Self {
        let mut order = std::collections::BTreeMap::new();

        order.insert(0, DataDirectoryResolutionMethods::FromDBus);
        order.insert(1, DataDirectoryResolutionMethods::FromXDG);

        Self {
            order
        }
    }
}

impl DataDirectoryPriority {
    #[cfg(feature = "dbus")]
    pub fn set_all(&mut self, new_order: [DataDirectoryResolutionMethods; 2]) {
        self.order = std::collections::BTreeMap::new();
        self.order.insert(0, new_order[0].clone());
        self.order.insert(1, new_order[1].clone());
    }

    #[cfg(not(feature = "dbus"))]
    pub fn set_all(&mut self, new_order: [DataDirectoryResolutionMethods; 1]) {
        self.order = std::collections::BTreeMap::new();
        self.order.insert(0, new_order[0].clone());
    }

    pub fn get(&self) -> std::collections::BTreeMap<usize, DataDirectoryResolutionMethods> {
        self.order.clone()
    }
}

#[mockall::automock]
pub trait DataDirectoryResolver {
    #[cfg(feature = "dbus")]
    fn resolve_using_dbus(&self) -> Result<PathBuf, VoxelsDirectoryError>;

    fn resolve_using_xdg(&self) -> Result<PathBuf, VoxelsDirectoryError>;

    fn resolve(&self) -> Result<PathBuf, VoxelsDirectoryError>;
    fn resolve_and_create(&self) -> Result<PathBuf, VoxelsDirectoryError>;

    fn is_resolved(&self) -> bool;
}

pub struct DataDirectory<BaseT: base::DataDirectoryResolver> {
    path: Option<PathBuf>,
    pub priority: DataDirectoryPriority,
    base: BaseT,
}

impl<BaseT: base::DataDirectoryResolver> DataDirectory<BaseT> {
    pub fn new(base: BaseT) -> Self {
        let priority = DataDirectoryPriority::default();
        Self {
            path: None,
            priority,
            base
        }
    }
}

impl<BaseT: base::DataDirectoryResolver> DataDirectoryResolver for DataDirectory<BaseT> {
    #[cfg(feature = "dbus")]
    fn resolve_using_dbus(&self) -> Result<PathBuf, VoxelsDirectoryError> {
        trace!("Resolving data directory from DBus");

        todo!()
    }

    fn resolve_using_xdg(&self) -> Result<PathBuf, VoxelsDirectoryError> {
        trace!("Resolving data directory from DBus");

        // if resolve has been called previously we update this objects path
        if self.is_resolved() {
            return Ok(self.path.clone().unwrap());
        }

        let (base, _how) = self.base.resolve()?;

        Ok(base.join("voxels"))
    }

    #[cfg(feature = "dbus")]
    fn resolve(&self) -> Result<PathBuf, VoxelsDirectoryError> {
        for index in 0..self.priority.order.len() {
            return match self.priority.order[&index] {
                DataDirectoryResolutionMethods::FromDBus => {
                    self.resolve_using_dbus()
                },
                DataDirectoryResolutionMethods::FromXDG => {
                    self.resolve_using_xdg()
                }
            }
        }
        Err(VoxelsDirectoryError::NoCandidate)
    }

    #[cfg(not(feature = "dbus"))]
    fn resolve(&self) -> Result<PathBuf, VoxelsDirectoryError> {
        for index in 0..self.priority.order.len() {
            return match self.priority.order[&index] {
                DataDirectoryResolutionMethods::FromXDG => {
                    self.resolve_using_xdg()
                }
            }
        }
        Err(VoxelsDirectoryError::NoCandidate)
    }

    fn resolve_and_create(&self) -> Result<PathBuf, VoxelsDirectoryError> {
        let resolved = self.resolve()?;

        std::fs::create_dir_all(resolved.as_path()).expect("Failed to create directory");

        Ok(resolved)
    }

    fn is_resolved(&self) -> bool {
        self.path.is_some()
    }
}

impl<BaseT: base::DataDirectoryResolver> Into<Option<PathBuf>> for DataDirectory<BaseT> {
    fn into(self) -> Option<PathBuf> {
        self.path
    }
}