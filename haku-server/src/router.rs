use actix_web::web;

use crate::orders;

pub fn config(config: &mut web::ServiceConfig) {
  config.service(web::resource("/orders/create").route(web::post().to(orders::post::create_order)));
}
