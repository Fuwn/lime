// Copyright (C) 2021-2021 Fuwn
// SPDX-License-Identifier: GPL-3.0-only

pub mod models;
mod schema;

use diesel::{insert_into, prelude::*, update};

use crate::db::models::{Link, LinkForm};

fn establish_connection() -> SqliteConnection {
  let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "lime.sqlite3".to_string());
  SqliteConnection::establish(&database_url)
    .unwrap_or_else(|_| panic!("error connecting to {}", database_url))
}

pub fn insert_link(form: LinkForm) -> Result<(), Box<dyn std::error::Error>> {
  use schema::links::dsl::*;

  if find_link(form.short, false).is_err() {
    insert_into(links)
      .values((long.eq(form.long), short.eq(form.short), ip.eq(form.ip)))
      .execute(&establish_connection())?;
  } else {
    bail!("short url already exists");
  }

  Ok(())
}

pub fn find_link(short_c: &str, tick: bool) -> Result<Link, Box<dyn std::error::Error>> {
  use schema::links::dsl::*;

  let results = links
    .filter(short.eq(short_c))
    .load::<Link>(&establish_connection())
    .unwrap();

  if results.is_empty() {
    bail!("no entry found with the short url: /{}", short_c)
  } else {
    let long_c = results[0].clone();

    if tick {
      update(links.find(&long_c.short))
        .set(uses.eq(long_c.uses + 1))
        .execute(&establish_connection())?;
    }

    if long_c.disabled {
      bail!("short url disabled")
    }

    Ok(long_c)
  }
}
