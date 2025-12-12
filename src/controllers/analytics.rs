use crate::{
    errors::ServiceError,
    models::{analytics::TodoAnalyticsResponse, todos::TodoDTO},
    schema::{todos::dsl::*, user_todos},
    utils::database,
};
use diesel::prelude::*;

pub async fn get_todos_analytics(user_id: uuid::Uuid) -> Result<impl warp::Reply, warp::Rejection> {
    let mut connection = database::get().map_err(ServiceError::from)?;

    let results = todos
        .inner_join(user_todos::table.on(user_todos::todo_id.eq(id)))
        .filter(user_todos::user_id.eq(user_id))
        .select(TodoDTO::as_select())
        .load(&mut connection)
        .map_err(ServiceError::from)?;

    Ok(warp::reply::with_status(
        warp::reply::json(&TodoAnalyticsResponse::new(results)),
        warp::http::StatusCode::OK,
    ))
}
