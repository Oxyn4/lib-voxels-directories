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
use crate::environment_variables::EnvInt;
use crate::filesystem::FsInt;
use super::BaseDirectoryError;

#[mockall::automock]
pub trait StateVerifier {
    fn verify(&self, path: &Path) -> bool;
}

#[derive(Default)]
pub struct DefaultStateVerifier<FsIntT: FsInt> {
    fs: FsIntT,
}


impl<FsIntT: FsInt> StateVerifier for DefaultStateVerifier<FsIntT> {
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

impl<FsIntT: FsInt> DefaultStateVerifier<FsIntT> {
    pub fn new(fs: FsIntT) -> Self {
        Self {
            fs
        }
    }
}


#[derive(Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum StateDirectoryResolutionMethods {
    FromXDG,
    FromFHS,
    FromVoxels
}

struct StateDirectoryPriority {
    order: std::collections::BTreeMap<usize, StateDirectoryResolutionMethods>,
}

impl Default for StateDirectoryPriority {
    fn default() -> Self {
        let mut order = std::collections::BTreeMap::new();
        order.insert(0, StateDirectoryResolutionMethods::FromVoxels);
        order.insert(1, StateDirectoryResolutionMethods::FromXDG);
        order.insert(2, StateDirectoryResolutionMethods::FromFHS);
        Self {
            order
        }
    }
}

impl StateDirectoryPriority {
    fn set_all(&mut self, new_order: [StateDirectoryResolutionMethods; 3]) {
        self.order = std::collections::BTreeMap::new();
        self.order.insert(0, new_order[0].clone());
        self.order.insert(1, new_order[1].clone());
        self.order.insert(2, new_order[2].clone());
    }

    fn get(&self) -> std::collections::BTreeMap<usize, StateDirectoryResolutionMethods> {
        self.order.clone()
    }
}

#[mockall::automock]
pub trait StateDirectoryResolver {
    fn using_fhs(&self) -> Result<PathBuf, BaseDirectoryError>;
    fn using_xdg(&self) -> Result<PathBuf, BaseDirectoryError>;
    fn using_voxels(&self) -> Result<PathBuf, BaseDirectoryError>;
    fn resolve(&self) -> Result<(PathBuf, StateDirectoryResolutionMethods), BaseDirectoryError>;
}

#[derive(Default)]
pub struct StateDirectory<EnvIntT: EnvInt, VerifierT: StateVerifier> {
    state_path: Option<PathBuf>,
    verifier: VerifierT,
    env: EnvIntT,
    pub priority: StateDirectoryPriority,
}

impl<EnvIntT: EnvInt, VerifierT: StateVerifier> StateDirectory<EnvIntT, VerifierT> {
    pub fn new(env: EnvIntT, verifier: VerifierT) -> Self {
        let priority = StateDirectoryPriority::default();
        Self {
            state_path: None,
            env,
            verifier,
            priority
        }
    }
}

impl<EnvIntT: EnvInt, VerifierT: StateVerifier> StateDirectoryResolver for StateDirectory<EnvIntT, VerifierT> {
    fn using_fhs(&self) -> Result<PathBuf, BaseDirectoryError> {
        let path: PathBuf = self.env.get_path_from_environment(String::from("HOME"))?;

        let state_path = path.join(".local/state/");

        if self.verifier.verify(&state_path) {
            Ok(state_path)
        } else {
            Err(BaseDirectoryError::NoCandidate)
        }
    }

    fn using_xdg(&self) -> Result<PathBuf, BaseDirectoryError> {
        let state_path: PathBuf = self.env.get_path_from_environment(String::from("XDG_STATE_HOME"))?;

        if self.verifier.verify(&state_path) {
            Ok(state_path)
        } else {
            Err(BaseDirectoryError::NoCandidate)
        }
    }

    fn using_voxels(&self) -> Result<PathBuf, BaseDirectoryError> {
        let path: PathBuf = self.env.get_path_from_environment(String::from("VOXELS_STATE_HOME"))?;

        if self.verifier.verify(&path) {
            Ok(path)
        } else {
            Err(BaseDirectoryError::NoCandidate)
        }
    }

    fn resolve(&self) -> Result<(PathBuf, StateDirectoryResolutionMethods), BaseDirectoryError> {
        for index in 0..self.priority.order.len() {
            match self.priority.order[&index] {
                StateDirectoryResolutionMethods::FromXDG => {
                    let path = self.using_xdg();

                    if path.is_ok() {
                        return Ok((path?, StateDirectoryResolutionMethods::FromXDG));
                    }
                },
                StateDirectoryResolutionMethods::FromVoxels => {
                    let path = self.using_voxels();

                    if path.is_ok() {
                        return Ok((path?, StateDirectoryResolutionMethods::FromVoxels));
                    }
                },
                StateDirectoryResolutionMethods::FromFHS => {
                    let path = self.using_fhs();

                    if path.is_ok() {
                        return Ok((path?, StateDirectoryResolutionMethods::FromFHS));
                    }
                }
            }
        }
        Err(BaseDirectoryError::NoCandidate)
    }
}

impl<EnvIntT: EnvInt, VerifierT: StateVerifier> Into<PathBuf> for StateDirectory<EnvIntT, VerifierT> {
    fn into(self) -> PathBuf {
        self.state_path.unwrap()
    }
}