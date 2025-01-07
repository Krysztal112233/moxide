use std::io;

use thiserror::Error;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, Default, Error)]
pub enum Error {
    #[error("{0}")]
    Io(#[from] io::Error),

    #[error("{0}")]
    TomlDe(#[from] toml::de::Error),

    #[error("{0}")]
    TomlSer(#[from] toml::ser::Error),

    #[error("render not found: {0}")]
    RenderNotFound(String),

    #[error("invalid data block")]
    InvalidDataBlock,

    #[default]
    #[error("unknown error")]
    Unknown,
}
