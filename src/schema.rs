// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Uuid,
        title -> Varchar,
        body -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        completed_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    user_todos (user_id, todo_id) {
        user_id -> Uuid,
        todo_id -> Uuid,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        email -> Varchar,
        name -> Varchar,
        surname -> Varchar,
        password_hash -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(user_todos -> todos (todo_id));
diesel::joinable!(user_todos -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(todos, user_todos, users,);
