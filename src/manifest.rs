use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Manifest {
    pub(crate) site: String,

    #[serde(default = "default_description")]
    pub(crate) description: String,

    #[serde(default = "default_theme")]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub(crate) theme: String,

    #[serde(default = "Vec::new")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) renders: Vec<String>,
}

fn default_description() -> String {
    "".to_owned()
}

fn default_theme() -> String {
    "".to_owned()
}

impl TryFrom<String> for Manifest {
    type Error = super::error::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(toml::from_str(&value)?)
    }
}
