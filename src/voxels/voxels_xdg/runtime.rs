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
use crate::voxels::voxels_xdg::xdg::{runtime as base};

use super::{VoxelsDirectoryError};

use std::path::{PathBuf};
use dbus_tokio::connection::IOResourceError;
use tracing::trace;

#[cfg(feature = "dbus")]
pub const DBUS_STANDARD_VOXELS_XDG_RUNTIME_METHOD_NAME: &str = "runtime";

#[derive(Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum RuntimeDirectoryResolutionMethods {
    FromXDG,
    #[cfg(feature = "dbus")]
    FromDBus,
}

pub struct RuntimeDirectoryPriority {
    pub(crate) order: std::collections::BTreeMap<usize, RuntimeDirectoryResolutionMethods>,
}

impl Default for RuntimeDirectoryPriority {
    #[cfg(feature = "dbus")]
    fn default() -> Self {
        let mut order = std::collections::BTreeMap::new();
        order.insert(0, RuntimeDirectoryResolutionMethods::FromDBus);
        order.insert(1, RuntimeDirectoryResolutionMethods::FromXDG);
        Self {
            order
        }
    }

    #[cfg(not(feature = "dbus"))]
    fn default() -> Self {
        let mut order = std::collections::BTreeMap::new();
        order.insert(0, RuntimeDirectoryResolutionMethods::FromXDG);
        Self {
            order
        }
    }
}

impl RuntimeDirectoryPriority {

    #[cfg(feature = "dbus")]
    pub fn set_all(&mut self, new_order: [RuntimeDirectoryResolutionMethods; 2]) {
        self.order = std::collections::BTreeMap::new();
        self.order.insert(0, new_order[0].clone());
        self.order.insert(1, new_order[1].clone());
    }

    #[cfg(not(feature = "dbus"))]
    pub fn set_all(&mut self, new_order: [RuntimeDirectoryResolutionMethods; 1]) {
        self.order = std::collections::BTreeMap::new();
        self.order.insert(0, new_order[0].clone());
    }

    pub fn get(&self) -> std::collections::BTreeMap<usize, RuntimeDirectoryResolutionMethods> {
        self.order.clone()
    }
}

#[mockall::automock]
pub trait RuntimeDirectoryResolver {
    #[cfg(feature = "dbus")]
    async fn resolve_using_dbus<F: FnOnce(IOResourceError) + Send + 'static>(&mut self, on_connection_loss: F) -> Result<PathBuf, VoxelsDirectoryError>;

    fn resolve_using_xdg(&mut self) -> Result<PathBuf, VoxelsDirectoryError>;

    #[cfg(not(feature = "dbus"))]
    fn resolve(&mut self) -> Result<PathBuf, VoxelsDirectoryError>;

    #[cfg(feature = "dbus")]
    async fn resolve(&mut self) -> Result<PathBuf, VoxelsDirectoryError>;

    #[cfg(feature = "dbus")]
    async fn resolve_and_create(&mut self) -> Result<PathBuf, VoxelsDirectoryError>;

    #[cfg(not(feature = "dbus"))]
    fn resolve_and_create(&mut self) -> Result<PathBuf, VoxelsDirectoryError>;

    fn is_resolved(&self) -> bool;
}

pub struct RuntimeDirectory<BaseT: base::RuntimeDirectoryResolver> {
    path: Option<PathBuf>,
    pub priority: RuntimeDirectoryPriority,
    base: BaseT,
}

impl<BaseT: base::RuntimeDirectoryResolver> RuntimeDirectory<BaseT> {
    pub fn new(base: BaseT) -> Self {
        let priority = RuntimeDirectoryPriority::default();
        Self {
            path: None,
            priority,
            base
        }
    }
}

impl<BaseT: base::RuntimeDirectoryResolver> RuntimeDirectoryResolver for RuntimeDirectory<BaseT> {
    #[cfg(feature = "dbus")]
    async fn resolve_using_dbus<F>(&mut self, on_connection_loss: F) -> Result<PathBuf, VoxelsDirectoryError>
    where
        F: FnOnce(IOResourceError) + Send + 'static
    {
        trace!("Resolving runtime directory from DBus");

        todo!()
    }

    fn resolve_using_xdg(&mut self) -> Result<PathBuf, VoxelsDirectoryError> {
        trace!("Resolving runtime directory from DBus");

        // if resolve has been called previously we update this objects path
        if self.is_resolved() {
            return Ok(self.path.clone().unwrap());
        }

        let (base, _how) = self.base.resolve()?;

        let config_path = base.join("voxels");

        self.path = Some(config_path.clone());

        Ok(config_path)
    }

    #[cfg(feature = "dbus")]
    async fn resolve(&mut self) -> Result<PathBuf, VoxelsDirectoryError> {
        for index in 0..self.priority.order.len() {
            return match self.priority.order[&index] {
                RuntimeDirectoryResolutionMethods::FromDBus => {
                    self.resolve_using_dbus(|_| {}).await
                },
                RuntimeDirectoryResolutionMethods::FromXDG => {
                    self.resolve_using_xdg()
                }
            }
        }
        Err(VoxelsDirectoryError::NoCandidate)
    }

    #[cfg(not(feature = "dbus"))]
    fn resolve(&mut self) -> Result<PathBuf, VoxelsDirectoryError> {
        for index in 0..self.priority.order.len() {
            return match self.priority.order[&index] {
                RuntimeDirectoryResolutionMethods::FromXDG => {
                    self.resolve_using_xdg()
                }
            }
        }
        Err(VoxelsDirectoryError::NoCandidate)
    }

    #[cfg(feature = "dbus")]
    async fn resolve_and_create(&mut self) -> Result<PathBuf, VoxelsDirectoryError> {
        let resolved = self.resolve().await?;

        std::fs::create_dir_all(resolved.as_path()).expect("Failed to create directory");

        Ok(resolved)

    }

    #[cfg(not(feature = "dbus"))]
    fn resolve_and_create(&mut self) -> Result<PathBuf, VoxelsDirectoryError> {
        let resolved = self.resolve()?;

        std::fs::create_dir_all(resolved.as_path()).expect("Failed to create directory");

        Ok(resolved)
    }

    fn is_resolved(&self) -> bool {
        self.path.is_some()
    }
}

impl<BaseT: base::RuntimeDirectoryResolver> Into<Option<PathBuf>> for RuntimeDirectory<BaseT> {
    fn into(self) -> Option<PathBuf> {
        self.path
    }
}