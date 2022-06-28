use std::error::Error;
use std::fmt::{Display, Formatter};

pub type FetchResult<T> = Result<T, FetchError>;

#[derive(Debug)]
pub struct FetchError;

impl Display for FetchError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unable to fetch the item")
    }
}

impl Error for FetchError {}
