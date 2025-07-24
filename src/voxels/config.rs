use std::path::PathBuf;

use lib_voxels_application::application::application::Application;

use super::VoxelsDirectoryError;

use super::voxels_xdg::config as base;

#[mockall::automock]
pub trait ConfigDirectoryResolver {
    async fn resolve(&self, application: Application) -> Result<PathBuf, VoxelsDirectoryError>;

    async fn resolve_and_create(&self, application: Application) -> Result<PathBuf, VoxelsDirectoryError>;

    fn is_resolved(&self) -> bool;
}

pub struct ConfigDirectory<BaseT: base::ConfigDirectoryResolver> {
    data_path: Option<PathBuf>,
    base: BaseT,
}

impl<BaseT: base::ConfigDirectoryResolver> ConfigDirectory<BaseT> {
    pub fn new(base: BaseT) -> Self {
        Self {
            data_path: None,
            base
        }
    }
}

impl<BaseT: base::ConfigDirectoryResolver> ConfigDirectoryResolver for ConfigDirectory<BaseT> {
    async fn resolve(&self, application: Application) -> Result<PathBuf, VoxelsDirectoryError> {
        // if resolve has been called previously we update this objects path
        if self.is_resolved() {
            return Ok(self.data_path.clone().unwrap());
        }

        let base = self.base.resolve()?;

        Ok(base.join(application.rdn().as_path()))
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