use warp::{Filter, Reply, filters::BoxedFilter};

mod repos;

// Register all routes
pub fn register() -> BoxedFilter<(impl Reply,)> {
    let api = warp::path("api");
    
    // Combine all route modules
    let routes = api.and(
        repos::routes()
    );

    routes.boxed()
}
