use std::path::PathBuf;

use serde::{Serialize, Deserialize};

#[derive(Debug, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Hash, Serialize, Deserialize)]
pub enum ApplicationErrors {
    InvalidName,
}

#[derive(Debug, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Hash, Serialize, Deserialize)]
pub enum ApplicationsType {
    Client,
    Server,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationRDN {
    name: String,
}

impl ApplicationRDN {
    pub fn new(name: String) -> Result<ApplicationRDN, ApplicationErrors> {
        if name.len() > 255 || name.is_empty() {
            return Err(ApplicationErrors::InvalidName);
        }

        if name.chars().filter(|&c| c == '.').count() == 0 {
            return Err(ApplicationErrors::InvalidName);
        }

        for element in name.split('.') {
            if element.is_empty() {
                return Err(ApplicationErrors::InvalidName);
            }

            if element.chars().nth(0).unwrap().is_numeric() {
                return Err(ApplicationErrors::InvalidName);
            }
        }

        if name.chars().filter(|&c| !c.is_alphanumeric() && c != '_' && c != '.').count().gt(&0) {
            return Err(ApplicationErrors::InvalidName);
        }

        Ok(ApplicationRDN {
            name
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Application {
    #[serde(skip)]
    rdn: ApplicationRDN,
    homepage: Option<url::Url>,
    description: Option<String>,
    app_type: Option<ApplicationsType>
}

impl Application {
    pub fn new(rdn: ApplicationRDN, homepage: Option<url::Url>, description: Option<String>, app_type: Option<ApplicationsType>) -> Application {
        Application {
            rdn,
            homepage,
            description,
            app_type
        }
    }

    pub fn from_file(path: PathBuf) -> Application {
        let mut app: Application = toml::from_str(&std::fs::read_to_string(path.clone()).unwrap()).unwrap();

        app.rdn = ApplicationRDN::new(path.parent().unwrap().file_name().unwrap().to_str().unwrap().to_string()).unwrap();

        app
    }
}
