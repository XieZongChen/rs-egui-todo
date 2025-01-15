use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum TaskStatus {
    Todo,
    Doing,
    Done,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum TaskGenre {
    Low,
    Medium,
    High,
}

/**
 * 任务数据结构
 */
#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub status: TaskStatus, // 任务状态
    pub genre: TaskGenre,   // 任务类型
    pub content: String,    // 任务内容
}

impl Task {
    pub fn new() -> Task {
        Task {
            status: TaskStatus::Todo,
            genre: TaskGenre::Medium,
            content: String::new(),
        }
    }

    pub fn create_by_content(content: String) -> Task {
        Task {
            status: TaskStatus::Todo,
            genre: TaskGenre::Medium,
            content: content,
        }
    }
}
