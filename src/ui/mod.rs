mod constants;
mod directory_tree;
mod file_list;
mod utils;

use druid::widget::{Container, Split};
use druid::{Widget, WidgetExt};
use crate::models::AppState;

pub use directory_tree::build_directory_tree;
pub use file_list::build_file_list;
pub use constants::*;

/// 构建应用程序的UI界面
pub fn build_ui() -> impl Widget<AppState> {
    // 创建分割视图，左侧是目录树，右侧是文件列表
    let split = Split::columns(
        build_directory_tree(),
        build_file_list()
    )
    .split_point(0.25)  // 左侧面板占25%的宽度，减少以给右侧更多空间
    .draggable(true)   // 允许调整分割位置
    .solid_bar(true);  // 使用实心分隔条

    // 使用Container包装分割视图，提供边距
    Container::new(split)
        .padding(5.0)
        .background(constants::DARK_BACKGROUND) // 整个应用使用深色背景
        .expand()
} 