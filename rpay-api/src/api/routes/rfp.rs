use actix_web::HttpResponse;

pub async fn create() -> HttpResponse {
    HttpResponse::Created().finish()
}

