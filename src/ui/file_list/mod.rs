mod row;
mod controllers;
mod icons;
mod item_styles;

pub use row::file_list_item;

use druid::widget::{Flex, Label, Scroll, List};
use druid::{Widget, WidgetExt};
use crate::models::AppState;
use crate::ui::constants::*;

/// 构建文件列表视图，包含表头和内容
pub fn build_file_list() -> impl Widget<AppState> {
    Flex::column()
        .with_child(build_file_list_header())
        .with_flex_child(build_file_list_content(), 1.0)
        .background(LIGHT_BACKGROUND)
}

/// 构建文件列表的表头
fn build_file_list_header() -> impl Widget<AppState> {
    // 创建表头的各个列标签
    let name_header = Label::new("名称")
        .with_text_color(SELECTED_TEXT)
        .with_text_size(14.0)
        .padding(10.0)
        .align_left();
        
    let size_header = Label::new("大小")
        .with_text_color(SELECTED_TEXT)
        .with_text_size(14.0)
        .padding(10.0)
        .align_left();
        
    let type_header = Label::new("类型")
        .with_text_color(SELECTED_TEXT)
        .with_text_size(14.0)
        .padding(10.0)
        .align_left();
        
    let modified_header = Label::new("修改时间")
        .with_text_color(SELECTED_TEXT)
        .with_text_size(14.0)
        .padding(10.0)
        .align_left();
    
    // 将各个列头组合成一个横向布局
    Flex::row()
        .with_flex_child(name_header, 0.4)
        .with_flex_child(size_header, 0.2)
        .with_flex_child(type_header, 0.2)
        .with_flex_child(modified_header, 0.2)
        .background(MID_BACKGROUND)
}

/// 构建文件列表的内容区域
fn build_file_list_content() -> impl Widget<AppState> {
    // 使用List小部件来显示文件列表
    // List小部件会为数据集合中的每项创建一个子小部件
    let list = List::new(file_list_item)
        .lens(AppState::current_dir_files);

    // 使用Scroll包装列表，以便在内容过多时可以滚动
    Scroll::new(list)
        .vertical()
        .expand()
} 