mod args;
mod body;

use lapin::{
  message::DeliveryResult, options::BasicAckOptions, publisher_confirm::Confirmation, Channel,
  Connection, Queue,
};

use super::{
  consumer_tags,
  core::{Consumer, Declarer, Delegator, Publisher},
  queue_names, routing_keys,
};
use crate::HakuResult;

use sendinblue::{Mailer, Sendinblue, TransactionalBody};

pub struct EmailPublisher;

impl EmailPublisher {
  pub async fn send(channel: &Channel, payload: Vec<u8>) -> HakuResult<Confirmation> {
    Publisher::send(channel, "", routing_keys::EMAIL, payload).await
  }
}

pub struct EmailDeclarer;

impl EmailDeclarer {
  pub async fn create_pubcus_channels(conn: &Connection) -> HakuResult<(Channel, Channel)> {
    let pub_chan = Declarer::create_channel(conn).await?;
    let cus_chan = Declarer::create_channel(conn).await?;
    Ok((pub_chan, cus_chan))
  }

  pub async fn declare_queue(channel: &Channel) -> HakuResult<Queue> {
    Declarer::declare_queue(channel, queue_names::EMAIL).await
  }
}

pub struct EmailConsumer;

impl EmailConsumer {
  pub async fn create(channel: &Channel) -> HakuResult<lapin::Consumer> {
    Consumer::create(channel, queue_names::EMAIL, consumer_tags::SEND_EMAIL).await
  }
}

pub struct EmailDelegator;

impl EmailDelegator {
  pub fn set(consumer: &lapin::Consumer, sendinblue_api_key: &'static str) -> HakuResult<()> {
    Delegator::set(consumer, move |delivery: DeliveryResult| async move {
      let (channel, delivery) = match delivery.ok().flatten() {
        Some((channel, delivery)) => (channel, delivery),
        None => return,
      };

      let data = delivery.clone().data;
      let email_notification = match serde_json::from_slice::<body::EmailNotification>(&data) {
        Ok(email_notification) => email_notification,
        Err(err) => {
          error!("serde_json err: {:?}", err);
          return;
        }
      };
      debug!("email notification: {:?}", email_notification);

      let client = Sendinblue::production(sendinblue_api_key.to_string());

      let sender = Mailer::new(
        &email_notification.sender.name,
        &email_notification.sender.email,
      );

      let to_mailer = Mailer::new(
        &email_notification.customer.name,
        &email_notification.customer.email,
      );

      let payload = create_payload(sender, to_mailer, email_notification);
      let result = client.send_transactional_email(payload).await;
      debug!("sent email result: {:?}", result);

      channel
        .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
        .await
        .map_err(|err| {
          error!(
            "delivery_tag: {:?}, ack err: {:?}",
            delivery.delivery_tag, &err
          )
        })
        .unwrap()
    })
  }
}

fn create_payload(
  sender: Mailer,
  to_mailer: Mailer,
  email_notification: body::EmailNotification,
) -> TransactionalBody {
  let payload = TransactionalBody::builder()
    .set_sender(sender.clone())
    .add_to_mailer(to_mailer)
    .reply_to(sender)
    .template_id(email_notification.template.id as u32)
    .subject(email_notification.template.subject)
    .add_values(email_notification.required_args);

  match email_notification.template.mode {
    body::TemplateMode::CreateOrder => {
      let order_created_at =
        serde_json::from_value::<args::CreateOrder>(email_notification.other_args.clone())
          .unwrap()
          .formatted_dt();

      payload
        .add_values(email_notification.other_args)
        .add_params("order_created_at", order_created_at)
        .create()
    }
    body::TemplateMode::NewCreateOrder => {
      let order_created_at =
        serde_json::from_value::<args::OrderContent>(email_notification.other_args.clone())
          .unwrap()
          .formatted_dt();

      payload
        .add_values(email_notification.other_args)
        .add_params("order_created_at", order_created_at)
        .create()
    }

    body::TemplateMode::WarehouseDelivered => panic!("Unimplement"),
    body::TemplateMode::LogisticsStatusChanged
    | body::TemplateMode::PaymentReturned
    | body::TemplateMode::AccountRegister
    | body::TemplateMode::AccountForgotPassword
    | body::TemplateMode::EarlyDelivery
    | body::TemplateMode::ShippingInfo
    | body::TemplateMode::CvsArriveInfo
    | body::TemplateMode::CsatSurvey => payload.add_values(email_notification.other_args).create(),
  }
}
