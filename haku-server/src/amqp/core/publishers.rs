use lapin::{
  options::*, publisher_confirm::Confirmation, BasicProperties, Channel,
};

use crate::{HakuError, HakuResult};

pub struct Publisher;

impl Publisher {
  pub async fn send(
    channel: &Channel,
    exchange: &str,
    routing_key: &str,
    payload: Vec<u8>,
  ) -> HakuResult<Confirmation> {
    channel
      .basic_publish(
        exchange,
        routing_key,
        BasicPublishOptions::default(),
        payload,
        BasicProperties::default(),
      )
      .await
      .map_err(|err| error!("publish confirm err: {:?}", err))
      .unwrap()
      .await
      .map_err(HakuError::from)
  }
}
