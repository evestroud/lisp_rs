use std::{error::Error, fmt};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct SchemeError(pub(crate) String);

impl fmt::Display for SchemeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for SchemeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

pub(crate) fn validate_num_args<T>(args: &[T], min: usize, max: usize) -> Result<(), SchemeError> {
    match args.len() >= min {
        true => Ok(()),
        false => Err(SchemeError(format!(
            "Expected at least {} args, found {}",
            min,
            args.len(),
        ))),
    }?;
    if max < usize::MAX {
        match args.len() <= max {
            true => Ok(()),
            false => Err(SchemeError(format!(
                "Procedure takes a maximum of {} args, found {}",
                max,
                args.len(),
            ))),
        }?;
    }
    Ok(())
}
