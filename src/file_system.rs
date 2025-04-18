use std::path::Path;
use std::time::SystemTime;
use druid::im::Vector;
use crate::models::{FileItem, FileDetail};

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
///
/// # 参数
///
/// * `path` - 目录路径
///
/// # 返回值
///
/// 返回指定目录下的文件和目录详情列表
pub fn get_directory_contents(path: &Path) -> Vector<FileDetail> {
    let mut result = Vec::new();
    
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                
                // 跳过隐藏文件和目录
                if name.starts_with(".") {
                    continue;
                }
                
                // 获取文件大小
                let size = if path.is_file() {
                    std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0)
                } else {
                    0 // 目录大小显示为0
                };
                
                // 获取文件类型
                let file_type = if path.is_dir() {
                    "目录".to_string()
                } else {
                    match path.extension() {
                        Some(ext) => format!("{} 文件", ext.to_string_lossy()),
                        None => "文件".to_string()
                    }
                };
                
                // 获取修改时间
                let modified = std::fs::metadata(&path)
                    .and_then(|m| m.modified())
                    .map(|time| {
                        // 将系统时间转换为简单格式
                        // 实际应用中应该使用chrono等库格式化时间
                        format!("{:?}", time.duration_since(SystemTime::UNIX_EPOCH).unwrap_or_default().as_secs())
                    })
                    .unwrap_or_else(|_| "未知".to_string());
                
                result.push(FileDetail {
                    name,
                    size,
                    file_type,
                    modified,
                });
            }
        }
    }
    
    // 按照相同的逻辑排序：目录在前，文件在后
    result.sort_by(|a, b| {
        let a_is_dir = a.file_type == "目录";
        let b_is_dir = b.file_type == "目录";
        
        if a_is_dir == b_is_dir {
            a.name.cmp(&b.name)
        } else {
            b_is_dir.cmp(&a_is_dir)
        }
    });
    
    // 转换为druid的Vector类型
    Vector::from(result)
} 