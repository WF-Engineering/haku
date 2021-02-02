use lapin::{options::*, types::*, Channel, Connection, Queue};

use crate::{HakuError, HakuResult};

pub struct Declarer;

impl Declarer {
  pub async fn create_channel(conn: &Connection) -> HakuResult<Channel> {
    conn.create_channel().await.map_err(HakuError::from)
  }

  pub async fn declare_queue(
    channel: &Channel,
    name: &str,
  ) -> HakuResult<Queue> {
    channel
      .queue_declare(
        name,
        QueueDeclareOptions::default(),
        FieldTable::default(),
      )
      .await
      .map_err(HakuError::from)
  }
}
