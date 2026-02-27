use serde::{Deserialize, Serialize}; // 引入 serde 的 Deserialize、Serialize，用于结构的序列化与反序列化

#[derive(Debug, Deserialize, Serialize, Clone)] // 派生调试输出、序列化、克隆
pub enum TaskStatus {
    // 任务状态枚举
    Todo,  // 待办
    Doing, // 进行中
    Done,  // 已完成
}

#[derive(Debug, Deserialize, Serialize, Clone)] // 派生调试输出、序列化、克隆
pub enum TaskGenre {
    // 任务优先级或类别
    Low,    // 低
    Medium, // 中
    High,   // 高
}

/** // 文档注释：任务数据结构 */
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Task {
    // 任务结构体，描述单个待办项
    pub id: usize,          // 唯一 id
    pub status: TaskStatus, // 任务状态
    pub genre: TaskGenre,   // 任务类型/优先级
    pub content: String,    // 任务内容文本
}

impl Task {
    // 实现与 Task 相关的方法
    pub fn create(content: String, id: usize) -> Task {
        // 构造一个默认的 Task
        Task {
            // 返回 Task 实例
            id: id,                   // 赋值唯一 id
            status: TaskStatus::Todo, // 初始状态为待办
            genre: TaskGenre::Medium, // 默认优先级为中
            content: content,         // 填入任务内容
        }
    } // create 结束

    pub fn ui(
        &self,
        ui: &mut egui::Ui,
        on_delete: impl FnOnce(),
        on_status_change: impl FnOnce(TaskStatus),
    ) {
        // 绘制当前任务的一行 UI
        ui.horizontal(|ui| {
            // 水平布局：左侧文本，右侧按钮
            let label_resp = ui.add(egui::Label::new(&self.content)); // 添加显示任务内容的 Label，并拿到响应
            let btn_w = ui.spacing().interact_size.x * 2.0 + ui.spacing().item_spacing.x; // 计算按钮区域宽度（两个按钮+间距）
            let btn_h = ui.spacing().interact_size.y; // 使用交互控件的标准高度
            let (buttons_rect, buttons_resp) =
                ui.allocate_exact_size(egui::vec2(btn_w, btn_h), egui::Sense::hover()); // 预留一块矩形区域并可感知 hover
            let hovered = ui.rect_contains_pointer(label_resp.rect.union(buttons_rect))
                || buttons_resp.hovered(); // 鼠标在文本或按钮区域上即视为 hover
            if hovered {
                // 仅在 hover 时真正绘制按钮，避免常驻挤占布局
                let mut child = ui.new_child(egui::UiBuilder::new().max_rect(buttons_rect)); // 在预留矩形内创建子 Ui
                if child.button("del").clicked() {
                    // 删除按钮：点击后调用外部删除回调
                    on_delete(); // 调用删除回调
                } // if 删除结束
                if child.button("ok").clicked() {
                    // 状态流转按钮：点击推进到下一个状态
                    on_status_change(Task::get_next_status(self.status.clone()));
                    // 调用状态变更回调
                } // if 状态推进结束
            } // if hovered 结束
        }); // 水平布局结束
    } // ui 方法结束

    fn get_next_status(cur_status: TaskStatus) -> TaskStatus {
        // 计算下一个状态的简单状态机
        return match cur_status {
            // 基于当前状态匹配到下一个状态
            TaskStatus::Todo => TaskStatus::Doing, // 待办 -> 进行中
            TaskStatus::Doing => TaskStatus::Done, // 进行中 -> 完成
            TaskStatus::Done => TaskStatus::Done,  // 完成 -> 保持完成
        }; // match 结束
    } // get_next_status 结束
} // impl Task 块结束
