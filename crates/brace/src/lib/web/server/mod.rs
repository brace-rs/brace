use std::net::Ipv4Addr;

use actix_web::{web, App, HttpServer};

pub async fn start(host: Ipv4Addr, port: u16) -> anyhow::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(crate::web::routes::index::get)))
        .bind((host, port))?
        .run()
        .await?;

    Ok(())
}
