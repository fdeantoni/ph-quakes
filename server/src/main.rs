use actix::*;
use actix::clock::*;
use actix_web::{App, HttpServer, Responder, HttpResponse, web};
use actix_web_static_files;
use askama::Template;

use std::time::Duration;
use std::collections::HashMap;

use quakes_api::*;
use quakes_scraper;
use log::info;
use dotenv::*;
use quakes_twitter::TwitterQuakes;
use crate::cache::UpdateCache;

mod websocket;
mod cache;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

async fn index() -> impl Responder {
    let body = Index.render().unwrap();
    HttpResponse::Ok().content_type("text/html").body(body)
}

async fn history() -> impl Responder {
    let quakes = get_quakes().await;
    let geojson = QuakeList::new(quakes);
    web::Json(geojson.to_geojson())
}

async fn get_quakes() -> Vec<Quake> {
    quakes_scraper::get_philvolcs_quakes().await.unwrap()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info,quakes_server=info");
    }
    env_logger::init();

    info!("Loading initial quake data from philvolcs...");
    let quakes = get_quakes().await;
    let cache = cache::CacheActor::create(|_| {
        cache::CacheActor::new(quakes)
    });

    let key = std::env::var("TWITTER_KEY").expect("Missing Twitter key");
    let secret = std::env::var("TWITTER_SECRET").expect("Missing Twitter secret");

    let data = web::Data::new(cache.clone());

    spawn(async move {
        let cache = cache.clone();
        info!("Will update quakes from twitter every 5 minutes...");
        let start = clock::Instant::now() + Duration::from_secs(60);
        let mut interval = interval_at(start, Duration::from_secs(300));
        let mut quakes = TwitterQuakes::new(key, secret);
        loop {
            interval.tick().await;
            if !quakes.has_started() {
                let updates = quakes.start().await.unwrap();
                cache.do_send(UpdateCache(updates))
            } else {
                let updates = quakes.next().await.unwrap();
                cache.do_send(UpdateCache(updates))
            }
        }
    });

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
