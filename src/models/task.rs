use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Task {
    pub title: String,
    pub body: String,
    pub completed_on: Option<DateTime<Utc>>,
    pub due_date: Option<DateTime<Utc>>,
    pub importance: TaskImportance,
    pub is_reminder_on: bool,
    pub reminder_date: Option<DateTime<Utc>>,
    pub status: TaskStatus,
    pub created_date_time: DateTime<Utc>,
    pub last_modified_date_time: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub enum TaskImportance {
    Low,
    Normal,
    High,
}

impl Default for TaskImportance {
    fn default() -> Self {
        TaskImportance::Normal
    }
}

#[derive(Debug, Clone)]
pub enum TaskStatus {
    NotStarted,
    Completed,
}

impl Default for TaskStatus {
    fn default() -> Self {
        TaskStatus::NotStarted
    }
}