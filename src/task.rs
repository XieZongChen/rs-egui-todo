use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum TaskStatus {
    Todo,
    Doing,
    Done,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum TaskGenre {
    Low,
    Medium,
    High,
}

/**
 * 任务数据结构
 */
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Task {
    pub id: usize,          // 唯一 id
    pub status: TaskStatus, // 任务状态
    pub genre: TaskGenre,   // 任务类型
    pub content: String,    // 任务内容
}

impl Task {
    pub fn create(content: String, id: usize) -> Task {
        Task {
            id: id,
            status: TaskStatus::Todo,
            genre: TaskGenre::Medium,
            content: content,
        }
    }

    pub fn ui(&self, ui: &mut egui::Ui, on_delete: impl FnOnce(), on_status_change: impl FnOnce(TaskStatus)) {
        ui.horizontal(|ui| {
            ui.label(&self.content);
            if ui.button("del").clicked() {
                on_delete();
            }
            if ui.button("ok").clicked() {
                on_status_change(Task::get_next_status(self.status.clone()));
            }
        });
    }

    fn get_next_status(cur_status: TaskStatus) -> TaskStatus {
        return match cur_status {
            TaskStatus::Todo => TaskStatus::Doing,
            TaskStatus::Doing => TaskStatus::Done,
            TaskStatus::Done => TaskStatus::Done,
        };
    }
}
