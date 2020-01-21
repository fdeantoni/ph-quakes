use actix_web::{App, HttpServer, Responder, HttpResponse, web, Result};
use actix_files::NamedFile;
use actix_web_static_files;
use askama::Template;

mod history;
mod error;

use std::collections::HashMap;
use std::path::PathBuf;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

async fn index() -> impl Responder {
    let body = Index.render().unwrap();
    HttpResponse::Ok().content_type("text/html").body(body)
}

async fn history() -> Result<NamedFile> {
    let path = PathBuf::from("quakes.json");
    Ok(NamedFile::open(path)?)
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
