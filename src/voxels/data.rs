use std::path::PathBuf;
use crate::voxels::VoxelsDirectoryError;

use super::voxels_xdg::data as base;

#[mockall::automock]
pub trait DataDirectoryResolver {
    fn resolve(&self) -> Result<PathBuf, VoxelsDirectoryError>;

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
    fn resolve(&self) -> Result<PathBuf, VoxelsDirectoryError> {
        // if resolve has been called previously we update this objects path
        if self.is_resolved() {
            return Ok(self.data_path.clone().unwrap());
        }

        let base = self.base.resolve()?;

        Ok(base.join("voxels"))
    }

    fn is_resolved(&self) -> bool {
        self.data_path.is_some()
    }
}