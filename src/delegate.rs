use druid::{AppDelegate, Env, Command, Target, DelegateCtx, Handled};
use std::path::PathBuf;

use crate::models::{AppState, FileItem};
use crate::file_system::{get_directory_contents, get_directory_contents_paged, build_file_tree, get_drives, get_directory_item_count};
use crate::commands::{NAVIGATE_TO, OPEN_FILE, RESET_CURSOR};
use crate::system;
use crate::{SELECT_DIRECTORY, LOAD_SUBDIRECTORIES};
use crate::utils::format_size;

/// 自定义AppDelegate实现，处理目录选择命令
pub struct FileExplorerDelegate;

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
            let contents = get_directory_contents(&dir_path);
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
                for drive in get_drives() {
                    // 计算驱动器可用空间和总空间
                    let (avail_space, total_space) = system::get_drive_space(&drive.path);
                    let size_info = if total_space > 0 {
                        format!("{} 可用/{} 总量", 
                            format_size(avail_space),
                            format_size(total_space))
                    } else {
                        "".to_string()
                    };
                    
                    drive_details.push(crate::models::FileDetail {
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
                // 正常加载目录内容，使用分页加载提高性能
                data.current_dir_files = get_directory_contents_paged(path, 0, 500);
                
                // 启动后台线程加载更多文件（如果目录中文件很多）
                let _path_clone = path.clone();
                let total_count = get_directory_item_count(path);
                
                if total_count > 500 {
                    println!("目录含有大量文件 ({}个)，使用分页加载", total_count);
                    
                    // 将额外的文件加载放到后台线程，避免阻塞UI
                    std::thread::spawn(move || {
                        // 后台加载剩余的文件
                        println!("后台加载剩余文件...");
                        
                        // 等待一段时间让UI先渲染
                        std::thread::sleep(std::time::Duration::from_millis(100));
                        
                        // 返回后台加载结果
                        // 注意：这里没有更新UI，因为我们需要一个方式在后台线程中更新UI
                        // 在实际应用中，需要添加一个命令来更新UI
                    });
                }
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
pub fn update_selection(item: &mut FileItem, selected_path: &PathBuf) {
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
pub fn load_subdirectories(item: &mut FileItem, target_path: &PathBuf) {
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

/// 查找特定路径对应的FileItem
pub fn find_item_by_path<'a>(item: &'a FileItem, target_path: &PathBuf) -> Option<&'a FileItem> {
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