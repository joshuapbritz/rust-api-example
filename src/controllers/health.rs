use crate::{errors::ServiceError, utils::database};
use diesel::prelude::*;
use std::time::Instant;

pub async fn get_health() -> Result<impl warp::Reply, warp::Rejection> {
    let start_time = Instant::now();

    log::debug!("Begin health check");

    let mut connection =
        database::get().map_err(|_| ServiceError::Unhealthy("db connection failed"))?;

    log::debug!("DB Connection Success");

    diesel::sql_query("SELECT 1")
        .execute(&mut connection)
        .map_err(|_| ServiceError::Unhealthy("db test query failed"))?;

    log::debug!("DB Query Success");
    log::debug!("Health Check Completed in {:?}", start_time.elapsed());

    Ok(warp::reply::with_status(
        warp::reply::reply(),
        warp::http::StatusCode::OK,
    ))
}
