use actix_web::Responder;

pub async fn get() -> impl Responder {
    "brace"
}
