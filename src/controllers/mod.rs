pub mod ai;
pub mod analytics;
pub mod auth;
pub mod file;
pub mod health;
pub mod todos;

pub async fn root() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::with_status(
        warp::reply::reply(),
        warp::http::StatusCode::OK,
    ))
}
