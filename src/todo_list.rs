use eframe::App; // 引入 eframe 库中的 App trait，用于定义应用程序的行为
use egui::Context; // 引入 egui 库中的 Context 结构体，用于管理 egui 的绘制上下文
use indexmap::IndexMap;

use crate::Task;

// 派生 Deserialize、Serialize，这样就可以在关机时持久化应用状态
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]

pub struct TodoList {
    // 定义一个公开的结构体 TodoList
    items: IndexMap<usize, Task>, // 定义一个字段 items，类型为 IndexMap<usize, Task>，用于存储待办事项列表
    new_item: String, // 定义一个字段 new_item，类型为 String，用于存储用户输入的新待办事项
    next_id: usize,   // 用于生成下一个任务的唯一 ID
}

// 默认数据初始化
impl Default for TodoList {
    fn default() -> Self {
        Self {
            items: IndexMap::new(),  // 初始化 items 字段
            new_item: String::new(), // 初始化 new_item 字段，为空字符串
            next_id: 2,              // 初始化 next_id，从 2 开始
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
            ui.heading("Todo List"); // 显示一个标题
            ui.horizontal(|ui| {
                // 创建一个水平布局
                ui.text_edit_singleline(&mut self.new_item); // 显示一个单行文本编辑框，绑定到 new_item
                if ui.button("Add").clicked() {
                    // 显示一个按钮 "Add"，当按钮被点击时执行以下代码
                    if !self.new_item.is_empty() {
                        // 如果 new_item 不为空
                        self.items.insert(
                            self.next_id,
                            Task::create(self.new_item.clone(), self.next_id),
                        ); // 将 new_item 复制并添加到 items 列表中
                        self.new_item.clear(); // 清空 new_item
                        self.next_id += 1; // 递增 next_id
                    }
                }
            });
            ui.separator(); // 显示一个分隔线
            let mut to_delete = None; // 记录当前帧需要删除的 task id
            let mut status_update = None; // 记录当前帧需要更新的 task id
            
            // 由于遍历中对 items 有多处修改，直接遍历 items 会导致借用冲突，通过先收集所有的 keys 到一个新的 Vec 中再进行遍历，可以有效解决这个问题
            let ids: Vec<usize> = self.items.keys().cloned().collect();
            
            // 创建三个区域分别显示不同状态的任务
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 10.0; // 设置水平布局中元素之间的水平间距
                let available_width = ui.available_width(); // 获取当前区域的可用宽度
                let column_width = (available_width - 20.0) / 3.0; // 计算每个区域的宽度

                ui.vertical(|ui| {
                    ui.set_min_width(column_width);
                    ui.heading("待办");
                    for id in &ids {
                        if let Some(item) = self.items.get(id) {
                            if matches!(item.status, crate::TaskStatus::Todo) {
                                let item_clone = item.clone();
                                item_clone.ui(ui, 
                                    || { to_delete = Some(*id); },
                                    |new_status| { status_update = Some((*id, new_status)); }
                                );
                            }
                        }
                    }
                });
                ui.separator();
                ui.vertical(|ui| {
                    ui.set_min_width(column_width);
                    ui.heading("进行中");
                    for id in &ids {
                        if let Some(item) = self.items.get(id) {
                            if matches!(item.status, crate::TaskStatus::Doing) {
                                let item_clone = item.clone();
                                item_clone.ui(ui, 
                                    || { to_delete = Some(*id); },
                                    |new_status| { status_update = Some((*id, new_status)); }
                                );
                            }
                        }
                    }
                });
                ui.separator();
                ui.vertical(|ui| {
                    ui.set_min_width(column_width);
                    ui.heading("已完成");
                    for id in &ids {
                        if let Some(item) = self.items.get(id) {
                            if matches!(item.status, crate::TaskStatus::Done) {
                                let item_clone = item.clone();
                                item_clone.ui(ui, 
                                    || { to_delete = Some(*id); },
                                    |new_status| { status_update = Some((*id, new_status)); }
                                );
                            }
                        }
                    }
                });
            });

            
            if let Some(id) = to_delete {
                self.items.shift_remove(&id);
            }
            if let Some((id, new_status)) = status_update {
                if let Some(task) = self.items.get_mut(&id) {
                    task.status = new_status;
                }
            }
        });
    }

    // 关闭前保存状态
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
