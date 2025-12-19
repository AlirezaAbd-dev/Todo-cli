use std::fmt::Display;

pub enum TodoStatus {
    Pending,
    Doing,
    Done,
}

impl Display for TodoStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TodoStatus::Pending => write!(f, "Pending"),
            TodoStatus::Doing => write!(f, "Doing"),
            TodoStatus::Done => write!(f, "Done"),
        }
    }
}

impl TodoStatus {
    pub fn new(status: &str) -> TodoStatus {
        match status {
            "Pending" => TodoStatus::Pending,
            "Doing" => TodoStatus::Doing,
            "Done" => TodoStatus::Done,
            _ => TodoStatus::Pending,
        }
    }
}
