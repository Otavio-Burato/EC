use actix_web::{middleware, get, web, App, HttpServer, Responder};
use serde::Deserialize;
use serde::Serialize;
use actix_files::Files;

#[derive(Serialize, Deserialize)]
struct Response {
    echo: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(||
        App::new()
        .wrap(middleware::Logger::default())
        .service(json)
        .service(Files::new("/", "./static/").index_file("index.html")))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

#[get("/json")]
async fn json(req: web::Json<Response>) -> impl Responder {
    web::Json(Response {
        echo: req.echo.clone(),
    })
}
