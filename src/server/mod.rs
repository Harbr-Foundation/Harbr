pub mod config;

use config::{Port, ServerConfig};
use warp::Filter;

// mod routes;
// pub mod error;
// pub mod db;

pub fn init() {
    let config: Option<ServerConfig> = None;
    let file = match std::fs::read_to_string("./harbr.config.json") {
        Ok(s) => {
            println!("read server config");
            println!("{s}");
            s
        }
        Err(e) => {
            println!("could not read server config");
            let default_config: ServerConfig = ServerConfig {
                fqdn: Some("".to_string()),
                main_port: Some(Port::default()),
                pool_size: Some(0),
            };
            let config_file = serde_json::to_string_pretty(&default_config);
            let _ = std::fs::write("./harbr.config.json", config_file.unwrap());
            run(Some(default_config));
            return;
        }
    };
    println!("Attempting to run with 'harbr.config.json'");
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
    println!("running main instance on 0.0.0.0:3030");

    // // Register the routes
    // let routes = routes::register()
    //     .recover(error::handle_rejection);

    warp::serve(json_route).run(([0, 0, 0, 0], 3030)).await;
}
