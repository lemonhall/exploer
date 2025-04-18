use druid::Selector;
use std::path::PathBuf;

/// 导航到指定目录的命令
pub const NAVIGATE_TO: Selector<PathBuf> = Selector::new("file-explorer.navigate-to"); 