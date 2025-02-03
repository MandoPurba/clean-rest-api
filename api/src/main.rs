use actix_web::{
    middleware::{DefaultHeaders, Logger},
    App, HttpServer,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(DefaultHeaders::new().add(("X-Version", "0.0.1")))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
