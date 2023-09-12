use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use chrono::prelude::*;
use std::env;
use serde::Serialize;
use serde::Deserialize;
// use futures_util::TryFutureExt;

extern crate actix_rt; 

#[get("/info")]
async fn get_info(query: web::Query<InfoParams>) -> impl Responder {
    let slack_name = &query.slack_name;
    let current_day = Utc::now().format("%A").to_string();
    let utc_time = Utc::now().to_rfc3339();
    let track = &query.track;
    let github_file_url = format!(
        "https://github.com/{}/{}/blob/main/{}",
        &query.github_username, &query.github_repo, &query.github_filename
    );
    let github_repo_url = format!(
        "https://github.com/{}/{}",
        &query.github_username, &query.github_repo
    );

    let response_data = InfoResponse {
        slack_name: slack_name.to_string(),
        current_day,
        utc_time,
        track: track.to_string(),
        github_file_url,
        github_repo_url,
        status_code: 200,
    };

    HttpResponse::Ok().json(response_data)
}

#[derive(Deserialize)]
struct InfoParams {
    slack_name: String,
    track: String,
    github_username: String,
    github_repo: String,
    github_filename: String,
}

#[derive(Serialize)]
struct InfoResponse {
    slack_name: String,
    current_day: String,
    utc_time: String,
    track: String,
    github_file_url: String,
    github_repo_url: String,
    status_code: i32,
}

#[actix_rt::main]
async fn main() -> Result<(), std::io::Error> {
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    HttpServer::new(|| {
        App::new().service(get_info)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await?;

    Ok(())
}
