use crate::{
    errors::ServiceError,
    models::todos::{InsertableTodo, UserTodo},
    utils::{database, parser},
};
use uuid::Uuid;

pub async fn post_upload_csv(
    user_id: Uuid,
    data: warp::multipart::FormData,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut connection = database::get().map_err(|error| ServiceError::from(error))?;

    let todos = parser::process_csv(data).await?;

    let inserted_todos =
        InsertableTodo::insert_many(todos, &mut connection).map_err(ServiceError::from)?;

    let user_todo_links: Vec<UserTodo> = inserted_todos
        .iter()
        .map(|todo| UserTodo::new(todo.id, user_id))
        .collect();

    let linked =
        UserTodo::link_many(user_todo_links, &mut connection).map_err(ServiceError::from)?;

    Ok(warp::reply::with_status(
        warp::reply::json(&serde_json::json!({ "total": linked.len() })),
        warp::http::StatusCode::OK,
    ))
}

pub async fn post_upload_json(
    _user_id: Uuid,
    data: warp::multipart::FormData,
) -> Result<impl warp::Reply, warp::Rejection> {
    let todos = parser::process_json(data).await?;

    Ok(warp::reply::with_status(
        warp::reply::json(&todos),
        warp::http::StatusCode::OK,
    ))
}
