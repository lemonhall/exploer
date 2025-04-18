// 声明模块
mod models;
mod file_system;
mod ui;

// 导入所需的类型和函数
use druid::{AppLauncher, WindowDesc};
use models::{AppState, FileItem};
use file_system::build_file_tree;
use ui::build_ui;

/// 程序入口函数
fn main() {
    // 创建主窗口描述
    let main_window = WindowDesc::new(build_ui())
        .title("文件管理器")  // 设置窗口标题
        .window_size((800.0, 600.0));  // 设置窗口大小

    // 创建根文件项
    let root = FileItem {
        name: "Root".to_string(),
        // 获取当前目录的文件树，最多递归3层
        children: build_file_tree(std::env::current_dir().unwrap().as_path(), 3),
        is_expanded: true,  // 默认展开根节点
    };

    // 创建初始应用程序状态
    let initial_state = AppState {
        root,
    };

    // 启动应用程序
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("启动应用程序失败");
}
