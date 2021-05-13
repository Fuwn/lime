// Copyright (C) 2021-2021 Fuwn
// SPDX-License-Identifier: GPL-3.0-only

#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate simple_error;

pub mod routes;
pub mod structure;

pub mod db;

use actix_web::web;
use routes::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv::dotenv().ok();

  actix_web::HttpServer::new(|| {
    actix_web::App::new()
      .wrap(actix_cors::Cors::default().allow_any_origin())
      .service(ui::index)
      .service(ui::handle)
      .service(ui::statistics)
      .service(
        web::scope("/api/v1")
          .service(api::create)
          .service(api::statistics)
          .service(api::index),
      )
  })
  .bind(format!(
    "0.0.0.0:{}",
    std::env::var("PORT").expect("no port was provided.")
  ))?
  .run()
  .await
}
