// Copyright (c) 2018-2020 Joseph R. Quinn <quinn.josephr@protonmail.com>
// SPDX-License-Identifier: ISC
#[macro_use]
extern crate lazy_static;

use actix_web::client::Client;

lazy_static! {
    pub(crate) static ref USER_AGENT: String = {
        format!("Calus-Bot v{} ({})", env!("CARGO_PKG_VERSION"),
            env!("CARGO_PKG_REPOSITORY"))
    };

    pub(crate) static ref
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("{}", *USER_AGENT);

    Ok(())
}
