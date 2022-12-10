use std::{error::Error, fmt};

#[derive(Debug)]
pub(crate) struct SchemeError {
    pub(crate) message: String,
    source: Option<&'static dyn Error>,
}

impl fmt::Display for SchemeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for SchemeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
    }
}

impl SchemeError {
    pub(crate) fn new(message: String) -> Self {
        Self {
            message,
            source: None,
        }
    }

    pub(crate) fn from(message: String, e: &dyn Error) -> Self {
        Self {
            message,
            source: Some(e),
        }
    }
}
