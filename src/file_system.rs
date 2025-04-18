use std::path::Path;
use crate::models::FileItem;

/// 构建文件树
/// 
/// # 参数
/// 
/// * `path` - 起始目录路径
/// * `depth` - 递归深度限制（防止过深的遍历）
/// 
/// # 返回值
/// 
/// 返回指定路径下的文件和目录列表
pub fn build_file_tree(path: &Path, depth: usize) -> Vec<FileItem> {
    // 限制递归深度，防止无限递归
    if depth == 0 {
        return Vec::new();
    }

    // 分别存储目录和文件，以便于目录排在前面
    let mut directories = Vec::new();
    let mut files = Vec::new();
    
    // 读取目录内容
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                // 获取文件或目录的名称
                let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                
                // 跳过隐藏文件和目录（以点号开头的文件和目录，如.git）
                if name.starts_with(".") {
                    continue;
                }
                
                // 区分处理目录和文件
                if path.is_dir() {
                    // 递归遍历子目录
                    let children = build_file_tree(&path, depth - 1);
                    directories.push(FileItem { 
                        name, 
                        children,
                        is_expanded: false 
                    });
                } else {
                    // 文件没有子项
                    files.push(FileItem { 
                        name, 
                        children: Vec::new(),
                        is_expanded: false 
                    });
                }
            }
        }
    }
    
    // 先添加所有目录，再添加所有文件（使目录显示在文件前面）
    let mut items = Vec::new();
    items.append(&mut directories);
    items.append(&mut files);
    
    items
} 