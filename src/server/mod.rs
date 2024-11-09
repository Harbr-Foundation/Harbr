use warp::Filter;

#[tokio::main]
pub async fn init() {
    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

    println!("running main instance on localhost:3030");
    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
}
