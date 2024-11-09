use serde::Serialize;
use std::convert::Infallible;
use thiserror::Error;
use validator::ValidationErrors;
use warp::{reply, Rejection, Reply};

#[derive(Error, Debug)]
pub enum Error {
    #[error("Not found")]
    NotFound,
    
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationErrors),
    
    #[error("Unauthorized")]
    Unauthorized,
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<String>,
}

impl Error {
    pub fn not_found() -> warp::Rejection {
        warp::reject::not_found()
    }

    pub fn db(err: sqlx::Error) -> warp::Rejection {
        warp::reject::not_found()
    }

    pub fn validation(err: ValidationErrors) -> warp::Rejection {
        warp::reject::not_found()
    }
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let (code, message, details) = if err.is_not_found() {
        (
            warp::http::StatusCode::NOT_FOUND,
            "Not Found".to_string(),
            None,
        )
    } else if let Some(e) = err.find::<Error>() {
        match e {
            Error::NotFound => (
                warp::http::StatusCode::NOT_FOUND,
                "Not Found".to_string(),
                None,
            ),
            Error::Database(_) => (
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
                None,
            ),
            Error::Validation(e) => (
                warp::http::StatusCode::BAD_REQUEST,
                "Validation Error".to_string(),
                Some(e.to_string()),
            ),
            Error::Unauthorized => (
                warp::http::StatusCode::UNAUTHORIZED,
                "Unauthorized".to_string(),
                None,
            ),
        }
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        (
            warp::http::StatusCode::METHOD_NOT_ALLOWED,
            "Method Not Allowed".to_string(),
            None,
        )
    } else {
        (
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error".to_string(),
            None,
        )
    };

    let json = reply::json(&ErrorResponse {
        message,
        details,
    });

    Ok(reply::with_status(json, code))
}