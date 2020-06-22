use std::net::Ipv4Addr;

use actix_web::web::ServiceConfig;
use actix_web::{App, HttpServer};

use crate::util::hook;

#[hook]
pub fn configure(serv: &mut ServiceConfig) {}

pub async fn start(host: Ipv4Addr, port: u16) -> anyhow::Result<()> {
    HttpServer::new(|| App::new().configure(|cfg| hook::exec(configure::with(cfg))))
        .bind((host, port))?
        .run()
        .await?;

    Ok(())
}
