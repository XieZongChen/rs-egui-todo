use rs_egui_todo::TodoList;

fn main() {
    // 创建一个默认的 NativeOptions 实例，用于配置应用选项
    let options = eframe::NativeOptions::default();

    // 调用 run_native 函数启动原生应用，返回值被忽略
    let _ = eframe::run_native(
        "X Todo List", // 应用程序的标题，显示在窗口标题栏
        options,       // 传入配置选项
        // 创建一个闭包，用于生成应用实例，_cc 是上下文参数
        Box::new(|_cc| {
            Ok(Box::new(TodoList::new(_cc))) // 返回一个包含新 TodoList 实例的 Ok 结果
        }),
    );
}
