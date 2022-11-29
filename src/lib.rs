use std::fmt;

#[derive(Debug, Clone)]
pub(crate) struct SchemeError(pub(crate) String);

impl fmt::Display for SchemeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub(crate) fn validate_num_args<T>(
    args: &Vec<T>,
    min: usize,
    max: usize,
) -> Result<(), SchemeError> {
    match args.len() >= min {
        true => Ok(()),
        false => Err(SchemeError(format!(
            "Expected at least {} args, found {}",
            min,
            args.len(),
        ))),
    }?;
    if max > min {
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
