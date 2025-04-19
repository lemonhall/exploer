use druid::{Data, Lens, im::Vector};
use druid_widget_nursery::TreeNode;
use std::path::PathBuf;

/// 文件项结构体，表示文件系统中的一个文件或目录
#[derive(Clone, Data, Lens, Debug, PartialEq)]
pub struct FileItem {
    /// 文件或目录的名称
    pub name: String,
    /// 当前节点是否展开（仅对目录有效）
    pub is_expanded: bool,
    /// 子项列表（对于文件，此列表为空）
    #[data(same_fn = "PartialEq::eq")]
    pub children: Vec<FileItem>,
    /// 文件路径
    #[data(same_fn = "PartialEq::eq")]
    pub path: PathBuf,
    /// 当前项是否被选中
    pub is_selected: bool,
}

/// 实现TreeNode特性，使FileItem可以在Tree控件中使用
impl TreeNode for FileItem {
    /// 返回子项数量
    fn children_count(&self) -> usize {
        // 如果是我的电脑或驱动器，总是返回子项数量
        if self.name == "我的电脑" || self.name.ends_with(":\\") {
            println!("强制显示 {} 的子项: {}", self.name, self.children.len());
            return self.children.len();
        }
        
        // 对其他项目，正常处理展开/折叠
        if self.is_expanded {
            self.children.len()
        } else {
            0
        }
    }
    
    /// 获取指定索引的子项
    fn get_child(&self, index: usize) -> &Self {
        let child = &self.children[index];
        if self.name == "我的电脑" {
            println!("获取我的电脑子项: {} (索引{}), 展开状态: {}", child.name, index, child.is_expanded);
        }
        child
    }
    
    /// 允许修改子项
    fn for_child_mut(&mut self, index: usize, mut cb: impl FnMut(&mut Self, usize)) {
        let child = &mut self.children[index];
        if self.name == "我的电脑" {
            println!("修改我的电脑子项: {} (索引{}), 展开状态: {}", child.name, index, child.is_expanded);
        }
        cb(child, index);
    }
}

/// 文件详情，用于在右侧面板显示
#[derive(Clone, Data, Lens, Debug, PartialEq)]
pub struct FileDetail {
    /// 文件名
    pub name: String,
    /// 文件大小（对目录为0）
    pub size: u64,
    /// 文件类型（文件/目录）
    pub file_type: String,
    /// 修改时间
    #[data(same_fn = "PartialEq::eq")]
    pub modified: String,
    /// 文件的完整路径
    #[data(same_fn = "PartialEq::eq")]
    pub full_path: PathBuf,
}

/// 应用程序状态结构体
#[derive(Clone, Data, Lens)]
pub struct AppState {
    /// 文件树的根节点
    pub root: FileItem,
    /// 当前选中的目录路径
    #[data(same_fn = "PartialEq::eq")]
    pub selected_path: Option<PathBuf>,
    /// 当前目录下的文件列表（用于右侧面板显示）
    pub current_dir_files: Vector<FileDetail>,
    /// 导航历史记录（已访问的路径）
    #[data(same_fn = "PartialEq::eq")]
    pub navigation_history: Vec<PathBuf>,
    /// 当前在历史记录中的位置
    pub history_position: usize,
}

impl AppState {
    /// 添加路径到导航历史记录
    pub fn add_to_history(&mut self, path: PathBuf) {
        // 检查是否与当前路径相同，避免重复添加
        if let Some(current) = &self.selected_path {
            if current == &path {
                return;
            }
        }
        
        // 如果当前不在历史记录末尾，截断后面的记录
        if self.history_position < self.navigation_history.len() {
            self.navigation_history.truncate(self.history_position + 1);
        }
        
        // 添加新路径到历史记录
        self.navigation_history.push(path);
        // 更新位置指向新添加的记录
        self.history_position = self.navigation_history.len() - 1;
    }
    
    /// 导航到历史记录中的上一个路径
    pub fn navigate_back(&mut self) -> Option<PathBuf> {
        if self.history_position > 0 {
            self.history_position -= 1;
            Some(self.navigation_history[self.history_position].clone())
        } else {
            None
        }
    }
    
    /// 导航到历史记录中的下一个路径
    pub fn navigate_forward(&mut self) -> Option<PathBuf> {
        if self.history_position < self.navigation_history.len() - 1 {
            self.history_position += 1;
            Some(self.navigation_history[self.history_position].clone())
        } else {
            None
        }
    }
    
    /// 检查是否可以后退
    pub fn can_navigate_back(&self) -> bool {
        self.history_position > 0
    }
    
    /// 检查是否可以前进
    pub fn can_navigate_forward(&self) -> bool {
        self.history_position < self.navigation_history.len() - 1
    }
} 