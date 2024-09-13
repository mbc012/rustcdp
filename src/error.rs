use std::fmt::{Display, Formatter};
use derive_more::From;


pub type Result<T> = core::result::Result<T, Error>;


#[derive(Debug, From)]
pub enum Error {
    // Exceeded specified timeout
    TimeoutExceeded {
        current_timer_ms: u64,
        timeout_ms: u64
    },

    // Expected a value but did not find it.
    ValueNotFound,

    // Expected a value but didnt find it
    // todo: Is this still needed?
    ValueExpected(String),

    // Wrapper for serde json
    // todo: Impl better handling??
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
