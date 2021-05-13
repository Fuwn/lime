// Copyright (C) 2021-2021 Fuwn
// SPDX-License-Identifier: GPL-3.0-only

use actix_web::{web, HttpResponse};
use askama::Template;

use crate::{db::find_link, structure::TextTemplate};

#[get("/")]
pub fn index() -> HttpResponse {
  HttpResponse::Ok().body(include_str!("../../templates/index.html"))
}

#[get("/{short}")]
pub async fn handle(info: web::Path<String>) -> HttpResponse {
  let result = find_link(&info.0, true);

  if let Err(ref e) = result {
    HttpResponse::Ok().body(
      TextTemplate {
        text: e.to_string().as_str(),
      }
      .render()
      .unwrap(),
    )
  } else {
    HttpResponse::Ok().body(format!(
      "<script>location.href=\"/{}\"</script>",
      result.unwrap().long,
    ))
  }
}

#[get("/{short}/statistics")]
pub async fn statistics(info: web::Path<String>) -> HttpResponse {
  let result = find_link(&info.0, false);

  if let Err(ref e) = result {
    HttpResponse::Ok().body(
      TextTemplate {
        text: e.to_string().as_str(),
      }
      .render()
      .unwrap(),
    )
  } else {
    HttpResponse::Ok().body(
      TextTemplate {
        text: &format!("/{} has {} uses", info.0, result.unwrap().uses),
      }
      .render()
      .unwrap(),
    )
  }
}
