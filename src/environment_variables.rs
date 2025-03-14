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

use std::{env::VarError, path::PathBuf};

use mockall::automock;

#[automock]
pub trait EnvInt {
    fn get_path_from_environment(&self, name: String) -> Result<PathBuf, VarError>;
}

#[derive(Clone, Default)]
pub struct DefaultEnvInt;

impl EnvInt for DefaultEnvInt {
    fn get_path_from_environment(&self, name: String) -> Result<PathBuf, VarError> {
        let rslt = std::env::var(name);

        if rslt.is_ok() {
            Ok(PathBuf::from(rslt?))
        } else {
            Err(rslt.unwrap_err().into())
        }
    }
}

impl MockEnvInt {
    pub fn expect_and_rig(&mut self, name: &str, env_var_value: PathBuf) -> &mut __mock_MockEnvInt_EnvInt::__get_path_from_environment::Expectation {
        self.expect_get_path_from_environment()
            .with(mockall::predicate::eq(name.to_owned()))
            .return_once(|_| Ok(env_var_value))
    }

    pub fn expect_and_rig_to_fail(&mut self, name: String) -> &mut __mock_MockEnvInt_EnvInt::__get_path_from_environment::Expectation {
        self.expect_get_path_from_environment()
            .with(mockall::predicate::eq(name.to_owned()))
            .return_once(|_| Err(VarError::NotPresent))
    }
}
