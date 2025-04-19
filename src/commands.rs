use druid::Selector;
use std::path::PathBuf;
use druid::im::Vector;
use crate::models::FileDetail;

/// 导航到指定目录的命令（使用字符串路径）
pub const NAVIGATE_TO: Selector<String> = Selector::new("app.navigate-to");

/// 使用默认程序打开文件的命令
pub const OPEN_FILE: Selector<PathBuf> = Selector::new("app.open-file");

/// 重置鼠标光标的命令
pub const RESET_CURSOR: Selector<()> = Selector::new("app.reset-cursor");

/// 新增命令：更新文件列表，用于后台加载完成后更新UI
pub const UPDATE_FILE_LIST: Selector<Vector<FileDetail>> = Selector::new("app.update-file-list"); 