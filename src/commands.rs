use druid::Selector;
use std::path::PathBuf;
use druid::im::Vector;
use crate::models::FileDetail;

/// 导航到指定路径的命令
pub const NAVIGATE_TO: Selector<PathBuf> = Selector::new("file-explorer.navigate-to");

/// 打开文件的命令
pub const OPEN_FILE: Selector<PathBuf> = Selector::new("file-explorer.open-file");

/// 重置鼠标光标的命令
pub const RESET_CURSOR: Selector<()> = Selector::new("file-explorer.reset-cursor");

/// 后台加载完成后更新文件列表的命令
pub const UPDATE_FILE_LIST: Selector<druid::im::Vector<crate::models::FileDetail>> = 
    Selector::new("file-explorer.update-file-list");

/// 导航到上一个目录（后退）
pub const NAVIGATE_BACK: Selector<()> = Selector::new("file-explorer.navigate-back");

/// 导航到下一个目录（前进）
pub const NAVIGATE_FORWARD: Selector<()> = Selector::new("file-explorer.navigate-forward");

/// 导航到上级目录
pub const NAVIGATE_UP: Selector<()> = Selector::new("file-explorer.navigate-up");

/// 刷新当前目录
pub const REFRESH_DIRECTORY: Selector<()> = Selector::new("file-explorer.refresh-directory");

/// 导航到主目录
pub const NAVIGATE_HOME: Selector<()> = Selector::new("file-explorer.navigate-home"); 