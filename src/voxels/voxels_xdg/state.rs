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
use tracing::trace;
use crate::voxels::voxels_xdg::runtime::{RuntimeDirectoryPriority, RuntimeDirectoryResolutionMethods};
use crate::voxels::voxels_xdg::xdg::config::ConfigDirectoryResolutionMethods;

#[derive(Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum StateDirectoryResolutionMethods {
    FromXDG,
    #[cfg(feature = "dbus")]
    FromDBus,
}

pub struct StateDirectoryPriority {
    order: std::collections::BTreeMap<usize, StateDirectoryResolutionMethods>,
}

impl Default for StateDirectoryPriority {
    #[cfg(feature = "dbus")]
    fn default() -> Self {
        let mut order = std::collections::BTreeMap::new();
        order.insert(0, StateDirectoryResolutionMethods::FromDBus);
        order.insert(1, StateDirectoryResolutionMethods::FromXDG);
        Self {
            order
        }
    }

    #[cfg(not(feature = "dbus"))]
    fn default() -> Self {
        let mut order = std::collections::BTreeMap::new();
        order.insert(0, StateDirectoryResolutionMethods::FromXDG);
        Self {
            order
        }
    }
}

impl StateDirectoryPriority {

    #[cfg(feature = "dbus")]
    pub fn set_all(&mut self, new_order: [StateDirectoryResolutionMethods; 2]) {
        self.order = std::collections::BTreeMap::new();
        self.order.insert(0, new_order[0].clone());
        self.order.insert(1, new_order[1].clone());
    }

    #[cfg(not(feature = "dbus"))]
    pub fn set_all(&mut self, new_order: [StateDirectoryResolutionMethods; 1]) {
        self.order = std::collections::BTreeMap::new();
        self.order.insert(0, new_order[0].clone());
    }

    pub fn get(&self) -> std::collections::BTreeMap<usize, StateDirectoryResolutionMethods> {
        self.order.clone()
    }
}

#[mockall::automock]
pub trait StateDirectoryResolver {
    #[cfg(feature = "dbus")]
    async fn resolve_using_dbus(&self) -> Result<PathBuf, VoxelsDirectoryError>;

    fn resolve_using_xdg(&self) -> Result<PathBuf, VoxelsDirectoryError>;
    #[cfg(feature = "dbus")]
    async fn resolve(&self) -> Result<PathBuf, VoxelsDirectoryError>;

    #[cfg(not(feature = "dbus"))]
    fn resolve(&self) -> Result<PathBuf, VoxelsDirectoryError>;

    #[cfg(feature = "dbus")]
    async fn resolve_and_create(&self) -> Result<PathBuf, VoxelsDirectoryError>;

    #[cfg(not(feature = "dbus"))]
    fn resolve_and_create(&self) -> Result<PathBuf, VoxelsDirectoryError>;

    fn is_resolved(&self) -> bool;
}

pub struct StateDirectory<BaseT: base::StateDirectoryResolver> {
    path: Option<PathBuf>,
    pub priority: StateDirectoryPriority,
    base: BaseT,
}

impl<BaseT: base::StateDirectoryResolver> StateDirectory<BaseT> {
    pub fn new(base: BaseT) -> Self {
        Self {
            path: None,
            priority: Default::default(),
            base
        }
    }
}

impl<BaseT: base::StateDirectoryResolver> StateDirectoryResolver for StateDirectory<BaseT> {
    #[cfg(feature = "dbus")]
    async fn resolve_using_dbus(&self) -> Result<PathBuf, VoxelsDirectoryError> {
        trace!("Resolving state directory from DBus");

        todo!()
    }

    fn resolve_using_xdg(&self) -> Result<PathBuf, VoxelsDirectoryError> {
        trace!("Resolving state directory from DBus");

        // if resolve has been called previously we update this objects path
        if self.is_resolved() {
            return Ok(self.path.clone().unwrap());
        }

        let (base, _how) = self.base.resolve()?;

        Ok(base.join("voxels"))
    }

    #[cfg(feature = "dbus")]
    async fn resolve(&self) -> Result<PathBuf, VoxelsDirectoryError> {
        for index in 0..self.priority.order.len() {
            return match self.priority.order[&index] {
                StateDirectoryResolutionMethods::FromDBus => {
                    self.resolve_using_dbus().await
                },
                StateDirectoryResolutionMethods::FromXDG => {
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
                StateDirectoryResolutionMethods::FromXDG => {
                    self.resolve_using_xdg()
                }
            }
        }
        Err(VoxelsDirectoryError::NoCandidate)
    }

    #[cfg(feature = "dbus")]
    async fn resolve_and_create(&self) -> Result<PathBuf, VoxelsDirectoryError> {
        let resolved = self.resolve().await?;

        std::fs::create_dir_all(resolved.as_path()).expect("Failed to create directory");

        Ok(resolved)

    }

    #[cfg(not(feature = "dbus"))]
    fn resolve_and_create(&self) -> Result<PathBuf, VoxelsDirectoryError> {
        let resolved = self.resolve()?;

        std::fs::create_dir_all(resolved.as_path()).expect("Failed to create directory");

        Ok(resolved)
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
