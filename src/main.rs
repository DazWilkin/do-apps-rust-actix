use std::collections::HashMap;
use std::env;
use std::net::SocketAddr;

use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

use actix_web_prom::PrometheusMetrics;

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("Hello Freddie")
}
#[get("/env")]
async fn envs() -> impl Responder {
    let e: Vec<String> = std::env::vars()
        .map(|(key, value)| format!("{}: {}", key, value))
        .collect();
    HttpResponse::Ok().body(e.join("\n"))
}
#[get("/headers")]
async fn headers(req: HttpRequest) -> impl Responder {
    let h: Vec<String> = req
        .headers()
        .iter()
        .map(|(name, value)| format!("{}: {:?}", name, value))
        .collect();
    HttpResponse::Ok().body(h.join("\n"))
}

#[get("/healthz")]
async fn healthz() -> impl Responder {
    HttpResponse::Ok().body("ok")
}

async fn manual() -> impl Responder {
    HttpResponse::Ok().body("ok")
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let key = "PORT";
    let port: u16 = match env::var(key) {
        Ok(val) => val.parse::<u16>().unwrap(),
        Err(_) => 8080,
    };

    let labels = HashMap::new();
    let prometheus = PrometheusMetrics::new("api", Some("/metrics"), Some(labels));

    HttpServer::new(move || {
        App::new()
            .service(root)
            .service(envs)
            .service(headers)
            .service(healthz)
            .route("/hey", web::get().to(manual))
            .wrap(prometheus.clone())
    })
    .bind(SocketAddr::from(([0, 0, 0, 0], port)))?
    .run()
    .await
}
