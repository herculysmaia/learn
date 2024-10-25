use actix_web::{web, App, HttpServer, Responder};

async fn index() -> impl Responder {
    "Hello, world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(web::scope("/app").route("/index.html", web::get().to(index)))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}