use actix_web::{web, App, HttpServer};

mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/{org_uid}/{message}").route(web::post().to(handlers::process_request)))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
