use std::sync::Mutex;
use rocket::http::Status;
use crate::cache::Cache;
use rocket::{get, routes, State};
use rocket::fairing::AdHoc;
use rocket_cors::{AllowedOrigins, CorsOptions};
use crate::job::run;
mod model;
mod job;
mod cache;

#[get("/news")]
async fn news_handler(cache: &State<Mutex<Cache>>) -> Result<String, Status> {
    let mut store = cache.lock().unwrap();
    match store.get_news() {
        None => Err(Status::NotFound),
        Some(res) => Ok(res)
    }
}

#[get("/search?<query>")]
async fn search_news(query: String, cache: &State<Mutex<Cache>>) -> Result<String, Status> {
    let mut store = cache.lock().unwrap();
    match store.search_news(&query) {
        Ok(Some(filtered)) => serde_json::to_string(&filtered)
            .map_err(|_| Status::InternalServerError),
        Ok(None) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tokio::spawn(async {
        run().await;
    });

    let cache = Mutex::new(Cache::new());

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .to_cors()
        .map_err(|e| {
            eprintln!("CORS setup failed: {}", e);
            std::process::exit(1);
        })?;

    rocket::build()
        .manage(cache)
        .mount("/", routes![news_handler, search_news])
        .attach(cors)
        .attach(AdHoc::config::<rocket::Config>())
        .configure(rocket::Config {
            port: 4000,
            ..rocket::Config::default()
        })
        .launch()
        .await?;

    Ok(())
}