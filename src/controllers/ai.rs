use crate::{errors::ServiceError, models::ai::UserAiRequest, utils::ai::gpt};

pub async fn post_todos_from_natural_language(
    input: UserAiRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    let request = gpt::prompt(input.input)
        .await
        .map_err(|_| ServiceError::BadRequest)?;

    Ok(warp::reply::with_status(
        warp::reply::json(&request),
        warp::http::StatusCode::OK,
    ))
}
