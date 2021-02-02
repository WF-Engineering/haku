use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Env {
  pub host: String,
  pub port: u32,
}

impl Env {
  pub fn to_address(&self) -> String {
    format!("{}:{}", self.host, self.port)
  }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
  pub amqp_url: String,
  pub sendinblue_api_key: String,
}
