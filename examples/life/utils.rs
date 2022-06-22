use std::io::BufRead;
use std::str::FromStr;

use crate::Result;

pub fn read_and_parse<T, R>(buf_read: &mut T) -> Result<R>
where
    T: BufRead,
    R: FromStr,
    <R as FromStr>::Err: std::error::Error + 'static,
{
    let mut buffer = String::new();
    buf_read.read_line(&mut buffer)?;

    let result = buffer.trim().parse()?;
    Ok(result)
}
