use redis::{streams::StreamId, RedisError};
use serde::{de::Deserialize, ser::Serialize};
use std::collections::BTreeMap;

extern crate self as redis_json;

pub mod error;
pub mod util;

use error::Error;
/// Extension trait to facilitate Redis Stream serialization/deserialization
pub trait StreamEntry: Send + Sync + Serialize + for<'de> Deserialize<'de> {
  /// Deserializes from a stringified key value map but only if every field is FromStr. See [`serde_with::PickFirst<(_, serde_with::DisplayFromStr)>`](https://docs.rs/serde_with/1.11.0/serde_with/guide/serde_as_transformations/index.html#pick-first-successful-deserialization)
  fn from_stream_id(stream_id: &StreamId) -> Result<Self, Error> {
    let data = stream_id
      .map
      .iter()
      .map(|(key, value)| match value {
        redis::Value::Data(ref bytes) => std::str::from_utf8(&bytes[..])
          .map(|value| serde_json::Value::String(String::from(value)))
          .map_err(|err| err.into())
          .map(|value| (key.to_owned(), value)),
        _ => Err(
          RedisError::from((
            redis::ErrorKind::TypeError,
            "invalid response type for JSON",
          ))
          .into(),
        ),
      })
      .collect::<Result<Vec<(String, serde_json::Value)>, Error>>()?;

    let data = serde_json::map::Map::from_iter(data.into_iter());

    let data = serde_json::from_value(serde_json::Value::Object(data))?;

    Ok(data)
  }

  /// Serialize into a stringified field value mapping for XADD
  fn into_xadd_map(&self) -> Result<BTreeMap<String, String>, serde_json::Error> {
    let value = serde_json::to_value(&self)?;

    let data: Vec<(String, String)> = value
      .as_object()
      .into_iter()
      .flat_map(|map| {
        map.into_iter().filter_map(|(key, value)| match value {
          serde_json::Value::Null => None,
          serde_json::Value::Bool(value) => Some(Ok((key.to_owned(), value.to_string()))),
          serde_json::Value::Number(value) => Some(Ok((key.to_owned(), value.to_string()))),
          serde_json::Value::String(value) => Some(Ok((key.to_owned(), value.to_owned()))),

          serde_json::Value::Array(value) => {
            Some(serde_json::to_string(value).map(|value| (key.to_owned(), value)))
          }

          serde_json::Value::Object(value) => {
            Some(serde_json::to_string(value).map(|value| (key.to_owned(), value)))
          }
        })
      })
      .collect::<Result<Vec<(String, String)>, serde_json::Error>>()?;

    let data: BTreeMap<String, String> = BTreeMap::from_iter(data.into_iter());

    Ok(data)
  }
}
