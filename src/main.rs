use std::sync::Arc;

use egui::FontData;
use rs_egui_todo::TodoList;

fn load_fonts(ctx: &egui::Context) {
    // 定义一个默认的字体定义类型
    let mut fonts = egui::FontDefinitions::default();
    /*
     * 代码中的 fonts.font_data 实际上是一个 BTreeMap 类型的数据容器，
     * 也就是以 BTree 数据结构存储的键值对，使用方式来说类似于其它编程语言中的字典。
     * 所以在插入字体时，以 "my_font" 为键，后续操作中也可以用 "my_font" 这个键来检索新插入的这个字体
     */
    // 默认的字体中插入一个新的字体，新字体的 key 为 my_font
    fonts.font_data.insert(
        "my_font".to_owned(),
        // include_bytes!("xxxxx.ttf") 是在静态编译环境下，加入指定字体文件的代码。也就是说编译好的程序不会再去动态地读取字体文件，而是在编译期就已经将字体文件静态读入
        Arc::new(FontData::from_static(include_bytes!(
            "font/LXGWWenKaiMonoGB-Light.ttf"
        ))),
    );
    // 将导入的字体加入到比例字体系族的第一个位置
    fonts
        .families
        .get_mut(&egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, "my_font".to_owned());
    // 将导入的字体加入到等宽字体系族的最后一个位置
    fonts
        .families
        .get_mut(&egui::FontFamily::Monospace)
        .unwrap()
        .push("my_font".to_owned());
    // 将字体设置到上下文中
    ctx.set_fonts(fonts);
}

fn main() {
    // 创建一个默认的 NativeOptions 实例，用于配置应用选项
    let options = eframe::NativeOptions::default();

    // 调用 run_native 函数启动原生应用，返回值被忽略
    let _ = eframe::run_native(
        "X Todo List", // 应用程序的标题，显示在窗口标题栏
        options,       // 传入配置选项
        // 创建一个闭包，用于生成应用实例，_cc 是上下文参数
        Box::new(|_cc| {
            load_fonts(&_cc.egui_ctx);
            Ok(Box::new(TodoList::new(_cc))) // 返回一个包含新 TodoList 实例的 Ok 结果
        }),
    );
}
