use chrono::{DateTime, Utc};
use lapin::Channel;

use crate::config::Config;

#[derive(Clone)]
pub struct AppState {
  pub now: fn() -> DateTime<Utc>,
  pub config: Config,
}

#[derive(Clone)]
pub struct EmailChannles {
  pub publish_chan: Channel,
  pub consume_chan: Channel,
}
