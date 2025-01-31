use actix_web::{web, HttpResponse};

pub mod controller;

pub fn route_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/health")
            .route(web::get().to(|| async { HttpResponse::Ok().body("connected") })),
    );
}
