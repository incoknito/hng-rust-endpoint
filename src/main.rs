use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use chrono::prelude::*;
use std::env;
use serde::Serialize;
use serde::Deserialize;

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

#[get("/info")]
async fn get_info(query: web::Query<InfoParams>) -> impl Responder {
    // Extract parameters from the query
    let slack_name = &query.slack_name;
    let track = &query.track;
    let github_username = &query.github_username;
    let github_repo = &query.github_repo;
    let github_filename = &query.github_filename;

    // Get the current day and time
    let current_day = Utc::now().format("%A").to_string();
    let utc_time = Utc::now().to_rfc3339();

    // Construct the GitHub URLs
    let github_file_url = format!(
        "https://github.com/{}/{}/blob/main/{}",
        github_username, github_repo, github_filename
    );
    let github_repo_url = format!("https://github.com/{}/{}", github_username, github_repo);

    // Create the response data
    let response_data = InfoResponse {
        slack_name: slack_name.clone(),
        current_day,
        utc_time,
        track: track.clone(),
        github_file_url,
        github_repo_url,
        status_code: 200,
    };

    // Return the response as JSON
    HttpResponse::Ok().json(response_data)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    HttpServer::new(|| {
        App::new().service(get_info)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
