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

use super::BaseDirectoryError;
use std::path::{Path, PathBuf};
use crate::voxels::voxels_xdg::xdg::config::ConfigDirectoryResolutionMethods::{FromFHS, FromVoxels, FromXDG};
use super::{FsInt};
use super::{EnvInt};

#[mockall::automock]
pub trait ConfigVerifier {
    fn verify(&self, path: &Path) -> bool;
}

#[derive(Default)]
pub struct DefaultConfigVerifier<FsIntT: FsInt> {
    fs: FsIntT,
}

impl<FsIntT: FsInt> ConfigVerifier for DefaultConfigVerifier<FsIntT> {
    fn verify(&self, path: &Path) -> bool {
        if !self.fs.exists(path) {
            return false;
        }

        if !self.fs.is_directory(path) {
            return false;
        }

        true
    }
}

impl<FsIntT: FsInt> DefaultConfigVerifier<FsIntT> {
    pub fn new(fs: FsIntT) -> Self {
        Self {
            fs
        }
    }
}

#[test]
fn test_default_config_verifier() {
    let mut fs = crate::filesystem::MockFsInt::new();

    let test_path = Path::new("Home/");

    fs.expect_exists()
        .once()
        .with(mockall::predicate::eq(test_path))
        .return_once(|_| true);

    fs.expect_is_directory()
        .once()
        .with(mockall::predicate::eq(test_path))
        .return_once(|_| true);


    let validator = DefaultConfigVerifier::new(fs);

    let result = validator.verify(test_path);

    assert!(result);
}

#[derive(Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum ConfigDirectoryResolutionMethods {
    FromXDG,
    FromFHS,
    FromVoxels
}

pub struct ConfigDirectoryPriority {
    order: std::collections::BTreeMap<usize, ConfigDirectoryResolutionMethods>,
}

impl Default for ConfigDirectoryPriority {
    fn default() -> Self {
        let mut order = std::collections::BTreeMap::new();
        order.insert(0, FromVoxels);
        order.insert(1, FromXDG);
        order.insert(2, FromFHS);
        Self {
            order
        }
    }
}

impl ConfigDirectoryPriority {
    pub fn set_all(&mut self, new_order: [ConfigDirectoryResolutionMethods; 3]) {
        self.order = std::collections::BTreeMap::new();
        self.order.insert(0, new_order[0].clone());
        self.order.insert(1, new_order[1].clone());
        self.order.insert(2, new_order[2].clone());
    }

    pub fn get(&self) -> std::collections::BTreeMap<usize, ConfigDirectoryResolutionMethods> {
        self.order.clone()
    }
}


#[mockall::automock]
pub trait ConfigDirectoryResolver {
    fn using_fhs(&self) -> Result<PathBuf, BaseDirectoryError>;
    fn using_xdg(&self) -> Result<PathBuf, BaseDirectoryError>;
    fn using_voxels(&self) -> Result<PathBuf, BaseDirectoryError>;
    fn resolve(&self) -> Result<(PathBuf, ConfigDirectoryResolutionMethods), BaseDirectoryError>;

}

#[derive(Default)]
pub struct ConfigDirectory<EnvIntT: EnvInt, VerifierT: ConfigVerifier> {
    config_path: Option<PathBuf>,
    verifier: VerifierT,
    env: EnvIntT,
    pub priority: ConfigDirectoryPriority,
}

impl<EnvIntT: EnvInt, VerifierT: ConfigVerifier> ConfigDirectory<EnvIntT, VerifierT> {
    pub fn new(env: EnvIntT, verifier: VerifierT) -> Self {
        let priority = ConfigDirectoryPriority::default();
        Self {
            config_path: None,
            env,
            verifier,
            priority
        }
    }
}

impl<EnvIntT: EnvInt, VerifierT: ConfigVerifier> ConfigDirectoryResolver for ConfigDirectory<EnvIntT, VerifierT> {
    fn using_fhs(&self) -> Result<PathBuf, BaseDirectoryError> {
        let path: PathBuf = self.env.get_path_from_environment(String::from("HOME"))?;

        let config_path = path.join(".config/");

        if self.verifier.verify(&config_path) {
            Ok(config_path)
        } else {
            Err(BaseDirectoryError::NoCandidate)
        }
    }

    fn using_xdg(&self) -> Result<PathBuf, BaseDirectoryError> {
        let config_path: PathBuf = self.env.get_path_from_environment(String::from("XDG_CONFIG_HOME"))?;

        if self.verifier.verify(&config_path) {
            Ok(config_path)
        } else {
            Err(BaseDirectoryError::NoCandidate)
        }
    }

    fn using_voxels(&self) -> Result<PathBuf, BaseDirectoryError> {
        let path: PathBuf = self.env.get_path_from_environment(String::from("VOXELS_CONFIG_HOME"))?;

        if self.verifier.verify(&path) {
            Ok(path)
        } else {
            Err(BaseDirectoryError::NoCandidate)
        }
    }

    fn resolve(&self) -> Result<(PathBuf, ConfigDirectoryResolutionMethods), BaseDirectoryError> {
        for index in 0..self.priority.order.len() {
            match self.priority.order[&index] {
                FromXDG => {
                    let path = self.using_xdg();

                    if path.is_ok() {
                        return Ok((path?, FromXDG));
                    }
                },
                FromVoxels => {
                    let path = self.using_voxels();

                    if path.is_ok() {
                        return Ok((path?, FromVoxels));
                    }
                },
                FromFHS => {
                    let path = self.using_fhs();

                    if path.is_ok() {
                        return Ok((path?, FromFHS));
                    }
                }
            }
        }
        Err(BaseDirectoryError::NoCandidate)
    }
}

impl<EnvIntT: EnvInt, VerifierT: ConfigVerifier> Into<PathBuf> for ConfigDirectory<EnvIntT, VerifierT> {
    fn into(self) -> PathBuf {
        self.config_path.unwrap()
    }
}


#[test]
fn test_from_fhs() {
    let mut env = crate::environment_variables::MockEnvInt::new();
    let mut validator = MockConfigVerifier::new();

    let home_env = PathBuf::from("/home");

    let expected_home_path = PathBuf::from("/home/.config/");


    env.expect_get_path_from_environment()
        .once()
        .with(mockall::predicate::eq(String::from("HOME")))
        .return_once({
            let expected_home = home_env.clone();
            |_| Ok(expected_home)
        });

    validator.expect_verify()
        .once()
        .with(mockall::predicate::eq(expected_home_path.clone()))
        .return_once(|_| true);

    let config = ConfigDirectory::new(env, validator);

    let res = config.using_fhs();

    assert!(res.is_ok());
    assert_eq!(res.unwrap(), expected_home_path);
}

#[test]
fn test_resolve() {
    // create mocked interfaces to filesystem and environment variables
    let mut env = crate::environment_variables::MockEnvInt::new();
    let mut validator = MockConfigVerifier::new();

    // first test setup conditions for voxels environment variable
    // value of VOXELS_CONFIG_HOME environment variable
    let voxels_config_home = PathBuf::from("/home");

    let expected_voxels_return = PathBuf::from("/home");

    env.expect_and_rig("VOXELS_CONFIG_HOME", voxels_config_home.clone());

    validator.expect_verify()
        .with(mockall::predicate::eq(expected_voxels_return.clone()))
        .returning(|_| true);

    let config = ConfigDirectory::new(env, validator);

    let result = config.resolve();

    assert!(result.is_ok());

    assert_eq!(result.unwrap().0, expected_voxels_return);
}

#[test]
fn test_from_xdg() {
    let mut env= crate::environment_variables::MockEnvInt::new();
    let mut validator = MockConfigVerifier::new();

    let xdg_home = PathBuf::from("/home");

    let expected_home_path = PathBuf::from("/home");

    env.expect_and_rig("XDG_CONFIG_HOME", xdg_home.clone());

    validator.expect_verify()
        .with(mockall::predicate::eq(expected_home_path.clone()))
        .once()
        .return_once(|_| true);

    let config = ConfigDirectory::new(env, validator);

    let res = config.using_xdg();

    assert!(res.is_ok());
    assert_eq!(res.unwrap(), expected_home_path);
}

#[test]
fn test_from_voxels() {
    let mut env = crate::environment_variables::MockEnvInt::new();
    let mut validator: MockConfigVerifier = MockConfigVerifier::new();

    let voxels_env_home = PathBuf::from("/voxels");

    let expected_home_path = PathBuf::from("/voxels");

    env.expect_and_rig("VOXELS_CONFIG_HOME", voxels_env_home.clone());

    validator.expect_verify().once().returning(|_| true);

    let config = ConfigDirectory::new(env, validator);

    let res = config.using_voxels();

    assert!(res.is_ok());
    assert_eq!(res.unwrap(), expected_home_path);

}
