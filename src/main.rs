use actix_web::{
    get,
    middleware::{DefaultHeaders, Logger},
    App, HttpServer,
};
use web::route_config;

mod web;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(DefaultHeaders::new().add(("X-Version", "0.0.1")))
            .service(index)
            .configure(route_config)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

#[get("/")]
async fn index() -> &'static str {
    "Hello, world!"
}
