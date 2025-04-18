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
        self.children.len()
    }
    
    /// 获取指定索引的子项
    fn get_child(&self, index: usize) -> &Self {
        &self.children[index]
    }
    
    /// 允许修改子项
    fn for_child_mut(&mut self, index: usize, mut cb: impl FnMut(&mut Self, usize)) {
        let child = &mut self.children[index];
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
} 