use actix_web::{App, HttpServer, Responder, HttpResponse, web};
use actix_web_static_files;
use askama::Template;

use std::collections::HashMap;

use quakes_api::*;
use quakes_scraper;
use std::sync::Mutex;
use log::info;

mod websocket;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

async fn index() -> impl Responder {
    let body = Index.render().unwrap();
    HttpResponse::Ok().content_type("text/html").body(body)
}

async fn history(features: web::Data<Mutex<GeoJson>>) -> impl Responder {
    let geojson = &*features.lock().unwrap();
    web::Json(geojson.clone())
}

async fn get_quakes() -> GeoJson {
    let quakes = quakes_scraper::get_philvolcs_quakes().await.unwrap();
    QuakeList::new(quakes).to_geojson().await
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info,quakes_server=debug");
    }
    env_logger::init();

    info!("Loading initial quake data from philvolcs...");
    let quakes = get_quakes().await;
    let data = web::Data::new(Mutex::new(quakes));

    HttpServer::new(move || {
        let generated = generate();
        App::new()
            .app_data(data.clone())
            .service(actix_web_static_files::ResourceFiles::new("/static", generated))
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/ws/").route(web::get().to(websocket::index)))
            .route("/quakes.json", web::get().to(history))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
