use std::path::Path;
use std::path::PathBuf;
use std::time::SystemTime;
use druid::im::Vector;
use crate::models::{FileItem, FileDetail};

/// 获取系统上所有可用的驱动器（盘符）
/// 在Windows上返回所有可用的盘符（如C:, D:等）
/// 在其他系统上返回根目录 "/"
pub fn get_drives() -> Vec<FileItem> {
    let mut drives = Vec::new();
    
    #[cfg(target_os = "windows")]
    {
        // 在Windows上，尝试检查所有可能的盘符（A-Z）
        for c in b'A'..=b'Z' {
            let drive = format!("{}:\\", char::from(c));
            let path = PathBuf::from(&drive);
            
            // 检查该盘符是否存在
            if path.exists() {
                // 预先加载每个驱动器的子目录
                let children = build_file_tree(&path, 1);
                
                drives.push(FileItem {
                    name: drive.clone(),
                    children, // 直接加载子目录，不再是空的
                    is_expanded: true, // 默认展开
                    path,
                    is_selected: false,
                });
            }
        }
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        // 在非Windows系统上，只添加根目录，并预加载子目录
        let root_path = PathBuf::from("/");
        let children = build_file_tree(&root_path, 1);
        
        drives.push(FileItem {
            name: "/".to_string(),
            children,
            is_expanded: true,
            path: root_path,
            is_selected: false,
        });
    }
    
    drives
}

/// 构建目录树（只包含目录，不包含文件）
/// 
/// # 参数
/// 
/// * `path` - 起始目录路径
/// * `depth` - 递归深度限制（防止过深的遍历）
/// 
/// # 返回值
/// 
/// 返回指定路径下的目录列表（不包含文件）
pub fn build_file_tree(path: &Path, depth: usize) -> Vec<FileItem> {
    // 限制递归深度，防止无限递归
    if depth == 0 {
        return Vec::new();
    }

    let mut directories = Vec::new();
    
    // 读取目录内容
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path_buf = entry.path();
                
                // 只处理目录，跳过文件
                if !path_buf.is_dir() {
                    continue;
                }
                
                // 获取目录名称
                let name = path_buf.file_name().unwrap_or_default().to_string_lossy().to_string();
                
                // 跳过隐藏目录（以点号开头的目录，如.git）
                if name.starts_with(".") {
                    continue;
                }
                
                // 递归遍历子目录
                let children = build_file_tree(&path_buf, depth - 1);
                directories.push(FileItem { 
                    name, 
                    children,
                    is_expanded: false,
                    path: path_buf,
                    is_selected: false,
                });
            }
        }
    }
    
    // 按目录名称排序
    directories.sort_by(|a, b| a.name.cmp(&b.name));
    
    directories
}

/// 获取指定目录下的文件详情列表（包含子目录和文件）
/// 默认一次加载所有文件
pub fn get_directory_contents(path: &Path) -> Vector<FileDetail> {
    get_directory_contents_paged(path, 0, 1000) // 默认加载前1000个条目
}

/// 获取指定目录下的文件详情列表（包含子目录和文件），支持分页加载
///
/// # 参数
///
/// * `path` - 目录路径
/// * `offset` - 开始加载的偏移量
/// * `limit` - 最大加载数量
///
/// # 返回值
///
/// 返回指定目录下的文件和目录详情列表
pub fn get_directory_contents_paged(path: &Path, offset: usize, limit: usize) -> Vector<FileDetail> {
    let mut result = Vec::new();
    let mut count = 0;
    let _skipped = 0; // 添加下划线前缀表示未使用的变量
    
    // 如果目录不存在，直接返回空列表
    if !path.exists() || !path.is_dir() {
        println!("路径不存在或不是目录: {:?}", path);
        return Vector::new();
    }
    
    // 先收集所有目录项
    if let Ok(entries) = std::fs::read_dir(path) {
        // 转换为Vec以便排序和分页
        let mut entry_vec: Vec<_> = entries.filter_map(Result::ok).collect();
        
        // 按照名称排序，同时把目录放在前面
        entry_vec.sort_by(|a, b| {
            let a_is_dir = a.path().is_dir();
            let b_is_dir = b.path().is_dir();
            
            if a_is_dir == b_is_dir {
                let a_name = a.file_name().to_string_lossy().to_string();
                let b_name = b.file_name().to_string_lossy().to_string();
                a_name.cmp(&b_name)
            } else {
                b_is_dir.cmp(&a_is_dir)
            }
        });
        
        // 应用分页逻辑
        for entry in entry_vec.iter().skip(offset).take(limit) {
            let entry_path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();
            
            // 跳过隐藏文件和目录
            if name.starts_with(".") {
                continue;
            }
            
            // 获取文件大小（仅对文件）
            let size = if entry_path.is_file() {
                std::fs::metadata(&entry_path).map(|m| m.len()).unwrap_or(0)
            } else {
                0 // 目录大小显示为0
            };
            
            // 获取文件类型
            let file_type = if entry_path.is_dir() {
                "目录".to_string()
            } else {
                match entry_path.extension() {
                    Some(ext) => format!("{} 文件", ext.to_string_lossy()),
                    None => "文件".to_string()
                }
            };
            
            // 简化获取修改时间的逻辑以提高性能
            let modified = std::fs::metadata(&entry_path)
                .and_then(|m| m.modified())
                .map(|time| {
                    let system_time = std::time::SystemTime::now();
                    let duration = system_time.duration_since(time).unwrap_or_default();
                    
                    if duration.as_secs() < 60 {
                        "刚刚".to_string()
                    } else if duration.as_secs() < 3600 {
                        format!("{} 分钟前", duration.as_secs() / 60)
                    } else if duration.as_secs() < 86400 {
                        format!("{} 小时前", duration.as_secs() / 3600)
                    } else {
                        // 简单格式化为 年-月-日
                        let secs = time.duration_since(SystemTime::UNIX_EPOCH).unwrap_or_default().as_secs();
                        let days = secs / 86400;
                        let years = 1970 + (days / 365);
                        let months = (days % 365) / 30 + 1;
                        let day = (days % 365) % 30 + 1;
                        format!("{}-{:02}-{:02}", years, months, day)
                    }
                })
                .unwrap_or_else(|_| "未知".to_string());
            
            // 保存完整路径以便导航
            let full_path = entry_path.clone();
            
            result.push(FileDetail {
                name,
                size,
                file_type,
                modified,
                full_path,
            });
            
            count += 1;
            if count >= limit {
                break;
            }
        }
    }
    
    // 打印分页加载统计信息
    println!("已加载目录 {:?} 中的 {} 个条目 (跳过 {})", path, count, offset);
    
    // 转换为druid的Vector类型
    Vector::from(result)
}

/// 获取目录中的文件和目录总数
pub fn get_directory_item_count(path: &Path) -> usize {
    if let Ok(entries) = std::fs::read_dir(path) {
        // 过滤掉隐藏文件
        entries.filter(|entry| {
            if let Ok(entry) = entry {
                let name = entry.file_name().to_string_lossy().to_string();
                !name.starts_with(".")
            } else {
                false
            }
        }).count()
    } else {
        0
    }
} 