use serde::de::DeserializeOwned;
use thiserror::Error;

pub use ezconfig_derive::Config;
pub use serde;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Missing or invalid config field: {0}")]
    Envy(#[from] envy::Error),

    #[error("Failed to read .env file: {0}")]
    DotEnv(#[from] dotenvy::Error),
}

pub trait Config: Sized + DeserializeOwned {
    fn load() -> Result<Self, Error>;
}

#[doc(hidden)]
pub mod __internal {
    use super::*;

    pub fn load<T: DeserializeOwned>() -> Result<T, Error> {
        match dotenvy::dotenv() {
            Ok(_) => {}
            Err(dotenvy::Error::Io(ref e)) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => return Err(Error::DotEnv(e)),
        }

        envy::from_env::<T>().map_err(Error::Envy)
    }
}