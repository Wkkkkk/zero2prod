use actix_web::HttpResponse;

pub async fn health_check() -> HttpResponse {
    println!("tic");

    HttpResponse::Ok().finish()
}
