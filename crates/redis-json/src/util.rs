use redis::RedisError;
use std::str::FromStr;

use crate::error::Error;

/// Parse value from redis::Value::Data
pub fn from_str<T>(value: &redis::Value) -> Result<T, Error>
where
  T: FromStr,
  Error: From<<T as FromStr>::Err>,
{
  match *value {
    redis::Value::Data(ref bytes) => std::str::from_utf8(&bytes[..])
      .map_err(|err| Error::Utf8(err))
      .and_then(|value| <T>::from_str(value).map_err(Error::from)),
    _ => Err(
      RedisError::from((
        redis::ErrorKind::TypeError,
        "invalid response type for JSON",
      ))
      .into(),
    ),
  }
}
