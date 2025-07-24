use std::path::PathBuf;
use crate::voxels::VoxelsDirectoryError;

use super::voxels_xdg::data as base;

use dbus::nonblock::{
    SyncConnection,
    Proxy
};

use std::sync::Arc;
use std::time::Duration;
use dbus_tokio::connection;
use dbus_tokio::connection::IOResourceError;

use tokio_util::sync::CancellationToken;

use lib_voxels_application::application::application::Application;

#[mockall::automock]
pub trait DataDirectoryResolver {
    async fn resolve(&self, application: Application) -> Result<PathBuf, VoxelsDirectoryError>;

    async fn resolve_and_create(&self, application: Application) -> Result<PathBuf, VoxelsDirectoryError>;

    fn is_resolved(&self) -> bool;
}

pub struct DataDirectory<BaseT: base::DataDirectoryResolver> {
    data_path: Option<PathBuf>,
    base: BaseT,
}

impl<BaseT: base::DataDirectoryResolver> DataDirectory<BaseT> {
    pub fn new(base: BaseT) -> Self {
        Self {
            data_path: None,
            base
        }
    }
}

impl<BaseT: base::DataDirectoryResolver> DataDirectoryResolver for DataDirectory<BaseT> {
    async fn resolve(&self, application: Application) -> Result<PathBuf, VoxelsDirectoryError> {
        // if resolve has been called previously we update this objects path
        if self.is_resolved() {
            return Ok(self.data_path.clone().unwrap());
        }

        let base = self.base.resolve()?;

        Ok(base.join("voxels"))
    }

    async fn resolve_and_create(&self, application: Application) -> Result<PathBuf, VoxelsDirectoryError> {
        let resolved = self.resolve(application).await?;

        std::fs::create_dir_all(resolved.as_path()).expect("Failed to create directory");

        Ok(resolved)
    }

    fn is_resolved(&self) -> bool {
        self.data_path.is_some()
    }
}