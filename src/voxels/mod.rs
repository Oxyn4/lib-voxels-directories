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

use voxels_xdg::xdg::BaseDirectoryError;

#[cfg(feature = "dbus")]
pub const DBUS_STANDARD_APPS_PATH: &str = "/apps";

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum VoxelsDirectoryError {
    NoCandidate
}

impl From<BaseDirectoryError> for VoxelsDirectoryError {
    fn from(err: BaseDirectoryError) -> Self {
        match err {
            BaseDirectoryError::NoCandidate => VoxelsDirectoryError::NoCandidate
        }
    }
}
pub mod voxels_xdg;

#[allow(dead_code)]
#[cfg(feature = "application")]
pub mod data;

#[allow(dead_code)]
#[cfg(feature = "application")]
pub mod config;

#[allow(dead_code)]
#[cfg(feature = "application")]
pub mod state;

#[allow(dead_code)]
#[cfg(feature = "application")]
pub mod runtime;
