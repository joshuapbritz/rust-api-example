use crate::{
    errors::ServiceError,
    models::auth::{InsertableUser, UserDTO, UserLoginRequest, UserRequest, UserResponseWithToken},
    utils::{crypto::pwdhash, database, jwt::JwtClaims},
};

use diesel::prelude::*;

pub async fn post_register_user(user: UserRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let mut connection = database::get().map_err(ServiceError::from)?;

    let new_user = InsertableUser::from(&user).map_err(|_| ServiceError::BadRequest)?;
    let result = new_user.insert(&mut connection)?;

    let jwt = JwtClaims::from_user_id(&result.id)
        .sign()
        .map_err(ServiceError::from)?;

    let user = UserResponseWithToken::create(result, jwt);

    Ok(warp::reply::with_status(
        warp::reply::json(&user),
        warp::http::StatusCode::OK,
    ))
}

pub async fn post_login_user(user: UserLoginRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let mut db = database::get().map_err(ServiceError::from)?;

    use crate::schema::users::dsl::{email, users};

    let matched_user = users
        .filter(email.eq(&user.email))
        .select(UserDTO::as_select())
        .first::<UserDTO>(&mut db)
        .map_err(|_| ServiceError::Unauthorized)?;

    pwdhash::verify(&matched_user.password_hash, &user.password)?;

    let jwt = JwtClaims::from_user_id(&matched_user.id)
        .sign()
        .map_err(ServiceError::from)?;

    let user = UserResponseWithToken::create(matched_user, jwt);

    Ok(warp::reply::with_status(
        warp::reply::json(&user),
        warp::http::StatusCode::OK,
    ))
}
