
use super::VoxelsDirectoryError;

#[cfg(feature = "dbus")]
pub const DBUS_STANDARD_DIRECTORIES_SERVICE_INTERFACE: &str = "voxels.directories";

#[cfg(feature = "dbus")]
pub const DBUS_STANDARD_VOXELS_XDG_PATH: &str = "/base";

#[allow(dead_code)]
pub mod config;
#[allow(dead_code)]
pub mod data;
#[allow(dead_code)]
pub mod runtime;
#[allow(dead_code)]
pub mod state;
#[allow(dead_code)]
pub mod xdg;