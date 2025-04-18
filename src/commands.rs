use druid::Selector;
use std::path::PathBuf;

/// 导航到指定目录的命令（使用字符串路径）
pub const NAVIGATE_TO: Selector<String> = Selector::new("file-explorer.navigate-to");

/// 使用默认程序打开文件的命令
pub const OPEN_FILE: Selector<PathBuf> = Selector::new("file-explorer.open-file");

/// 重置鼠标光标的命令
pub const RESET_CURSOR: Selector<()> = Selector::new("file-explorer.reset-cursor"); 