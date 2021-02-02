use lapin::{Consumer, ConsumerDelegate};

use crate::{HakuError, HakuResult};

pub struct Delegator;

impl Delegator {
  pub fn set<D: ConsumerDelegate + 'static>(
    consumer: &Consumer,
    delegate: D,
  ) -> HakuResult<()> {
    consumer.set_delegate(delegate).map_err(HakuError::from)
  }
}
