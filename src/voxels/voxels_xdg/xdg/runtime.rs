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
use crate::voxels::voxels_xdg::xdg::BaseDirectoryError;
use crate::environment_variables::EnvInt;
use crate::filesystem::FsInt;

#[mockall::automock]
pub trait RuntimeVerifier {
    fn verify(&self, path: &Path) -> bool;
}

#[derive(Default)]
pub struct DefaultRuntimeVerifier<FsIntT: FsInt> {
    fs: FsIntT,
}


impl<FsIntT: FsInt> RuntimeVerifier for DefaultRuntimeVerifier<FsIntT> {
    fn verify(&self, path: &Path) -> bool {
        if !self.fs.exists(path) {
            return false;
        }

        if !self.fs.is_directory(path) {
            return false;
        }

        if !self.fs.is_absolute(path) {
            return false;
        }

        true
    }
}

impl<FsIntT: FsInt> DefaultRuntimeVerifier<FsIntT> {
    pub fn new(fs: FsIntT) -> Self {
        Self {
            fs
        }
    }
}


#[derive(Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum RuntimeDirectoryResolutionMethods {
    FromXDG,
    FromVoxels
}

pub struct RuntimeDirectoryPriority {
    order: std::collections::BTreeMap<usize, RuntimeDirectoryResolutionMethods>,
}

impl Default for RuntimeDirectoryPriority {
    fn default() -> Self {
        let mut order = std::collections::BTreeMap::new();
        order.insert(0, RuntimeDirectoryResolutionMethods::FromVoxels);
        Self {
            order
        }
    }
}

impl RuntimeDirectoryPriority {
    fn set_all(&mut self, new_order: [RuntimeDirectoryResolutionMethods; 3]) {
        self.order = std::collections::BTreeMap::new();
        self.order.insert(0, new_order[0].clone());
        self.order.insert(1, new_order[1].clone());
        self.order.insert(2, new_order[2].clone());
    }

    fn get(&self) -> std::collections::BTreeMap<usize, RuntimeDirectoryResolutionMethods> {
        self.order.clone()
    }
}

#[mockall::automock]
pub trait RuntimeDirectoryResolver {
    fn using_xdg(&self) -> Result<PathBuf, BaseDirectoryError>;
    fn using_voxels(&self) -> Result<PathBuf, BaseDirectoryError>;
    fn resolve(&self) -> Result<(PathBuf, RuntimeDirectoryResolutionMethods), BaseDirectoryError>;
}

#[derive(Default)]
pub struct RuntimeDirectory<EnvIntT: EnvInt, VerifierT: RuntimeVerifier> {
    data_path: Option<PathBuf>,
    verifier: VerifierT,
    env: EnvIntT,
    pub priority: RuntimeDirectoryPriority,
}

impl<EnvIntT: EnvInt, VerifierT: RuntimeVerifier> RuntimeDirectory<EnvIntT, VerifierT> {
    pub fn new(env: EnvIntT, verifier: VerifierT) -> Self {
        let priority = RuntimeDirectoryPriority::default();
        Self {
            data_path: None,
            env,
            verifier,
            priority
        }
    }
}

impl<EnvIntT: EnvInt, VerifierT: RuntimeVerifier> RuntimeDirectoryResolver for RuntimeDirectory<EnvIntT, VerifierT> {
    fn using_xdg(&self) -> Result<PathBuf, BaseDirectoryError> {
        let data_path: PathBuf = self.env.get_path_from_environment(String::from("XDG_RUNTIME_DIR"))?;

        if self.verifier.verify(&data_path) {
            Ok(data_path)
        } else {
            Err(BaseDirectoryError::NoCandidate)
        }
    }

    fn using_voxels(&self) -> Result<PathBuf, BaseDirectoryError> {
        let path: PathBuf = self.env.get_path_from_environment(String::from("VOXELS_RUNTIME_HOME"))?;

        if self.verifier.verify(&path) {
            Ok(path)
        } else {
            Err(BaseDirectoryError::NoCandidate)
        }
    }

    fn resolve(&self) -> Result<(PathBuf, RuntimeDirectoryResolutionMethods), BaseDirectoryError> {
        for index in 0..self.priority.order.len() {
            match self.priority.order[&index] {
                RuntimeDirectoryResolutionMethods::FromXDG => {
                    let path = self.using_xdg();

                    if path.is_ok() {
                        return Ok((path?, RuntimeDirectoryResolutionMethods::FromXDG));
                    }
                },
                RuntimeDirectoryResolutionMethods::FromVoxels => {
                    let path = self.using_voxels();

                    if path.is_ok() {
                        return Ok((path?, RuntimeDirectoryResolutionMethods::FromVoxels));
                    }
                }
            }
        }
        Err(BaseDirectoryError::NoCandidate)
    }
}

impl<EnvIntT: EnvInt, VerifierT: RuntimeVerifier> Into<PathBuf> for RuntimeDirectory<EnvIntT, VerifierT> {
    fn into(self) -> PathBuf {
        self.data_path.unwrap()
    }
}