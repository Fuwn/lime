// Copyright (C) 2021-2021 Fuwn
// SPDX-License-Identifier: GPL-3.0-only

#[derive(Serialize, Deserialize, Debug)]
pub struct PostCreateShort {
  pub long:  String,
  pub short: String,
}

#[derive(askama::Template)]
#[template(path = "text.html")]
pub struct TextTemplate<'a> {
  pub text: &'a str,
}

#[derive(Serialize, Deserialize)]
pub struct StatisticsApi {
  pub long:     String,
  pub disabled: bool,
  pub uses:     i32,
}
