use crate::errors::ServiceError;
use crate::utils::jwt;
use uuid::Uuid;
use warp::Filter;
use warp::http::header::{AUTHORIZATION, HeaderValue};

pub fn authenticated() -> impl Filter<Extract = (Uuid,), Error = warp::Rejection> + Clone {
    warp::header::optional::<HeaderValue>(AUTHORIZATION.as_str()).and_then(
        |auth_header: Option<HeaderValue>| async move {
            match auth_header {
                Some(header) => jwt::verify_token(&header)
                    .map_err(|_| warp::reject::custom(ServiceError::Unauthorized)),
                None => Err(warp::reject::custom(ServiceError::Unauthorized)),
            }
        },
    )
}
