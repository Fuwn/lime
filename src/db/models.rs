// Copyright (C) 2021-2021 Fuwn
// SPDX-License-Identifier: GPL-3.0-only

#[derive(Queryable, Debug, Clone)]
pub struct Link {
  pub long:     String,
  pub short:    String,
  pub disabled: bool,
  pub ip:       String,
  pub uses:     i32,
}

#[derive(Deserialize, Debug)]
pub struct LinkForm<'a> {
  pub long:  &'a str,
  pub short: &'a str,
  pub ip:    &'a str,
}
