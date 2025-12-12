use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::todos::{self, dsl};
use crate::schema::user_todos;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = user_todos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserTodo {
    pub user_id: Uuid,
    pub todo_id: Uuid,
}

impl UserTodo {
    pub fn link(self, db_conn: &mut PgConnection) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(user_todos::table)
            .values(&self)
            .get_result(db_conn)
    }

    pub fn new(todo_id: Uuid, user_id: Uuid) -> Self {
        Self { todo_id, user_id }
    }

    pub fn link_many(
        user_todos: Vec<Self>,
        db_conn: &mut PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        diesel::insert_into(user_todos::table)
            .values(&user_todos)
            .get_results(db_conn)
    }
}

#[derive(Queryable, Selectable, Clone)]
#[diesel(table_name = crate::schema::todos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TodoDTO {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub completed_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Deserialize, Serialize)]
pub struct TodoResponse {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub completed_at: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl From<TodoDTO> for TodoResponse {
    fn from(todo: TodoDTO) -> Self {
        Self {
            id: todo.id,
            title: todo.title,
            body: todo.body,
            completed_at: todo.completed_at,
            created_at: todo.created_at,
            updated_at: todo.updated_at,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct TodoRequest {
    pub title: String,
    pub body: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = todos)]
pub struct InsertableTodo {
    pub title: String,
    pub body: String,
}

impl From<&TodoRequest> for InsertableTodo {
    fn from(todo: &TodoRequest) -> Self {
        Self {
            title: todo.title.clone(),
            body: todo.body.clone(),
        }
    }
}

impl InsertableTodo {
    pub fn insert(self, db_conn: &mut PgConnection) -> Result<TodoDTO, diesel::result::Error> {
        let todo: TodoDTO = diesel::insert_into(dsl::todos)
            .values(&self)
            .get_result(db_conn)?;

        Ok(todo)
    }

    pub fn insert_many(
        todos: Vec<Self>,
        db_conn: &mut PgConnection,
    ) -> Result<Vec<TodoDTO>, diesel::result::Error> {
        diesel::insert_into(dsl::todos)
            .values(&todos)
            .get_results(db_conn)
    }
}
