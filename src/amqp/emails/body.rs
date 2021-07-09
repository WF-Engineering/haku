use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as Jsonb;

#[derive(Debug, Deserialize, Serialize)]
pub struct EmailNotification {
  pub brand_id: i64,
  pub template: Template,
  pub customer: Customer,
  pub sender: Sender,
  pub required_args: Jsonb,
  pub other_args: Jsonb,
  pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Template {
  pub id: i32,
  pub mode: String,
  pub subject: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Customer {
  pub email: String,
  pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Sender {
  pub id: i32,
  pub name: String,
  pub email: String,
}
