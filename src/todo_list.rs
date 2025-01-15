use eframe::App; // 引入 eframe 库中的 App trait，用于定义应用程序的行为
use egui::Context; // 引入 egui 库中的 Context 结构体，用于管理 egui 的绘制上下文

use crate::Task;

// 派生 Deserialize、Serialize，这样就可以在关机时持久化应用状态
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]

pub struct TodoList {
    // 定义一个公开的结构体 TodoList
    items: Vec<Task>, // 定义一个字段 items，类型为 Vec<Task>，用于存储待办事项列表
    new_item: String,   // 定义一个字段 new_item，类型为 Task，用于存储用户输入的新待办事项
}

// 默认数据初始化
impl Default for TodoList {
    fn default() -> Self {
        Self {
            items: vec![Task::create_by_content("Item 1".to_string()), Task::create_by_content("Item 2".to_string())], // 初始化 items 字段，包含两个示例待办事项
            new_item: String::new(), // 初始化 new_item 字段，为空字符串
        }
    }
}

impl TodoList {
    // 开始实现 TodoList 的方法
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            // 如果发现 storage 有值，则使用 storage 的值
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        // 初始化应用的值
        Default::default()
    }
}

impl App for TodoList {
    // 开始为 TodoList 实现 App trait
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // 实现 update 方法，该方法在每一帧被调用，用于更新 UI
        egui::CentralPanel::default().show(ctx, |ui| {
            // 在中央面板中显示 UI 内容
            ui.heading("Todo List"); // 显示一个标题 "Todo List"
            ui.horizontal(|ui| {
                // 创建一个水平布局
                ui.text_edit_singleline(&mut self.new_item); // 显示一个单行文本编辑框，绑定到 new_item
                if ui.button("Add").clicked() {
                    // 显示一个按钮 "Add"，当按钮被点击时执行以下代码
                    if !self.new_item.is_empty() {
                        // 如果 new_item 不为空
                        self.items.push(Task::create_by_content(self.new_item.clone())); // 将 new_item 复制并添加到 items 列表中
                        self.new_item.clear(); // 清空 new_item
                    }
                }
            });
            ui.separator(); // 显示一个分隔线
            for item in &self.items {
                // 遍历 items 列表
                ui.label(item.content.clone()); // 显示每一个待办事项
            }
        });
    }

    // 关闭前保存状态
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
