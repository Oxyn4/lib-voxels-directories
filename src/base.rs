
pub mod config;

use super::environment_variables::{MockEnvInt, EnvInt};
use super::filesystem::{MockFsInt, FsInt};

#[derive(Debug)]
enum BaseDirectoryError {
    NoCandidate
}

