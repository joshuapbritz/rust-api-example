use crate::models::todos::{self, TodoDTO, TodoResponse};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TodoAnalyticsStats {
    pub total_todos: i64,
    pub total_unarchived_todos: i64,
    pub complete_todos: i64,
    pub incomplete_todos: i64,
    pub total_incomplete_todos: i64,
}

impl TodoAnalyticsStats {
    fn new(todos: &Vec<todos::TodoDTO>) -> Self {
        let total = todos.len() as i64;

        let total_unarchived = todos
            .iter()
            .filter(|todo| todo.deleted_at.is_none())
            .count() as i64;

        let unarchived_complete = todos
            .iter()
            .filter(|todo| todo.completed_at.is_some() && todo.deleted_at.is_none())
            .count() as i64;

        let complete = todos
            .iter()
            .filter(|todo| todo.completed_at.is_some())
            .count() as i64;

        Self {
            total_todos: total,
            total_unarchived_todos: total_unarchived,
            complete_todos: complete,
            incomplete_todos: total_unarchived - unarchived_complete,
            total_incomplete_todos: total - complete,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct TodoAnalyticsResponse {
    pub todo_stats: TodoAnalyticsStats,
    pub oldest_incomplete_todo: Option<TodoResponse>,
}

impl TodoAnalyticsResponse {
    pub fn new(todos: Vec<todos::TodoDTO>) -> Self {
        let oldest_todo = match Self::oldest_incomplete(&todos) {
            Some(todo) => Some(TodoResponse::from(todo)),
            None => None,
        };

        Self {
            todo_stats: TodoAnalyticsStats::new(&todos),
            oldest_incomplete_todo: oldest_todo,
        }
    }

    fn oldest_incomplete(todos: &Vec<todos::TodoDTO>) -> Option<TodoDTO> {
        todos
            .iter()
            .filter(|todo| todo.completed_at.is_none() && todo.deleted_at.is_none())
            .min_by_key(|todo| todo.created_at)
            .cloned()
    }
}
