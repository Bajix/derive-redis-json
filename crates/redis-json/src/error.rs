use redis::{ErrorKind, RedisError};
use std::{
  num::{ParseFloatError, ParseIntError},
  str::Utf8Error,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
  #[error(transparent)]
  Utf8(#[from] Utf8Error),
  #[error(transparent)]
  ParseFloat(#[from] ParseFloatError),
  #[error(transparent)]
  ParseInt(#[from] ParseIntError),
  #[error(transparent)]
  Serde(#[from] serde_json::Error),
  #[error(transparent)]
  Redis(#[from] RedisError),
}

impl From<Error> for RedisError {
  fn from(err: Error) -> Self {
    match err {
      Error::Utf8(err) => RedisError::from((
        redis::ErrorKind::TypeError,
        "Invalid utf8 value",
        err.to_string(),
      )),
      Error::ParseFloat(err) => RedisError::from((
        redis::ErrorKind::TypeError,
        "Invalid float value",
        err.to_string(),
      )),
      Error::ParseInt(err) => RedisError::from((
        redis::ErrorKind::TypeError,
        "Invalid integer value",
        err.to_string(),
      )),
      Error::Serde(err) => {
        RedisError::from((ErrorKind::TypeError, "Invalid JSON value", err.to_string()))
      }
      Error::Redis(err) => err,
    }
  }
}
