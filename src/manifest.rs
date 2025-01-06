use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Manifest {
    pub(crate) site: String,

    #[serde(default = "default_description")]
    pub(crate) description: String,

    #[serde(default = "default_theme")]
    pub(crate) theme: String,
}

fn default_description() -> String {
    "".to_owned()
}

fn default_theme() -> String {
    "theme".to_owned()
}

impl TryFrom<String> for Manifest {
    type Error = super::error::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(toml::from_str(&value)?)
    }
}
