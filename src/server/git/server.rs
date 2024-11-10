use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tokio::sync::RwLock;
use warp::{
    http::StatusCode,
    reject::Rejection,
    reply::{Json, Reply, Response},
    Filter,
};
use serde::{Deserialize, Serialize};

// Repository configuration and status information
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RepoConfig {
    name: String,
    path: Option<PathBuf>,
    description: Option<String>,
    active: bool,
}

// Repository manager to handle multiple repositories
#[derive(Clone)]
pub struct RepoManager {
    repos: Arc<RwLock<HashMap<String, RepoConfig>>>,
    base_path: PathBuf,
}

impl RepoManager {
    pub fn new(base_path: PathBuf) -> Self {
        Self {
            repos: Arc::new(RwLock::new(HashMap::new())),
            base_path,
        }
    }

    async fn add_repo(&self, name: String, description: Option<String>) -> Result<(), String> {
        println!("creating repo");
        let repo_path = self.base_path.join(&name);
        
        // Initialize new bare git repository
        let output = tokio::process::Command::new("git")
            .arg("init")
            .arg("--bare")
            .arg(&repo_path)
            .output()
            .await
            .map_err(|e| format!("Failed to initialize repository: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        let config = RepoConfig {
            name: name.clone(),
            path: Some(repo_path),
            description,
            active: true,
        };

        self.repos.write().await.insert(name, config);
        Ok(())
    }

    async fn handle_git_http(&self, repo_name: String, action: &str, data: Vec<u8>) -> Result<Vec<u8>, String> {
        let repos = self.repos.read().await;
        let repo = repos.get(&repo_name)
            .ok_or_else(|| format!("Repository {} not found", repo_name))?;

        if !repo.active {
            return Err("Repository is inactive".to_string());
        }

        let mut command = tokio::process::Command::new("git");
        command.current_dir(&repo.path.clone().unwrap());

        match action {
            "upload-pack" => {
                command.arg("upload-pack")
                    .arg("--stateless-rpc")
                    .arg(".");
            },
            "receive-pack" => {
                command.arg("receive-pack")
                    .arg("--stateless-rpc")
                    .arg(".");
            },
            _ => return Err("Invalid git action".to_string()),
        }

        let mut child = command.stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to spawn git process: {}", e))?;

        // Write incoming data to git process
        if let Some(mut stdin) = child.stdin.take() {
            tokio::spawn(async move {
                use tokio::io::AsyncWriteExt;
                stdin.write_all(&data).await
            });
        }

        // Read response from git process
        let output = child.wait_with_output()
            .await
            .map_err(|e| format!("Failed to get git output: {}", e))?;

        if output.status.success() {
            Ok(output.stdout)
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }
}

// HTTP handlers for the git server
pub async fn create_repo(
    manager: Arc<RepoManager>,
    body: RepoConfig,
) -> Result<impl Reply, Rejection> {
    match manager.add_repo(body.name, body.description).await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e) => Ok(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn handle_git_request(
    repo_name: String,
    action: String,
    manager: Arc<RepoManager>,
    body: warp::hyper::body::Bytes,
) -> Result<impl Reply, Rejection> {
    match manager.handle_git_http(repo_name, &action, body.to_vec()).await {
        Ok(response) => Ok(Response::new(response.into())),
        Err(_) => Ok(Response::new(Vec::new().into())),
    }
}

#[tokio::main]
async fn main() {
    // Initialize repository manager
    let repo_manager = Arc::new(RepoManager::new(PathBuf::from("./repos")));
    let repo_manager = warp::any().map(move || repo_manager.clone());

    // Route for creating new repositories
    let create_repo_route = warp::post()
        .and(warp::path("repo"))
        .and(warp::path::end())
        .and(repo_manager.clone())
        .and(warp::body::json())
        .and_then(create_repo);

    // Route for git HTTP protocol
    let git_route = warp::path!("git" / String / String)
        .and(repo_manager)
        .and(warp::body::bytes())
        .and_then(handle_git_request);

    // Combine routes
    let routes = create_repo_route
        .or(git_route)
        .with(warp::cors().allow_any_origin());

    println!("Starting git server on 0.0.0.0:22");
    warp::serve(routes).run(([0, 0, 0, 0], 22)).await;
}

// Tests module
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[tokio::test]
    async fn test_repo_manager() {
        let temp_dir = tempdir().unwrap();
        let manager = RepoManager::new(temp_dir.path().to_path_buf());
        
        // Test repository creation
        let result = manager.add_repo(
            "test-repo".to_string(),
            Some("Test repository".to_string())
        ).await;
        assert!(result.is_ok());
        
        // Verify repository was created
        let repos = manager.repos.read().await;
        assert!(repos.contains_key("test-repo"));
    }
}