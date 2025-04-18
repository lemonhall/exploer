// 声明模块
mod models;
mod file_system;
mod ui;
mod assets;
mod commands;
mod system;

// 导入所需的类型和函数
use druid::{AppLauncher, WindowDesc, Selector, AppDelegate, Env, Command, Target, DelegateCtx, Handled};
use models::{AppState, FileItem};
use file_system::{build_file_tree, get_directory_contents, get_drives};
use ui::build_ui;
use commands::{NAVIGATE_TO, OPEN_FILE};
use system::open_file;
use std::path::PathBuf;

// 自定义命令：选择目录
pub const SELECT_DIRECTORY: Selector<PathBuf> = Selector::new("file-explorer.select-directory");
// 自定义命令：加载子目录
pub const LOAD_SUBDIRECTORIES: Selector<PathBuf> = Selector::new("file-explorer.load-subdirectories");

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
            // 清除之前的选中状态
            clear_selection(&mut data.root);
            
            // 更新选中的目录路径
            data.selected_path = Some(path.clone());
            
            // 更新右侧面板的文件列表
            data.current_dir_files = get_directory_contents(&path);
            
            // 设置当前选中的目录项
            update_selection(&mut data.root, &path);
            
            return Handled::Yes;
        } else if let Some(path) = cmd.get(LOAD_SUBDIRECTORIES) {
            // 动态加载子目录
            load_subdirectories(&mut data.root, path);
            return Handled::Yes;
        } else if let Some(path) = cmd.get(NAVIGATE_TO) {
            // 清除之前的选中状态
            clear_selection(&mut data.root);
            
            // 更新选中的目录路径
            data.selected_path = Some(path.clone());
            
            // 更新右侧面板的文件列表
            data.current_dir_files = get_directory_contents(&path);
            
            // 设置当前选中的目录项
            update_selection(&mut data.root, &path);
            
            return Handled::Yes;
        } else if let Some(path) = cmd.get(OPEN_FILE) {
            // 使用系统默认程序打开文件
            if let Err(err) = open_file(path) {
                eprintln!("打开文件失败: {}", err);
            }
            return Handled::Yes;
        }
        Handled::No
    }
}

/// 清除所有项的选中状态
fn clear_selection(item: &mut FileItem) {
    item.is_selected = false;
    for child in &mut item.children {
        clear_selection(child);
    }
}

/// 更新选中状态
fn update_selection(item: &mut FileItem, selected_path: &PathBuf) {
    if item.path == *selected_path {
        item.is_selected = true;
        return;
    }
    
    for child in &mut item.children {
        update_selection(child, selected_path);
    }
}

/// 动态加载子目录
fn load_subdirectories(item: &mut FileItem, target_path: &PathBuf) {
    if item.path == *target_path {
        // 找到目标路径，加载子目录
        if item.children.is_empty() {
            item.children = build_file_tree(&item.path, 1);
        }
        return;
    }
    
    for child in &mut item.children {
        load_subdirectories(child, target_path);
    }
}

/// 程序入口函数
fn main() {
    // 创建主窗口描述
    let main_window = WindowDesc::new(build_ui())
        .title("文件管理器")  // 设置窗口标题
        .window_size((1000.0, 600.0));  // 设置窗口大小，增大以适应双栏布局

    // 获取当前系统的驱动器列表
    let drives = get_drives();
    
    // 确定初始选中的驱动器和目录
    let default_drive = if !drives.is_empty() {
        drives[0].path.clone()
    } else {
        // 如果没有找到驱动器，使用当前目录
        std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
    };
    
    // 创建根文件项
    let mut root = FileItem {
        name: "我的电脑".to_string(),  // 使用"我的电脑"作为根节点名称
        children: drives,  // 使用驱动器列表作为子项
        is_expanded: true,  // 默认展开根节点
        path: PathBuf::from(""),  // 根节点路径为空
        is_selected: false,
    };
    
    // 设置默认选中的驱动器
    update_selection(&mut root, &default_drive);

    // 创建初始应用程序状态
    let initial_state = AppState {
        root,
        selected_path: Some(default_drive.clone()),
        current_dir_files: get_directory_contents(&default_drive),
    };

    // 启动应用程序
    AppLauncher::with_window(main_window)
        .delegate(FileExplorerDelegate)
        .launch(initial_state)
        .expect("启动应用程序失败");
}
