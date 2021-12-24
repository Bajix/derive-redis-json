# derive-redis-json

![License](https://img.shields.io/badge/license-MIT-green.svg)
![License](https://img.shields.io/badge/license-BSD-green.svg)
[![Cargo](https://img.shields.io/crates/v/derive-redis-json.svg)](https://crates.io/crates/derive-redis-json)
[![Documentation](https://docs.rs/derive-redis-json/badge.svg)](https://docs.rs/derive-redis-json)

A derive to store and retrieve JSON values in redis encoded using serde.
## Example

Cargo.toml:

```toml
[dependencies]
derive-redis-json = "0.1.1"
```

main.rs:

```rust
use std::sync::Arc;

use anyhow::Result;
use deadpool_redis::{redis::cmd, Pool as RedisPool};
use derive_redis_json::RedisJsonValue;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, RedisJsonValue)]
pub struct User {
  pub id: u64,
  pub name: String,
}

pub async fn add_user(
  redis_pool: Arc<RedisPool>,
  user: User,
) -> Result<usize> {
  let mut conn = redis_pool.get().await?;
  let res: usize = cmd("SADD")
    .arg("Users")
    .arg(&user)
    .query_async(&mut conn)
    .await?;

  Ok(res)
}

pub async fn get_users(
  redis_pool: Arc<RedisPool>,
) -> Result<Vec<User>> {
  let mut conn = redis_pool.get().await?;
  let res: Vec<User> = cmd("SMEMBERS").arg("Users").query_async(&mut conn).await?;

  Ok(res)
}
```
