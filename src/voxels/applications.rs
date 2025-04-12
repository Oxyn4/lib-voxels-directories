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
use crate::application::{Application, ApplicationRDN};
use crate::filesystem::FsInt;

#[mockall::automock]
pub trait ApplicationsDirectoryResolver {
    fn resolve(&self) -> Result<PathBuf, VoxelsDirectoryError>;

    fn is_resolved(&self) -> bool;
}

struct ApplicationsDirectory<BaseT: base::DataDirectoryResolver> {
    applications_path: Option<PathBuf>,
    base: BaseT,
}

impl<BaseT: base::DataDirectoryResolver> ApplicationsDirectory<BaseT> {
    fn new(base: BaseT) -> Self {
        Self {
            applications_path: None,
            base
        }
    }
}

impl<BaseT: base::DataDirectoryResolver> ApplicationsDirectoryResolver for ApplicationsDirectory<BaseT> {
    fn resolve(&self) -> Result<PathBuf, VoxelsDirectoryError> {
        // if resolve has been called previously we update this objects path
        if self.is_resolved() {
            return Ok(self.applications_path.clone().unwrap());
        }

        let (base, _how) = self.base.resolve()?;

        Ok(base.join(String::from("voxels/applications/")))
    }

    fn is_resolved(&self) -> bool {
        self.applications_path.is_some()
    }
}

impl<BaseT: base::DataDirectoryResolver> Into<Option<PathBuf>> for ApplicationsDirectory<BaseT> {
    fn into(self) -> Option<PathBuf> {
        self.applications_path
    }
}

#[mockall::automock]
pub trait ApplicationDirectoryResolver {
    fn resolve(&self, application: &ApplicationRDN) -> Result<Application, VoxelsDirectoryError>;

    fn is_resolved(&self) -> bool;
}


struct ApplicationDirectory<BaseT: ApplicationsDirectoryResolver, FsIntT: FsInt> {
    path: Option<PathBuf>,
    app: Option<Application>,
    base: BaseT,
    fs: FsIntT,
}

impl<AppsDirResT: ApplicationsDirectoryResolver, FsIntT: FsInt> ApplicationDirectory<AppsDirResT, FsIntT> {
    fn new(base: AppsDirResT, fs: FsIntT) -> Self {
        Self {
            path: None,
            app: None,
            base,
            fs
        }
    }
}

impl<AppsDirResT: ApplicationsDirectoryResolver, FsIntT: FsInt> ApplicationDirectoryResolver for ApplicationDirectory<AppsDirResT, FsIntT> {
    fn resolve(&self, application: &ApplicationRDN) -> Result<Application, VoxelsDirectoryError> {
        // if resolve has been called previously we update this objects path
        if self.is_resolved() {
            return Ok(self.app.clone().unwrap());
        }

        let base = self.base.resolve()?;

        Ok(Application::from_file(&self.fs, base.join(String::from("voxels/applications/") + application.name() + "manifest.toml")))
    }

    fn is_resolved(&self) -> bool {
        self.path.is_some()
    }
}

impl<BaseT: ApplicationsDirectoryResolver, FsIntT: FsInt> Into<Option<PathBuf>> for ApplicationDirectory<BaseT, FsIntT> {
    fn into(self) -> Option<PathBuf> {
        self.path
    }
}

impl<BaseT: ApplicationsDirectoryResolver, FsIntT: FsInt> Into<Option<Application>> for ApplicationDirectory<BaseT, FsIntT> {
    fn into(self) -> Option<Application> {
        self.app
    }
}
