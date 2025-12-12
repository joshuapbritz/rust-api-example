use crate::{
    errors::ServiceError,
    models::todos::{InsertableTodo, TodoDTO, TodoRequest, TodoResponse, UserTodo},
    schema::todos::dsl::*,
    schema::user_todos,
    utils::database,
};
use diesel::prelude::*;

pub async fn get_todo(
    user_id: uuid::Uuid,
    todo_id: uuid::Uuid,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut connection = database::get().map_err(|error| ServiceError::from(error))?;

    let result = todos
        .inner_join(user_todos::table.on(user_todos::todo_id.eq(id)))
        .filter(id.eq(todo_id))
        .filter(user_todos::user_id.eq(user_id))
        .filter(deleted_at.is_null())
        .select(TodoDTO::as_select())
        .first(&mut connection)
        .map_err(ServiceError::from)?;

    Ok(warp::reply::with_status(
        warp::reply::json(&TodoResponse::from(result)),
        warp::http::StatusCode::OK,
    ))
}

pub async fn get_all_todos(user_id: uuid::Uuid) -> Result<impl warp::Reply, warp::Rejection> {
    let mut connection = database::get().map_err(ServiceError::from)?;

    let results = todos
        .inner_join(user_todos::table.on(user_todos::todo_id.eq(id)))
        .filter(user_todos::user_id.eq(user_id))
        .filter(deleted_at.is_null())
        // .limit(5) // TODO: add paging
        .select(TodoDTO::as_select())
        .load(&mut connection)
        .map_err(ServiceError::from)?;

    let response: Vec<TodoResponse> = results.into_iter().map(|todo| todo.into()).collect();

    Ok(warp::reply::with_status(
        warp::reply::json(&response),
        warp::http::StatusCode::OK,
    ))
}

pub async fn get_archived_todos(user_id: uuid::Uuid) -> Result<impl warp::Reply, warp::Rejection> {
    let mut connection = database::get().map_err(|error| ServiceError::from(error))?;

    let results = todos
        .inner_join(user_todos::table.on(user_todos::todo_id.eq(id)))
        .filter(user_todos::user_id.eq(user_id))
        .filter(deleted_at.is_not_null())
        // .limit(5) // TODO: add paging
        .select(TodoDTO::as_select())
        .load(&mut connection)
        .map_err(ServiceError::from)?;

    let response: Vec<TodoResponse> = results.into_iter().map(|todo| todo.into()).collect();

    Ok(warp::reply::with_status(
        warp::reply::json(&response),
        warp::http::StatusCode::OK,
    ))
}

pub async fn post_todo(
    user_id: uuid::Uuid,
    todo: TodoRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut connection = database::get().map_err(ServiceError::from)?;

    let new_todo = InsertableTodo::from(&todo);

    let result = new_todo
        .insert(&mut connection)
        .map_err(ServiceError::from)?;

    let user_todo = UserTodo::new(result.id, user_id);

    user_todo
        .link(&mut connection)
        .map_err(ServiceError::from)?;

    Ok(warp::reply::with_status(
        warp::reply::json(&TodoResponse::from(result)),
        warp::http::StatusCode::OK,
    ))
}

pub async fn delete_todo(
    user_id: uuid::Uuid,
    todo_id: uuid::Uuid,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut connection = database::get().map_err(|error| ServiceError::from(error))?;

    let count = todos
        .inner_join(user_todos::table.on(user_todos::todo_id.eq(id)))
        .filter(id.eq(todo_id))
        .filter(user_todos::user_id.eq(user_id))
        .filter(deleted_at.is_null())
        .select(diesel::dsl::count(id))
        .first::<i64>(&mut connection)
        .map_err(ServiceError::from)?;

    if count <= 0 {
        return Err(warp::reject::custom(ServiceError::BadRequest));
    }

    let result = diesel::update(todos.filter(id.eq(todo_id)))
        .set(deleted_at.eq(Some(chrono::Utc::now().naive_utc())))
        .returning(TodoDTO::as_returning())
        .get_result::<TodoDTO>(&mut connection)
        .map_err(ServiceError::from)?;

    Ok(warp::reply::with_status(
        warp::reply::json(&TodoResponse::from(result)),
        warp::http::StatusCode::OK,
    ))
}

pub async fn patch_restore_todo(
    user_id: uuid::Uuid,
    todo_id: uuid::Uuid,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut connection = database::get().map_err(|error| ServiceError::from(error))?;

    let count = todos
        .inner_join(user_todos::table.on(user_todos::todo_id.eq(id)))
        .filter(id.eq(todo_id))
        .filter(user_todos::user_id.eq(user_id))
        .filter(deleted_at.is_not_null())
        .select(diesel::dsl::count(id))
        .first::<i64>(&mut connection)
        .map_err(ServiceError::from)?;

    if count <= 0 {
        return Err(warp::reject::custom(ServiceError::BadRequest));
    }

    let result = diesel::update(todos.filter(id.eq(todo_id)))
        .set(deleted_at.eq(Option::<chrono::NaiveDateTime>::None))
        .returning(TodoDTO::as_returning())
        .get_result::<TodoDTO>(&mut connection)
        .map_err(ServiceError::from)?;

    Ok(warp::reply::with_status(
        warp::reply::json(&TodoResponse::from(result)),
        warp::http::StatusCode::OK,
    ))
}

pub async fn patch_toggle_todo_status(
    user_id: uuid::Uuid,
    todo_id: uuid::Uuid,
    value_to: bool,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut connection = database::get().map_err(|error| ServiceError::from(error))?;

    let count = todos
        .inner_join(user_todos::table.on(user_todos::todo_id.eq(id)))
        .filter(id.eq(todo_id))
        .filter(user_todos::user_id.eq(user_id))
        .filter(deleted_at.is_null())
        .select(diesel::dsl::count(id))
        .first::<i64>(&mut connection)
        .map_err(ServiceError::from)?;

    if count <= 0 {
        return Err(warp::reject::custom(ServiceError::BadRequest));
    }

    let todo = diesel::update(todos.filter(id.eq(todo_id)));

    let action = if value_to == true {
        todo.set(completed_at.eq(Some(chrono::Utc::now().naive_utc())))
    } else {
        todo.set(completed_at.eq(Option::<chrono::NaiveDateTime>::None))
    };

    let result = action
        .returning(TodoDTO::as_returning())
        .get_result::<TodoDTO>(&mut connection)
        .map_err(ServiceError::from)?;

    Ok(warp::reply::with_status(
        warp::reply::json(&TodoResponse::from(result)),
        warp::http::StatusCode::OK,
    ))
}
