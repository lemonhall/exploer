// 声明模块
mod models;
mod file_system;
mod ui;

// 导入所需的类型和函数
use druid::{AppLauncher, WindowDesc, Selector, AppDelegate, Env, Command, Target, DelegateCtx, Handled};
use models::{AppState, FileItem};
use file_system::{build_file_tree, get_directory_contents};
use ui::build_ui;
use std::path::PathBuf;

// 自定义命令：选择目录
pub const SELECT_DIRECTORY: Selector<PathBuf> = Selector::new("file-explorer.select-directory");

/// 自定义AppDelegate实现，处理目录选择命令
struct FileExplorerDelegate;

impl AppDelegate<AppState> for FileExplorerDelegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppState,
        _env: &Env,
    ) -> Handled {
        if let Some(path) = cmd.get(SELECT_DIRECTORY) {
            data.selected_path = Some(path.clone());
            data.current_dir_files = get_directory_contents(&path);
            return Handled::Yes;
        }
        Handled::No
    }
}

/// 程序入口函数
fn main() {
    // 创建主窗口描述
    let main_window = WindowDesc::new(build_ui())
        .title("文件管理器")  // 设置窗口标题
        .window_size((1000.0, 600.0));  // 设置窗口大小，增大以适应双栏布局

    // 获取当前目录
    let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    
    // 创建根文件项
    let root = FileItem {
        name: "Root".to_string(),
        // 获取目录树，最多递归3层
        children: build_file_tree(&current_dir, 3),
        is_expanded: true,  // 默认展开根节点
        path: current_dir.clone(),
    };

    // 创建初始应用程序状态
    let initial_state = AppState {
        root,
        selected_path: Some(current_dir.clone()),
        current_dir_files: get_directory_contents(&current_dir),
    };

    // 启动应用程序
    AppLauncher::with_window(main_window)
        .delegate(FileExplorerDelegate)
        .launch(initial_state)
        .expect("启动应用程序失败");
}
