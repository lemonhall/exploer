use druid::{AppDelegate, Env, Command, Target, DelegateCtx, Handled};
use std::path::PathBuf;

use crate::models::{AppState, FileItem};
use crate::file_system::{get_directory_contents, get_directory_contents_paged, build_file_tree, 
                        get_drives, get_directory_item_count, preload_directory, invalidate_cache};
use crate::commands::*;
use crate::system;
use crate::{SELECT_DIRECTORY, LOAD_SUBDIRECTORIES};
use crate::utils::format_size;

/// 自定义AppDelegate实现，处理目录选择命令
pub struct FileExplorerDelegate;

impl AppDelegate<AppState> for FileExplorerDelegate {
    fn command(
        &mut self,
        ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppState,
        _env: &Env,
    ) -> Handled {
        if let Some(dir_path) = cmd.get(NAVIGATE_TO) {
            // 处理导航命令
            let dir_path = PathBuf::from(dir_path);
            
            // 加载当前目录内容
            let contents = get_directory_contents(&dir_path);
            data.current_dir_files = contents;
            
            // 异步预加载父目录和兄弟目录
            if let Some(parent) = dir_path.parent() {
                // 预加载父目录（便于快速向上导航）
                preload_directory(parent);
                
                // 获取当前目录名
                if let Some(current_dir_name) = dir_path.file_name() {
                    // 尝试预加载兄弟目录
                    if let Ok(entries) = std::fs::read_dir(parent) {
                        for entry in entries.filter_map(Result::ok) {
                            let path = entry.path();
                            
                            // 只处理目录
                            if path.is_dir() {
                                let name = entry.file_name();
                                // 不预加载当前目录，但预加载兄弟目录
                                if name != current_dir_name {
                                    preload_directory(&path);
                                }
                            }
                        }
                    }
                }
            }
            
            // 更新选中路径
            data.selected_path = Some(dir_path.clone());
            
            // 更新树的选中状态
            update_selection(&mut data.root, &dir_path);
            
            Handled::Yes
        } else if let Some(path) = cmd.get(OPEN_FILE) {
            // 处理打开文件命令
            let path = path.as_path();
            
            // 使用系统默认程序打开文件
            if let Err(e) = system::open_file(path) {
                eprintln!("打开文件失败: {}", e);
            }
            
            Handled::Yes
        } else if let Some(path) = cmd.get(SELECT_DIRECTORY) {
            // 处理目录选择命令
            let path = path.as_path();
            
            // 如果选择的是一个有效目录
            if path.exists() && path.is_dir() {
                // 将目录内容加载到右侧面板
                data.current_dir_files = get_directory_contents(path);
                
                // 更新当前选中的路径
                data.add_to_history(path.to_path_buf());
                data.selected_path = Some(path.to_path_buf());
                
                // 如果是大目录，则仅加载部分内容，其余在后台加载
                let total_count = get_directory_item_count(path);
                
                if total_count > 30 {
                    println!("目录含有大量文件 ({}个)，使用分页加载，初始加载30个", total_count);
                    
                    // 创建一个线程安全的上下文引用，供后台线程使用
                    let event_sink = ctx.get_external_handle();
                    let path_clone = path.to_path_buf();
                    
                    // 将额外的文件加载放到后台线程，避免阻塞UI
                    std::thread::spawn(move || {
                        // 后台加载剩余的文件
                        println!("后台加载更多文件...");
                        
                        // 等待一段时间让UI先渲染
                        std::thread::sleep(std::time::Duration::from_millis(50));
                        
                        // 加载更多文件 - 使用较大数量加载所有文件
                        let more_files = get_directory_contents_paged(&path_clone, 0, 10000);
                        println!("后台加载完成，总共加载 {} 个文件", more_files.len());
                        
                        // 发送命令更新UI
                        if let Err(e) = event_sink.submit_command(UPDATE_FILE_LIST, more_files, Target::Auto) {
                            eprintln!("更新文件列表失败: {:?}", e);
                        }
                    });
                }
            } else {
                // 正常加载目录内容，使用分页加载提高性能
                data.current_dir_files = get_directory_contents_paged(path, 0, 30);
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
        } else if let Some(files) = cmd.get(UPDATE_FILE_LIST) {
            // 处理更新文件列表命令
            println!("收到后台加载的文件列表，更新UI，文件数量: {}", files.len());
            data.current_dir_files = files.clone();
            Handled::Yes
        } else if let Some(()) = cmd.get(NAVIGATE_UP) {
            // 处理上级目录导航命令
            if let Some(current_path) = data.selected_path.clone() {
                if let Some(parent) = current_path.parent() {
                    // 创建路径的可拥有拷贝
                    let parent_path = parent.to_path_buf();
                    
                    // 从缓存获取目录内容
                    let directory_contents = get_directory_contents(parent);
                    
                    // 添加到历史记录并更新当前路径
                    data.add_to_history(parent_path.clone());
                    data.selected_path = Some(parent_path.clone());
                    
                    // 更新UI
                    data.current_dir_files = directory_contents;
                    
                    // 更新树的选中状态
                    update_selection(&mut data.root, &parent_path);
                }
            }
            Handled::Yes
        } else if let Some(()) = cmd.get(NAVIGATE_BACK) {
            // 处理后退命令
            if let Some(prev_path) = data.navigate_back() {
                // 获取历史记录中的上一个路径内容
                let contents = get_directory_contents(&prev_path);
                
                // 更新UI
                data.current_dir_files = contents;
                data.selected_path = Some(prev_path.clone());
                
                // 更新树的选中状态
                update_selection(&mut data.root, &prev_path);
            }
            Handled::Yes
        } else if let Some(()) = cmd.get(NAVIGATE_FORWARD) {
            // 处理前进命令
            if let Some(next_path) = data.navigate_forward() {
                // 获取历史记录中的下一个路径内容
                let contents = get_directory_contents(&next_path);
                
                // 更新UI
                data.current_dir_files = contents;
                data.selected_path = Some(next_path.clone());
                
                // 更新树的选中状态
                update_selection(&mut data.root, &next_path);
            }
            Handled::Yes
        } else if let Some(()) = cmd.get(REFRESH_DIRECTORY) {
            // 处理刷新目录命令
            if let Some(current_path) = data.selected_path.clone() {
                // 清除目录缓存
                invalidate_cache(&current_path);
                
                // 重新加载目录内容
                let contents = get_directory_contents(&current_path);
                data.current_dir_files = contents;
            }
            Handled::Yes
        } else if let Some(()) = cmd.get(NAVIGATE_HOME) {
            // 处理导航到主目录命令
            if let Some(home_dir) = dirs::home_dir() {
                // 获取主目录内容
                let contents = get_directory_contents(&home_dir);
                
                // 更新UI
                data.current_dir_files = contents;
                data.add_to_history(home_dir.clone());
                data.selected_path = Some(home_dir.clone());
                
                // 更新树的选中状态
                update_selection(&mut data.root, &home_dir);
            }
            Handled::Yes
        } else {
            Handled::No
        }
    }
}

/// 递归更新树的选中状态，确保选中的路径在树中高亮显示
pub fn update_selection(item: &mut FileItem, selected_path: &std::path::Path) {
    // 清除当前选中状态
    item.is_selected = false;
    
    // 检查当前项是否匹配
    if item.path == selected_path {
        item.is_selected = true;
        return;
    }
    
    // 检查选中路径是否是当前项的子孙路径
    if selected_path.starts_with(&item.path) {
        // 确保当前目录处于展开状态
        item.is_expanded = true;
        
        // 递归检查子目录
        for child in &mut item.children {
            update_selection(child, selected_path);
        }
    }
}

/// 递归加载子目录
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