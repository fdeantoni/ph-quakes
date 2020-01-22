use actix_web::{App, HttpServer, Responder, HttpResponse, web, Result};
use actix_web_static_files;
use askama::Template;

use std::collections::HashMap;

use quakes_api::*;
use quakes_scraper;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

async fn index() -> impl Responder {
    let body = Index.render().unwrap();
    HttpResponse::Ok().content_type("text/html").body(body)
}

async fn history() -> impl Responder {
    let quakes = quakes_scraper::get_philvolcs_quakes().await.unwrap();
    let geojson = QuakeList::new(quakes).to_geojson().await;
    web::Json(geojson)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    }
    env_logger::init();

    HttpServer::new(move || {
        let generated = generate();
        App::new()
            .service(actix_web_static_files::ResourceFiles::new(
            "/static", generated,
            ))
            .service(web::resource("/").route(web::get().to(index)))
            .route("/quakes.json", web::get().to(history))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
