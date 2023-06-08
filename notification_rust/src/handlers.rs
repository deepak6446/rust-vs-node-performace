use actix_web::{web, HttpResponse, Responder};
use std::time::Duration;
use tokio::time::sleep;

pub async fn process_request(params: web::Path<(String, String)>) -> impl Responder {
    
    println!("org_uid={}, message={}", &params.0, &params.1);

    // Simulate a 2 ms delay
    sleep(Duration::from_millis(2)).await;

    HttpResponse::Ok().finish()
}
