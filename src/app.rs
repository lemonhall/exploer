use std::path::PathBuf;
use druid::{AppLauncher, WindowDesc, Target};
use std::thread;
use std::time::Duration;

use crate::models::{AppState, FileItem};
use crate::file_system::{get_directory_contents, get_drives, build_file_tree};
use crate::ui::build_ui;
use crate::delegate::{FileExplorerDelegate, update_selection};
use crate::{LOAD_SUBDIRECTORIES, SELECT_DIRECTORY};

/// 初始化应用程序并运行
pub fn run_app() {
    // 创建主窗口描述
    let main_window = WindowDesc::new(build_ui())
        .title("柠檬文件管理器")
        .window_size((1000.0, 600.0));

    // 创建初始状态
    let initial_state = create_initial_state();

    // 创建应用启动器
    let launcher = AppLauncher::with_window(main_window)
        .delegate(FileExplorerDelegate);
        
    // 启动初始化线程
    initialize_folders(launcher.get_external_handle(), &initial_state);

    // 启动应用程序
    launcher
        .launch(initial_state)
        .expect("启动应用程序失败");
}

/// 创建应用程序的初始状态
fn create_initial_state() -> AppState {
    // 获取当前系统的驱动器列表
    let mut drives = get_drives();
    println!("初始驱动器数量: {}", drives.len());
    
    // 确保所有驱动器为展开状态并预加载子目录
    for drive in &mut drives {
        println!("设置驱动器为展开状态: {}", drive.name);
        drive.is_expanded = true;
        if drive.children.is_empty() {
            drive.children = build_file_tree(&drive.path, 1);
            println!("预加载驱动器子目录: {} 数量: {}", drive.name, drive.children.len());
        }
    }
    
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
        is_expanded: true, // 默认展开
        path: desktop_dir.clone(),
        is_selected: false,
    };
    
    // 创建我的电脑项（包含驱动器）
    let computer_item = FileItem {
        name: "我的电脑".to_string(),
        children: drives,
        is_expanded: true,  // 设置为默认展开状态
        path: PathBuf::from("C:\\"), // 使用有效路径而不是空字符串
        is_selected: false,
    };
    
    // 创建根文件项
    let mut root = FileItem {
        name: "文件导航".to_string(),  // 根节点名称
        children: vec![home_item, desktop_item, computer_item],  // 添加主文件夹、桌面和我的电脑作为子项
        is_expanded: true,  // 默认展开根节点
        path: PathBuf::from("ROOT"), // 使用特殊标识而不是空字符串
        is_selected: false,
    };
    
    // 设置默认选中的驱动器
    update_selection(&mut root, &default_drive);

    // 创建初始导航历史记录
    let navigation_history = vec![default_drive.clone()];
    
    // 创建初始应用程序状态
    AppState {
        root,
        selected_path: Some(default_drive.clone()),
        current_dir_files: get_directory_contents(&default_drive),
        navigation_history,
        history_position: 0,
    }
}

/// 初始化应用程序文件夹
fn initialize_folders(event_sink: druid::ExtEventSink, initial_state: &AppState) {
    // 启用桌面文件夹的展开
    let desktop_path = initial_state.root.children[1].path.clone();
    
    // 获取我的电脑路径
    let computer_path = PathBuf::from("C:\\");
    
    // 延迟一小段时间后发送展开命令，确保应用已完全初始化
    thread::spawn(move || {
        // 增加延迟时间，确保应用完全初始化
        thread::sleep(Duration::from_millis(1000));
        
        // 展开桌面文件夹
        if let Err(e) = event_sink.submit_command(LOAD_SUBDIRECTORIES, desktop_path, Target::Auto) {
            eprintln!("发送桌面展开命令失败: {}", e);
        }
        
        // 强制展开我的电脑文件夹
        if let Err(e) = event_sink.submit_command(LOAD_SUBDIRECTORIES, computer_path.clone(), Target::Auto) {
            eprintln!("发送我的电脑展开命令失败: {}", e);
        }
        
        // 强制选中我的电脑节点，确保其UI状态正确
        thread::sleep(Duration::from_millis(100));
        if let Err(e) = event_sink.submit_command(SELECT_DIRECTORY, computer_path.clone(), Target::Auto) {
            eprintln!("发送我的电脑选择命令失败: {}", e);
        }
        
        // 再次强制展开
        thread::sleep(Duration::from_millis(100));
        if let Err(e) = event_sink.submit_command(LOAD_SUBDIRECTORIES, computer_path, Target::Auto) {
            eprintln!("发送第二次我的电脑展开命令失败: {}", e);
        }
    });
} 