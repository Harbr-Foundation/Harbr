pub mod config;

use std::{
    collections::HashMap, path::PathBuf, ptr::null, sync::{Arc, Mutex}
};

use config::{Port, ServerConfig};
use git::{
    check_git_server,
    server::{create_repo, handle_git_request, RepoManager},
};

use git2::{Repository, Status};
use warp::{
    reply::{with_header, with_status, Reply, Response},
    Filter,
};

// mod routes;
// pub mod error;
// pub mod db;
pub mod auth;
pub mod git;

struct ThreadSafeRepo {
    repo: Arc<Mutex<Repository>>,
}

impl ThreadSafeRepo {
    pub fn new(repo: Repository) -> Self {
        println!("🚀 Creating new ThreadSafeRepo instance...");
        Self {
            repo: Arc::new(Mutex::new(repo)),
        }
    }
    pub fn clone(&self) -> Self {
        println!("🔄 Cloning ThreadSafeRepo...");
        Self {
            repo: Arc::clone(&self.repo),
        }
    }
}

pub fn init() {
    println!("🌟 Initializing server...");
    let config: Option<ServerConfig> = None;
    let file = match std::fs::read_to_string("./harbr.config.json") {
        Ok(s) => {
            println!("✅ Successfully read server config:\n{s}");
            s
        }
        Err(e) => {
            println!("⚠️ Could not read server config: {e}. Using default config...");
            let default_config: ServerConfig = ServerConfig {
                fqdn: Some("".to_string()),
                main_port: Some(Port::default()),
                pool_size: Some(0),
            };
            let config_file = serde_json::to_string_pretty(&default_config);
            let _ = std::fs::write("./harbr.config.json", config_file.unwrap());
            println!("📝 Written default config to file!");
            run(Some(default_config));
            return;
        }
    };
    println!("🚀 Attempting to run with 'harbr.config.json'...");
    run(config);
}

#[tokio::main]
pub async fn run(config: Option<ServerConfig>) {
    println!("� Running server with config: {:?}", config);

    let repo_manager = Arc::new(RepoManager::new(PathBuf::from("./repos")));
    println!("📂 Initialized RepoManager at './repos'");

    let repo_manager = warp::any().map(move || repo_manager.clone());
    let new_repo = git2::Repository::open("./repos/new-repo").unwrap();
    println!("📂 Opened repository at './repos/new-repo'");

    let threaded_repo = ThreadSafeRepo::new(new_repo);
    println!("⚡ Created ThreadSafeRepo");

    let info_refs = {
        let repo = threaded_repo.clone();
        warp::path("info")
            .and(warp::path("refs"))
            .and(warp::query::<HashMap<String, String>>())
            .map(move |params: HashMap<String, String>| {
                println!("💭 Received request at 'info/refs' with params: {:?}", params);
                if params.get("service") != Some(&"git-upload-pack".to_string()) {
                    println!("🚫 Service not supported! Returning forbidden...");
                    return warp::reply::with_status(
                        "Service not supported",
                        warp::http::StatusCode::FORBIDDEN,
                    );
                }

                let repo_guard = repo.repo.lock();
                match repo_guard {
                    Ok(binding) => {
                        println!("🔒 Successfully locked repo for reading refs");
                        let refs = binding.references().unwrap();

                        let mut response = String::new();
                        response.push_str("001e# service=git-upload-pack\n0000");

                        let mut resp_clone = response.clone();
                        for r in refs {
                            match r {
                                Ok(r) => {
                                    let target_ref = r.target().expect("could not read target ref");
                                    let ref_name = r.name().expect("could not fetch ref name");
                                    let line = &format!("{} {}\n", target_ref, ref_name);
                                    let length = line.len() + 4;
                                    resp_clone.push_str(&format!("{:04x}{}", length, line));
                                }
                                Err(e) => {
                                    println!("⚠️ Failed to read ref: {:?}", e);
                                }
                            }
                        }
                        let response: Box<str> = response.into_boxed_str();
                        println!("💬 Returning refs response: {}", response);

                        // Leak the string for use in the response
                        let static_str = Box::leak(response); 
                        warp::reply::with_status(static_str, warp::http::StatusCode::OK)
                    }
                    Err(e) => {
                        println!("❌ Failed to lock repo: {:?}", e);
                        warp::reply::with_status(
                            "Internal server error",
                            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                        )
                    }
                }
            })
    };

    let create_repo_route = warp::post()
        .and(warp::path("repo"))
        .and(warp::path::end())
        .and(repo_manager.clone())
        .and(warp::body::json())
        .and_then(create_repo);

    let git_route = warp::path!("git" / String / String)
        .and(repo_manager)
        .and(warp::body::bytes())
        .and_then(handle_git_request);

    let routes = create_repo_route
        .or(git_route)
        .with(warp::cors().allow_any_origin());

    // Print and start the server
    println!("🎉 Starting Git server on 0.0.0.0:8080...");
    warp::serve(info_refs).run(([0, 0, 0, 0], 8080)).await;
    println!("🚀 Server running at http://0.0.0.0:8080");
}
