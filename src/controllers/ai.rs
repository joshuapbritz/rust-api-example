use crate::{
    errors::ServiceError,
    models::{
        ai::UserAiRequest,
        todos::{InsertableTodo, TodoResponse, UserTodo},
    },
    utils::{ai::ollama, database},
};

pub async fn post_todos_from_natural_language(
    user_id: uuid::Uuid,
    input: UserAiRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut connection = database::get().map_err(ServiceError::from)?;
    let result = ollama::create_todos(input.input).await?;

    let created_todos = InsertableTodo::insert_many(result, &mut connection)
        .map_err(|_| ServiceError::BadRequest)?;

    let user_todo_links: Vec<UserTodo> = created_todos
        .iter()
        .map(|todo| UserTodo::new(todo.id, user_id))
        .collect();

    UserTodo::link_many(user_todo_links, &mut connection).map_err(|_| ServiceError::BadRequest)?;

    let response: Vec<TodoResponse> = created_todos.into_iter().map(|todo| todo.into()).collect();

    Ok(warp::reply::with_status(
        warp::reply::json(&response),
        warp::http::StatusCode::OK,
    ))
}
