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
  let required =
    serde_json::from_value::<args::RequiredData>(email_notification.required_args).unwrap();

  let payload = TransactionalBody::builder()
    .set_sender(sender.clone())
    .add_to_mailer(to_mailer)
    .reply_to(sender)
    .template_id(email_notification.template.id as u32)
    .subject(email_notification.template.subject)
    .add_params("brand_name", required.brand_name.clone())
    .add_params("banner", required.banner.clone())
    .add_params("homepage_link", required.homepage_link.clone())
    .add_params("facebook_link", required.facebook_link.clone())
    .add_params("instagram_link", required.instagram_link);

  match email_notification.template.mode {
    body::TemplateMode::CreateOrder => {
      let create_order =
        serde_json::from_value::<args::CreateOrder>(email_notification.other_args).unwrap();

      payload
        .add_params("order_number", create_order.order_number.clone())
        .add_params("order_created_at", create_order.formatted_dt())
        .add_params("customer_name", create_order.customer_name)
        .create()
    }
    body::TemplateMode::WarehouseDelivered => panic!("Unimplement"),
    body::TemplateMode::LogisticsStatusChanged => panic!("Unimplement"),
    body::TemplateMode::PaymentReturned => panic!("Unimplement"),
    body::TemplateMode::AccountRegister => {
      let signup = serde_json::from_value::<args::Signup>(email_notification.other_args).unwrap();

      payload
        .add_params("customer_name", signup.customer_name)
        .add_params("link", signup.link)
        .create()
    }
    body::TemplateMode::AccountForgotPassword => {
      let forgot_password =
        serde_json::from_value::<args::ForgotPassword>(email_notification.other_args).unwrap();

      payload
        .add_params("customer_name", forgot_password.customer_name)
        .add_params("link", forgot_password.link)
        .create()
    }
    body::TemplateMode::EarlyDelivery => {
      let early_delivery =
        serde_json::from_value::<args::EarlyDelivery>(email_notification.other_args).unwrap();

      payload
        .add_params("customer_name", early_delivery.customer_name)
        .add_params("order_number", early_delivery.order_number)
        .create()
    }
    body::TemplateMode::ShippingInfo => {
      let shipping_info =
        serde_json::from_value::<args::ShippingInfo>(email_notification.other_args).unwrap();

      payload
        .add_params("customer_name", shipping_info.customer_name)
        .add_params("order_number", shipping_info.order_number)
        .add_params("logistics_type_name", shipping_info.logistics_type_name)
        .add_params("package_numbers", shipping_info.package_numbers)
        .create()
    }
    body::TemplateMode::CvsArriveInfo => {
      let cvs_arrive_info =
        serde_json::from_value::<args::CvsArriveInfo>(email_notification.other_args).unwrap();

      payload
        .add_params("customer_name", cvs_arrive_info.customer_name)
        .add_params("order_number", cvs_arrive_info.order_number)
        .add_params("branch_name", cvs_arrive_info.branch_name)
        .add_params("addressee_name", cvs_arrive_info.addressee_name)
        .add_params("package_number", cvs_arrive_info.package_number)
        .create()
    }
    body::TemplateMode::CsatSurvey => {
      let csat_survey_info =
        serde_json::from_value::<args::CsatSurvey>(email_notification.other_args).unwrap();

      payload
        .add_params("customer_name", csat_survey_info.customer_name)
        .add_params("order_number", csat_survey_info.order_number)
        .add_params("link", csat_survey_info.link)
        .create()
    }
    body::TemplateMode::NewCreateOrder => {
      let content =
        serde_json::from_value::<args::OrderContent>(email_notification.other_args).unwrap();

      payload
        .add_params("customer_name", content.customer_name.clone())
        .add_params("order_number", content.order_number.clone())
        .add_params_array("products", content.products.clone())
        .add_params("products_subtotal", content.products_subtotal.to_string())
        .add_params("promotion_discount", content.promotion_discount.to_string())
        .add_params("delivery_fee", content.delivery_fee.to_string())
        .add_params("total", content.total.to_string())
        .add_params("order_number", content.order_number.clone())
        .add_params("order_created_at", content.formatted_dt())
        .add_params("customer_name", content.customer_name)
        .create()
    }
  }
}
