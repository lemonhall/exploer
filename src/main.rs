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
            
            // 特殊处理"我的电脑"，直接加载驱动器列表到右侧面板
            let is_computer = find_item_by_path(&data.root, path)
                .map(|item| item.name == "我的电脑")
                .unwrap_or(false);
            
            if is_computer {
                println!("选择了我的电脑，直接加载驱动器列表");
                // 创建驱动器的FileDetail列表
                let mut drive_details = Vec::new();
                for drive in file_system::get_drives() {
                    // 计算驱动器可用空间和总空间
                    let (avail_space, total_space) = system::get_drive_space(&drive.path);
                    let size_info = if total_space > 0 {
                        format!("{} 可用/{} 总量", 
                            format_size(avail_space),
                            format_size(total_space))
                    } else {
                        "".to_string()
                    };
                    
                    drive_details.push(models::FileDetail {
                        name: drive.name.clone(),
                        size: total_space,
                        file_type: "驱动器".to_string(),
                        modified: size_info,
                        full_path: drive.path.clone(),
                    });
                }
                
                // 转换为druid的Vector类型
                data.current_dir_files = druid::im::Vector::from(drive_details);
            } else {
                // 正常加载目录内容
                data.current_dir_files = file_system::get_directory_contents(path);
            }
            
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
    // 特殊处理"我的电脑"目录
    if item.name == "我的电脑" {
        // 打印调试信息
        println!("处理我的电脑目录: {} 子项数量: {}", item.name, item.children.len());
        
        // 强制设置为展开状态
        item.is_expanded = true;
        
        // 如果子目录不存在或为空，重新创建驱动器列表
        if item.children.is_empty() {
            println!("我的电脑子目录为空，重新获取驱动器");
            item.children = get_drives();
            
            // 强制所有驱动器为展开状态
            for drive in &mut item.children {
                println!("设置驱动器: {} 为展开状态", drive.name);
                drive.is_expanded = true;
                // 预加载驱动器下的子目录
                if drive.children.is_empty() {
                    drive.children = build_file_tree(&drive.path, 1);
                    println!("驱动器 {} 加载了 {} 个子目录", drive.name, drive.children.len());
                }
            }
        } else {
            println!("我的电脑已有 {} 个子目录", item.children.len());
            // 确保所有子驱动器为展开状态
            for drive in &mut item.children {
                println!("确认驱动器: {} 展开状态: {}", drive.name, drive.is_expanded);
                drive.is_expanded = true;
            }
        }
        return;
    }
    
    // 处理路径匹配的情况
    if item.path == *target_path {
        // 设置为展开状态
        item.is_expanded = true;
        println!("路径匹配: {} 正在展开", item.name);
        
        // 找到目标路径，加载子目录
        if item.children.is_empty() {
            item.children = build_file_tree(&item.path, 1);
            println!("路径 {} 加载了 {} 个子目录", item.path.display(), item.children.len());
        }
        return;
    }
    
    // 递归处理所有子项
    for child in &mut item.children {
        load_subdirectories(child, target_path);
    }
}

/// 格式化文件大小显示方式
fn format_size(size: u64) -> String {
    if size < 1024 {
        format!("{} B", size)
    } else if size < 1024 * 1024 {
        format!("{:.1} KB", size as f64 / 1024.0)
    } else if size < 1024 * 1024 * 1024 {
        format!("{:.1} MB", size as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.1} GB", size as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

/// 查找特定路径对应的FileItem
fn find_item_by_path<'a>(item: &'a FileItem, target_path: &PathBuf) -> Option<&'a FileItem> {
    if item.path == *target_path {
        return Some(item);
    }
    
    for child in &item.children {
        if let Some(found) = find_item_by_path(child, target_path) {
            return Some(found);
        }
    }
    
    None
}

/// 程序入口函数
fn main() {
    // 创建主窗口描述
    let main_window = WindowDesc::new(build_ui())
        .title("柠檬文件管理器")  // 更新窗口标题
        .window_size((1000.0, 600.0));  // 设置窗口大小，增大以适应双栏布局

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

    // 创建初始应用程序状态
    let initial_state = AppState {
        root,
        selected_path: Some(default_drive.clone()),
        current_dir_files: get_directory_contents(&default_drive),
    };

    // 创建应用启动器
    let launcher = AppLauncher::with_window(main_window)
        .delegate(FileExplorerDelegate);
        
    // 在启动前发送命令模拟展开操作
    let event_sink = launcher.get_external_handle();
    
    // 启用桌面文件夹的展开
    let desktop_path = initial_state.root.children[1].path.clone();
    
    // 延迟一小段时间后发送展开命令，确保应用已完全初始化
    std::thread::spawn(move || {
        // 增加延迟时间，确保应用完全初始化
        std::thread::sleep(std::time::Duration::from_millis(1000));
        
        // 展开桌面文件夹
        if let Err(e) = event_sink.submit_command(LOAD_SUBDIRECTORIES, desktop_path, Target::Auto) {
            eprintln!("发送桌面展开命令失败: {}", e);
        }
        
        // 强制展开我的电脑文件夹
        let computer_path = PathBuf::from("C:\\");
        if let Err(e) = event_sink.submit_command(LOAD_SUBDIRECTORIES, computer_path.clone(), Target::Auto) {
            eprintln!("发送我的电脑展开命令失败: {}", e);
        }
        
        // 强制选中我的电脑节点，确保其UI状态正确
        std::thread::sleep(std::time::Duration::from_millis(100));
        if let Err(e) = event_sink.submit_command(SELECT_DIRECTORY, computer_path.clone(), Target::Auto) {
            eprintln!("发送我的电脑选择命令失败: {}", e);
        }
        
        // 再次强制展开
        std::thread::sleep(std::time::Duration::from_millis(100));
        if let Err(e) = event_sink.submit_command(LOAD_SUBDIRECTORIES, computer_path, Target::Auto) {
            eprintln!("发送第二次我的电脑展开命令失败: {}", e);
        }
    });

    // 启动应用程序
    launcher
        .launch(initial_state)
        .expect("启动应用程序失败");
}
