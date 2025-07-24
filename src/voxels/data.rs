use std::path::PathBuf;
use crate::voxels::VoxelsDirectoryError;

use super::voxels_xdg::data as base;

use dbus::nonblock::{
    SyncConnection,
    Proxy
};

use std::sync::Arc;

#[mockall::automock]
pub trait DataDirectoryResolver {
    async fn resolve(&self) -> Result<PathBuf, VoxelsDirectoryError>;

    async fn resolve_with_connection(&self, con: Arc<SyncConnection>) -> Result<PathBuf, VoxelsDirectoryError>;

    async fn resolve_with_proxy<'a>(&self, proxy: Proxy<'a, Arc<SyncConnection>>) -> Result<PathBuf, VoxelsDirectoryError>;

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
    async fn resolve(&self) -> Result<PathBuf, VoxelsDirectoryError> {
        // if resolve has been called previously we update this objects path
        if self.is_resolved() {
            return Ok(self.data_path.clone().unwrap());
        }

        let base = self.base.resolve()?;

        Ok(base.join("voxels"))
    }

    async fn resolve_with_connection(&self, con: Arc<SyncConnection>) -> Result<PathBuf, VoxelsDirectoryError> {
        todo!()
    }

    async fn resolve_with_proxy<'a>(&self, proxy: Proxy<'a, Arc<SyncConnection>>) -> Result<PathBuf, VoxelsDirectoryError> {
        todo!()
    }

    fn is_resolved(&self) -> bool {
        self.data_path.is_some()
    }
}