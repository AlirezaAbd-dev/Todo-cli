use std::fmt::Display;

use crate::models::todo_status::TodoStatus;

pub struct Todo {
    pub id: u64,
    pub name: String,
    pub status: TodoStatus,
}

impl Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            " id: {}, name: {}, status: {}",
            self.id, self.name, self.status
        )
    }
}
