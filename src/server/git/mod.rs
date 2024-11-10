use std::env::temp_dir;

use clap::builder::Str;
use diesel::{expression::is_aggregate::No, serialize};
use serde::{Deserialize, Serialize};
use url::Url;
use warp::{reject::Rejection, reply::Reply};
pub mod server;
#[derive(Serialize)]
struct ServerResponse {
    is_git_server: bool,
    details: Option<String>,
    error: Option<String>
}

#[derive(Deserialize)]
pub struct CheckRequest {
    pub url: Url
}

pub async fn check_git_server(request: CheckRequest) -> Result<impl Reply,Rejection> {
    println!("check received");
    let output = tokio::process::Command::new("git")
        .arg("ls-remote")
        .arg("--quiet")
        .arg("--exit-code")
        .arg(&request.url.to_string())
        .output().await;
    match output {
        Ok(output) => {
            let response = ServerResponse {
                is_git_server: output.status.success(),
                details: if output.status.success() {
                Some(format!("Connected to {}",request.url))
            } else {
                None
            },
                error: if !output.status.success() {
                    Some(String::from_utf8_lossy(&output.stderr).to_string())
                } else {
                    None
                },
            };
            return Ok(warp::reply::json(&response))
        }
        Err(e) => Ok(warp::reply::json(&ServerResponse {
            is_git_server: false,
            details: None,
            error: Some(format!("Failed to find server {}", e.to_string())),
        }))
    }
}