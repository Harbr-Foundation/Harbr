use warp::{Filter, Reply};
use super::super::db::Db;


pub mod handlers;
pub mod models;



pub fn routes() -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
    let repos = warp::path("repos");
    let repo = Arc::new(); 
    
    // GET /api/repos
    let list = repos
        .and(warp::get())
        .and(warp::query())
        .and(with_db())
        .and_then(handlers::list);
    
    // POST /api/repos
    let create = repos
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db())
        .and_then(handlers::create);

    // GET /api/repos/:id
    let get = repos
        .and(warp::get())
        .and(warp::path::param())
        .and(with_db())
        .and_then(handlers::get);

    // PATCH /api/repos/:id
    let update = repos
        .and(warp::patch())
        .and(warp::path::param())
        .and(warp::body::json())
        .and(with_db())
        .and_then(handlers::update);

    // DELETE /api/repos/:id
    let delete = repos
        .and(warp::delete())
        .and(warp::path::param())
        .and(with_db())
        .and_then(handlers::delete);

    list
        .or(create)
        .or(get)
        .or(update)
        .or(delete)
}

fn with_db() -> impl Filter<Extract = (Db,), Error = Infallible> + Clone {
    warp::any().map(|| Db::new())
}