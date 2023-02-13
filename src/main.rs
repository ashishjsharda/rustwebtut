use actix_web::{web, App, HttpResponse, HttpServer, Responder, get};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
    })
        .bind("127.0.0.1:8081")?
        .run().await
}

