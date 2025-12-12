mod config;
mod controllers;
mod errors;
mod middleware;
mod models;
mod routes;
mod schema;
mod utils;

#[tokio::main]
async fn main() {
    let cfg = config::config();

    let routes = routes::router();

    println!("Server started at http://localhost:3030");

    warp::serve(routes)
        .run(([0, 0, 0, 0], cfg.server_port))
        .await;
}
