pub mod core;
pub mod emails;

pub mod queue_names {
  pub const EMAIL: &str = "email_notification";
}

pub mod consumer_tags {
  pub const SEND_EMAIL: &str = "send_email";
}

pub mod routing_keys {
  pub const EMAIL: &str = "email_notification";
}
