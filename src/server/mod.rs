pub mod config;
use std::{
    fs::File,
    path::{self, Path},
};

use config::{Port, ServerConfig};
use warp::{filters::path::path, reply::Response, Filter};

// mod routes;
// pub mod error;
// pub mod db;

pub fn init() {
    let config: Option<ServerConfig> = None;
    let file = match std::fs::read_to_string("./harbor.config.json") {
        Ok(s) => {
            println!("read server config");
            println!("{s}");
            s
        }
        Err(e) => {
            let default_config: ServerConfig = ServerConfig {
                fqdn: Some("".to_string()),
                main_port: Some(Port::default()),
                pool_size: Some(0),
            };
            let config_file = serde_json::to_string_pretty(&default_config);
            let _ = std::fs::write("./harbor.config.json", config_file.unwrap());
            //let read_config = std::fs::read_to_string(Path::new("./harbor.config.json")).unwrap();
            return;
        }
    };
    run(config);
}

#[tokio::main]
pub async fn run(config: Option<ServerConfig>) {
    let server_config = config.unwrap_or_default();
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "OPTIONS"])
        .allow_headers(vec![
            "Content-Type",
            "Authorization",
            "Access-Control-Allow-Origin",
        ])
        .max_age(3600); // Cache CORS preflight requests for 1 hour
    let json_route = warp::path("repo.json")
        .and(warp::fs::file("repo.json"))
        // Add CORS headers
        .with(cors);
    println!("running main instance on localhost:3030");

    // // Register the routes
    // let routes = routes::register()
    //     .recover(error::handle_rejection);

    warp::serve(json_route).run(([127, 0, 0, 1], 3030)).await;
}
