//! A [fluentd](https://www.fluentd.org/) client using tokio.
//!
//! ## Example
//!
//! ```
//! use std::collections::HashMap;
//!
//! use tokio_fluent::record_map;
//! use tokio_fluent::{Client, Config, FluentClient};
//! use tokio_fluent::record::{Map, Value};
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = Client::new_tcp(
//!         "127.0.0.1:24224".parse().unwrap(),
//!         &Config{..Default::default()},
//!     )
//!     .await
//!     .unwrap();
//!
//!     // With Map::new()
//!     let mut map = Map::new();
//!     map.insert("age".to_string(), 10.into());
//!     client.send("fluent.test", map).unwrap();
//!
//!     // With record_map! macro
//!     let mut map_from_macro = record_map!(
//!       "age".to_string() => 22.into(),
//!       "scores".to_string() => [80, 90].into_iter().map(|e| e.into()).collect::<Vec<_>>().into(),
//!     );
//! }
//! ```

pub mod client;
pub mod record;
mod worker;

pub use client::{Client, Config, FluentClient};
