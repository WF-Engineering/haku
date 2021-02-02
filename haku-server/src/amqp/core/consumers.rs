use lapin::{options::*, types::*, Channel};

use crate::{HakuError, HakuResult};

pub struct Consumer;

impl Consumer {
  pub async fn create(
    channel: &Channel,
    queue: &str,
    tag: &str,
  ) -> HakuResult<lapin::Consumer> {
    channel
      .basic_consume(
        queue,
        tag,
        BasicConsumeOptions::default(),
        FieldTable::default(),
      )
      .await
      .map_err(HakuError::from)
  }
}
