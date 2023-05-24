use actix_web::{HttpResponse, get};

#[get("/testping")]
async fn ping() -> HttpResponse {
    HttpResponse::Ok().body("ping success!".to_string())
}