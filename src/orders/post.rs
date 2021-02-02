use actix_web::{web, HttpResponse};
use std::sync::Arc;

use crate::{amqp::emails::EmailPublisher, EmailChannles, HakuError};

pub async fn create_order(
  body: web::Bytes,
  channels: web::Data<Arc<EmailChannles>>,
) -> Result<HttpResponse, HakuError> {
  let payload = body.to_vec();
  EmailPublisher::send(&channels.publish_chan, payload).await?;

  Ok(HttpResponse::Ok().finish())
}
