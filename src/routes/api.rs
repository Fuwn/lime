// Copyright (C) 2021-2021 Fuwn
// SPDX-License-Identifier: GPL-3.0-only

use actix_web::{web, HttpRequest, HttpResponse};
use askama::Template;

use crate::{
  db::{find_link, insert_link, models::LinkForm},
  structure::{PostCreateShort, StatisticsApi, TextTemplate},
};

#[post("/create")]
pub fn create(req: HttpRequest, params: web::Form<PostCreateShort>) -> HttpResponse {
  if let Err(e) = insert_link(LinkForm {
    long:  &params.long,
    short: &params.short,
    ip:    &req.peer_addr().unwrap().ip().to_string(),
  }) {
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
        text: &format!("short url created: /{}", params.short),
      }
      .render()
      .unwrap(),
    )
  }
}

#[get("/statistics")]
pub fn statistics(req: HttpRequest) -> HttpResponse {
  let queries = qstring::QString::from(req.query_string());

  let result = find_link(queries.get("short").unwrap_or(""), false);

  HttpResponse::Ok().json(if let Err(e) = result {
    StatisticsApi {
      long:     e.to_string(),
      disabled: true,
      uses:     0,
    }
  } else {
    let usable = result.unwrap();
    StatisticsApi {
      long:     usable.long,
      disabled: usable.disabled,
      uses:     usable.uses,
    }
  })
}

#[get("/")]
pub fn index() -> HttpResponse {
  HttpResponse::Ok().body(
    r#"# lime api
## routes
if a route requires a parameter, it will be notated like <this>.
for example; if a route is notated as "/api/v1/route?<parameter>", you can
access that route via the url
"http://this.domain/api/v1/route?parameter=something"

- /api/v1
  - /: index page (you are here)
  - /statistics?<short>: get information about a short url; long, disabled, uses
  - /create: a post route which takes a form; long and short, creates a new
    short url

### license
gnu general public license v3.0 (gpl-3.0-only)
https://github.com/fuwn/lime/blob/main/license"#,
  )
}
