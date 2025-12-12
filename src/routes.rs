use super::controllers;
use crate::middleware;
use crate::{controllers::auth::post_login_user, errors::handle_rejection};
use std::convert::Infallible;
use warp::{Filter, reply::Reply};

pub fn router()
-> impl Filter<Extract = impl Reply, Error = Infallible> + Clone + Send + Sync + 'static {
    let v1_routes = warp::path("v1").and(
        health_routes()
            .or(user_routes())
            .or(todos_routes())
            .or(file_routes())
            .or(ai_routes())
            .or(analytics_routes()),
    );

    warp::get()
        .and(warp::path::end())
        .and_then(controllers::root)
        .or(v1_routes)
        .with(warp::log("api"))
        .with(warp::cors().allow_any_origin())
        .recover(handle_rejection)
}

fn health_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path("health"))
        .and(warp::path::end())
        .and_then(controllers::health::get_health)
}

fn ai_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path("ai"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and_then(controllers::ai::post_todos_from_natural_language)
}

fn todos_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let todos_base = warp::path("todos").and(middleware::auth::authenticated());

    let get_one = todos_base
        .clone()
        .and(warp::path::param::<uuid::Uuid>())
        .and(warp::path::end())
        .and(warp::get())
        .and_then(controllers::todos::get_todo);

    let get_all = todos_base
        .clone()
        .and(warp::path::end())
        .and(warp::get())
        .and_then(controllers::todos::get_all_todos);

    let get_all_archived = todos_base
        .clone()
        .and(warp::path("archived"))
        .and(warp::path::end())
        .and(warp::get())
        .and_then(controllers::todos::get_archived_todos);

    let create = todos_base
        .clone()
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::body::json())
        .and_then(controllers::todos::post_todo);

    let delete = todos_base
        .clone()
        .and(warp::path::param::<uuid::Uuid>())
        .and(warp::path::end())
        .and(warp::delete())
        .and_then(controllers::todos::delete_todo);

    let restore = todos_base
        .clone()
        .and(warp::path::param::<uuid::Uuid>())
        .and(warp::path("restore"))
        .and(warp::path::end())
        .and(warp::patch())
        .and_then(controllers::todos::patch_restore_todo);

    let toggle = todos_base
        .clone()
        .and(warp::path::param::<uuid::Uuid>())
        .and(warp::path("toggle"))
        .and(warp::path::param::<bool>())
        .and(warp::path::end())
        .and(warp::patch())
        .and_then(controllers::todos::patch_toggle_todo_status);

    get_all
        .or(get_all_archived)
        .or(create)
        .or(get_one)
        .or(toggle)
        .or(delete)
        .or(restore)
}

fn file_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let files_base = warp::path("file")
        .and(warp::path("todos"))
        .and(warp::post())
        .and(middleware::auth::authenticated());

    let upload_csv = files_base
        .clone()
        .and(warp::path("csv"))
        .and(warp::path::end())
        .and(warp::multipart::form().max_length(5_000_000))
        .and_then(controllers::file::post_upload_csv);

    let upload_json = files_base
        .clone()
        .and(warp::path("json"))
        .and(warp::path::end())
        .and(warp::multipart::form().max_length(5_000_000))
        .and_then(controllers::file::post_upload_json);

    upload_csv.or(upload_json)
}

fn analytics_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let analytics_base = warp::path("analytics").and(middleware::auth::authenticated());

    let todo_anlytics = analytics_base
        .clone()
        .and(warp::path("todos"))
        .and(warp::path::end())
        .and(warp::get())
        .and_then(controllers::analytics::get_todos_analytics);

    todo_anlytics
}

fn user_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let auth_base = warp::path("auth");

    let register = auth_base
        .and(warp::path("register"))
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::body::json())
        .and_then(controllers::auth::post_register_user);

    let login = auth_base
        .and(warp::path("login"))
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::body::json())
        .and_then(post_login_user);

    register.or(login)
}
