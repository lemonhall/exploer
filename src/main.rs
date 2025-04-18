// 声明模块
mod models;
mod file_system;
mod ui;
mod assets;
mod commands;
mod system;
mod delegate;
mod utils;
mod app;

// 导入所需的类型和函数
use druid::Selector;
use std::path::PathBuf;

// 自定义命令：选择目录
pub const SELECT_DIRECTORY: Selector<PathBuf> = Selector::new("file-explorer.select-directory");
// 自定义命令：加载子目录
pub const LOAD_SUBDIRECTORIES: Selector<PathBuf> = Selector::new("file-explorer.load-subdirectories");

/// 程序入口函数
fn main() {
    app::run_app();
}
