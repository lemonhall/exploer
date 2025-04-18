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
use commands::{NAVIGATE_TO, OPEN_FILE, RESET_CURSOR};
use std::path::PathBuf;
use dirs;

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
        if let Some(dir_path) = cmd.get(NAVIGATE_TO) {
            // 处理导航命令
            let dir_path = PathBuf::from(dir_path);
            let contents = file_system::get_directory_contents(&dir_path);
            data.current_dir_files = contents;
            
            // 更新选中路径
            data.selected_path = Some(dir_path.clone());
            
            // 更新树的选中状态
            update_selection(&mut data.root, &dir_path);
            
            Handled::Yes
        } else if let Some(file_path) = cmd.get(OPEN_FILE) {
            // 处理打开文件命令
            match system::open_file(file_path) {
                Ok(_) => {},
                Err(e) => eprintln!("打开文件失败: {}", e),
            }
            Handled::Yes
        } else if let Some(path) = cmd.get(SELECT_DIRECTORY) {
            // 处理选择目录命令
            data.selected_path = Some(path.clone());
            data.current_dir_files = file_system::get_directory_contents(path);
            
            // 更新树的选中状态
            update_selection(&mut data.root, path);
            
            Handled::Yes
        } else if let Some(path) = cmd.get(LOAD_SUBDIRECTORIES) {
            // 处理加载子目录命令
            load_subdirectories(&mut data.root, path);
            Handled::Yes
        } else if let Some(()) = cmd.get(RESET_CURSOR) {
            // 处理重置光标命令
            Handled::Yes
        } else {
            Handled::No
        }
    }
}

/// 更新选中状态
fn update_selection(item: &mut FileItem, selected_path: &PathBuf) {
    // 先清除当前项的选中状态
    item.is_selected = false;
    
    // 如果当前项就是要选中的路径，则设置为选中
    if item.path == *selected_path {
        item.is_selected = true;
    }
    
    // 递归处理所有子项
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
        .title("柠檬文件管理器")  // 更新窗口标题
        .window_size((1000.0, 600.0));  // 设置窗口大小，增大以适应双栏布局

    // 获取当前系统的驱动器列表
    let drives = get_drives();
    
    // 获取用户主目录路径
    let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("C:\\Users"));
    
    // 获取桌面目录路径
    let desktop_dir = dirs::desktop_dir().unwrap_or_else(|| home_dir.join("Desktop"));
    
    // 确定初始选中的驱动器和目录
    let default_drive = if !drives.is_empty() {
        drives[0].path.clone()
    } else {
        // 如果没有找到驱动器，使用当前目录
        std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
    };
    
    // 创建主文件夹项
    let home_item = FileItem {
        name: "主文件夹".to_string(),
        children: build_file_tree(&home_dir, 1),
        is_expanded: false,
        path: home_dir.clone(),
        is_selected: false,
    };
    
    // 创建桌面项
    let desktop_item = FileItem {
        name: "桌面".to_string(),
        children: build_file_tree(&desktop_dir, 1),
        is_expanded: false,
        path: desktop_dir.clone(),
        is_selected: false,
    };
    
    // 创建我的电脑项（包含驱动器）
    let computer_item = FileItem {
        name: "我的电脑".to_string(),
        children: drives,
        is_expanded: true,
        path: PathBuf::from(""),
        is_selected: false,
    };
    
    // 创建根文件项
    let mut root = FileItem {
        name: "文件导航".to_string(),  // 根节点名称
        children: vec![home_item, desktop_item, computer_item],  // 添加主文件夹、桌面和我的电脑作为子项
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
