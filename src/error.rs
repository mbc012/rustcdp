use std::fmt::{Display, Formatter};

use derive_more::From;
use serde::Serialize;
use serde_json::Value;
// todo: Need to restructure this
// - req a better process w/ 'universal' design


pub type Result<T> = core::result::Result<T, Error>;


#[derive(Debug, From)]
pub enum Error {
    // Avoid using
    General(String),

    TimeoutExceeded {
        timeout: u64
    },

    ValueNotFound,

    // Expected a value but didnt find it
    ValueExpected(String),

    // Wrapper for serde json todo: Impl better handling
    #[from]
    SerdeJson(serde_json::Error),
}

impl Error {
    pub fn as_err<T>(self) -> Result<T> {
        Err(self)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}


impl std::error::Error for Error {}
