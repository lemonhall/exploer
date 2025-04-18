use druid::{Data, Lens};
use druid_widget_nursery::TreeNode;

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

/// 应用程序状态结构体
#[derive(Clone, Data, Lens)]
pub struct AppState {
    /// 文件树的根节点
    pub root: FileItem,
} 