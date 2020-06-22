use actix_web::web::{self, ServiceConfig};
use actix_web::Responder;

use crate::util::hook;

#[hook(crate::web::server::configure)]
fn configure(serv: &mut ServiceConfig) {
    serv.route("/", web::get().to(get));
}

async fn get() -> impl Responder {
    "brace"
}
