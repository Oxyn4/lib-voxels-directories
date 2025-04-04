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
use crate::base::BaseDirectoryError;
use crate::environment_variables::EnvInt;
use crate::filesystem::FsInt;

#[mockall::automock]
trait DataVerifier {
    fn verify(&self, path: &Path) -> bool;
}

#[derive(Default)]
struct DefaultDataVerifier<FsIntT: FsInt> {
    fs: FsIntT,
}


impl<FsIntT: FsInt> DataVerifier for DefaultDataVerifier<FsIntT> {
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

impl<FsIntT: FsInt> DefaultDataVerifier<FsIntT> {
    fn new(fs: FsIntT) -> Self {
        Self {
            fs
        }
    }
}


#[derive(Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum DataDirectoryResolutionMethods {
    FromXDG,
    FromFHS,
    FromVoxels
}

struct DataDirectoryPriority {
    order: std::collections::BTreeMap<usize, DataDirectoryResolutionMethods>,
}

impl Default for DataDirectoryPriority {
    fn default() -> Self {
        let mut order = std::collections::BTreeMap::new();
        order.insert(0, DataDirectoryResolutionMethods::FromVoxels);
        order.insert(1, DataDirectoryResolutionMethods::FromXDG);
        order.insert(2, DataDirectoryResolutionMethods::FromFHS);
        Self {
            order
        }
    }
}

impl DataDirectoryPriority {
    fn set_all(&mut self, new_order: [DataDirectoryResolutionMethods; 3]) {
        self.order = std::collections::BTreeMap::new();
        self.order.insert(0, new_order[0].clone());
        self.order.insert(1, new_order[1].clone());
        self.order.insert(2, new_order[2].clone());
    }

    fn get(&self) -> std::collections::BTreeMap<usize, DataDirectoryResolutionMethods> {
        self.order.clone()
    }
}

#[mockall::automock]
pub trait DataDirectoryResolver {
    fn using_fhs(&self) -> Result<PathBuf, BaseDirectoryError>;
    fn using_xdg(&self) -> Result<PathBuf, BaseDirectoryError>;
    fn using_voxels(&self) -> Result<PathBuf, BaseDirectoryError>;
    fn resolve(&self) -> Result<(PathBuf, DataDirectoryResolutionMethods), BaseDirectoryError>;
}

#[derive(Default)]
pub struct DataDirectory<EnvIntT: EnvInt, VerifierT: DataVerifier> {
    data_path: Option<PathBuf>,
    verifier: VerifierT,
    env: EnvIntT,
    pub priority: DataDirectoryPriority,
}

impl<EnvIntT: EnvInt, VerifierT: DataVerifier> DataDirectory<EnvIntT, VerifierT> {
    pub fn new(env: EnvIntT, verifier: VerifierT) -> Self {
        let priority = DataDirectoryPriority::default();
        Self {
            data_path: None,
            env,
            verifier,
            priority
        }
    }
}

impl<EnvIntT: EnvInt, VerifierT: DataVerifier> DataDirectoryResolver for DataDirectory<EnvIntT, VerifierT> {
    fn using_fhs(&self) -> Result<PathBuf, BaseDirectoryError> {
        let path: PathBuf = self.env.get_path_from_environment(String::from("HOME")).unwrap();

        let data_path = path.join(".local/share/");

        if self.verifier.verify(&data_path) {
            Ok(data_path)
        } else {
            Err(BaseDirectoryError::NoCandidate)
        }
    }

    fn using_xdg(&self) -> Result<PathBuf, BaseDirectoryError> {
        let data_path: PathBuf = self.env.get_path_from_environment(String::from("XDG_DATA_HOME")).unwrap();

        if self.verifier.verify(&data_path) {
            Ok(data_path)
        } else {
            Err(BaseDirectoryError::NoCandidate)
        }
    }

    fn using_voxels(&self) -> Result<PathBuf, BaseDirectoryError> {
        let path: PathBuf = self.env.get_path_from_environment(String::from("VOXELS_DATA_HOME")).unwrap();

        if self.verifier.verify(&path) {
            Ok(path)
        } else {
            Err(BaseDirectoryError::NoCandidate)
        }
    }

    fn resolve(&self) -> Result<(PathBuf, DataDirectoryResolutionMethods), BaseDirectoryError> {
        for index in 0..self.priority.order.len() {
            return match self.priority.order[&index] {
                DataDirectoryResolutionMethods::FromXDG => {
                    let path = self.using_xdg();

                    if path.is_ok() {
                        Ok((path?, DataDirectoryResolutionMethods::FromXDG))
                    } else {
                        Err(BaseDirectoryError::NoCandidate)
                    }
                },
                DataDirectoryResolutionMethods::FromVoxels => {
                    let path = self.using_voxels();

                    if path.is_ok() {
                        Ok((path?, DataDirectoryResolutionMethods::FromVoxels))
                    } else {
                        Err(BaseDirectoryError::NoCandidate)
                    }
                },
                DataDirectoryResolutionMethods::FromFHS => {
                    let path = self.using_fhs();

                    if path.is_ok() {
                        Ok((path?, DataDirectoryResolutionMethods::FromFHS))
                    } else {
                        Err(BaseDirectoryError::NoCandidate)
                    }
                }
            }
        }
        unreachable!()
    }
}

impl<EnvIntT: EnvInt, VerifierT: DataVerifier> Into<PathBuf> for DataDirectory<EnvIntT, VerifierT> {
    fn into(self) -> PathBuf {
        self.data_path.unwrap()
    }
}