use log::*;
use dotenv::*;
use actix::*;
use actix::clock::*;
use actix_web::{App, HttpServer, Responder, HttpResponse, web};
use actix_web_static_files;
use askama::Template;

use std::time::Duration;
use std::collections::HashMap;

use quakes_api::*;
use quakes_scraper;
use quakes_twitter::TwitterQuakes;
use crate::cache::UpdateCache;

mod websocket;
mod cache;
mod redirect;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

async fn index() -> impl Responder {
    let body = Index.render().unwrap();
    HttpResponse::Ok().content_type("text/html").body(body)
}

async fn quakes_json(cache: web::Data<Addr<cache::CacheActor>>) -> impl Responder {
    match cache.send(cache::GetQuakes).await {
        Ok(response) => web::Json(response.0.to_geojson()),
        Err(error) => {
            error!("Failed to retrieve latest quakes: {:?}", error);
            let quakes = QuakeList::new(Vec::new());
            web::Json(quakes.to_geojson())
        }
    }
}

async fn get_quakes() -> Vec<Quake> {
    quakes_scraper::get_phivolcs_quakes().await.unwrap()
}

fn dummy_quakes() -> Vec<Quake> {
    vec![
        Quake::new(Utc::now(), 125.71, 9.15, 4.0, 1, "TEST".to_string(), "TEST".to_string(), "https://example.com".to_string(), "test_tweet".to_string()),
    ]
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info,quakes_server=info");
    }
    env_logger::init();

    let is_test = std::env::var("TEST").is_ok();
    if is_test {
        info!("Using dummy quakes for updates!");
    } else {
        info!("Loading initial quake data from phivolcs...");
    }

    let quakes = if !is_test { get_quakes().await } else { dummy_quakes() };
    info!("Collected {} quakes.", &quakes.len());
    let cache = cache::CacheActor::create(|_| {
        cache::CacheActor::new(quakes)
    });

    let data = web::Data::new(cache.clone());

    let key = std::env::var("TWITTER_KEY").expect("Missing Twitter key");
    let secret = std::env::var("TWITTER_SECRET").expect("Missing Twitter secret");

    spawn(async move {
        let cache = cache.clone();
        let start = clock::Instant::now() + Duration::from_secs(60);
        let i = if !is_test { 300 } else { 10 };
        let mut interval = interval_at(start, Duration::from_secs(i));
        info!("Will update quakes from twitter every {} seconds...", i);
        let mut quakes = TwitterQuakes::new(key, secret);
        loop {
            interval.tick().await;
            let updates = if !is_test {
                quakes.get_tweets().await.unwrap_or_else(|error| {
                    error!("An error occurred retrieving quake tweets: {}", error.to_string());
                    Vec::new()
                })
            } else {
                dummy_quakes()
            };
            info!("Collected {} new quake tweets from twitter: {:#?}", &updates.len(), &updates);
            if !updates.is_empty() {
                cache.do_send(UpdateCache(updates))
            }
        }
    });


    let host = std::env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port = std::env::var("PORT").unwrap_or("8080".to_string());
    let https_enabled = std::env::var("HTTPS_ONLY").is_ok();
    info!("Will redirect http to https...");

    HttpServer::new(move || {
        let generated = generate();
        App::new()
            .wrap(redirect::RedirectHTTPS::default().enable(https_enabled))
            .app_data(data.clone())
            .service(actix_web_static_files::ResourceFiles::new("/static", generated))
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/ws/").route(web::get().to(websocket::index)))
            .route("/quakes.json", web::get().to(quakes_json))
    })
        .bind(format!("{}:{}", host, port))?
        .run()
        .await
}
