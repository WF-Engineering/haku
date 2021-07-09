#[macro_use]
extern crate log;

mod amqp;
mod config;
mod errors;
mod orders;
mod router;
mod state;

use actix_cors::Cors;
use actix_web::{http, middleware, App, HttpServer};
use chrono::Utc;
use lapin::{Connection, ConnectionProperties};
use std::{env, io, sync::Arc};
use tokio_amqp::*;

use amqp::emails::{EmailConsumer, EmailDeclarer, EmailDelegator};
use config::{Config, Env};
use errors::{HakuError, HakuResult};
use state::{AppState, EmailChannles};

#[actix_web::main]
async fn main() -> io::Result<()> {
  load_env();
  env_logger::init();

  let environment = match envy::from_env::<Env>() {
    Ok(env) => env,
    Err(err) => {
      panic!("Failed to load environment cause: {:?}", err);
    }
  };

  debug!("Running server in {}", &environment.to_address());

  let config = match envy::from_env::<Config>() {
    Ok(cfg) => cfg,
    Err(err) => panic!("Failed to load config cause: {:?}", err),
  };

  let connection = Connection::connect(
    &config.amqp_url,
    ConnectionProperties::default().with_tokio(),
  )
  .await
  .expect("Failed to connect AMQP");

  let email_channels = Arc::new(
    setup_email_pub_cus(&connection, &config)
      .await
      .expect("Failed to create email pub_cus environment"),
  );

  HttpServer::new(move || {
    App::new()
      .data(AppState {
        now: Utc::now,
        config: config.clone(),
      })
      .data(Arc::clone(&email_channels))
      .wrap(
        Cors::default()
          .allowed_methods(vec!["POST"])
          .allowed_headers(vec![http::header::CONTENT_TYPE, http::header::ACCEPT])
          .max_age(3600),
      )
      .wrap(middleware::Logger::default())
      .configure(router::config)
  })
  .bind(environment.to_address())?
  .run()
  .await
}

fn load_env() {
  dotenv::dotenv().ok();

  // 如果從根目錄找不到 AMQP_URL，說明 current_dir 不是 haku-server，
  // 就從 haku-server/.env 讀取環境變量
  if env::var("AMQP_URL").is_err() {
    let haku_env = env::current_dir()
      .map(|p| p.join("haku-server/.env"))
      .expect("Failed to get haku env path");

    dotenv::from_path(haku_env).unwrap();
  }
}

async fn create_email_channels(conn: &Connection) -> HakuResult<EmailChannles> {
  let (publish_chan, consume_chan) = EmailDeclarer::create_pubcus_channels(conn).await?;
  let queue = EmailDeclarer::declare_queue(&publish_chan).await?;
  debug!("email queue declare: {:?}", queue);

  Ok(EmailChannles {
    publish_chan,
    consume_chan,
  })
}

async fn setup_email_pub_cus(conn: &Connection, config: &Config) -> HakuResult<EmailChannles> {
  let email_channels = create_email_channels(conn)
    .await
    .expect("Failed to create email channels");

  let consumer = EmailConsumer::create(&email_channels.consume_chan)
    .await
    .expect("Failed to create email customer");

  EmailDelegator::set(&consumer, to_static(config.sendinblue_api_key.clone()))
    .expect("Failed to set email delegator");

  Ok(email_channels)
}

fn to_static(input: String) -> &'static str {
  Box::leak(input.into_boxed_str())
}
