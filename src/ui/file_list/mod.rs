mod controllers;
mod row;

use druid::{widget::{Flex, Label, List, Scroll}, Widget, WidgetExt};
use crate::models::AppState;
use crate::ui::constants::*;

use self::row::file_list_item;

/// 构建文件列表视图
pub fn build_file_list() -> impl Widget<AppState> {
    let header = build_file_list_header();
    let content = build_file_list_content();

    Flex::column()
        .with_child(header)
        .with_flex_child(content, 1.0)
        .background(LIGHT_BACKGROUND)
}

/// 构建文件列表头部（列名）
fn build_file_list_header() -> impl Widget<AppState> {
    let name_label = Label::new("名称")
        .with_text_size(14.0)
        .with_font(FONT_BOLD)
        .align_left();

    let size_label = Label::new("大小")
        .with_text_size(14.0)
        .with_font(FONT_BOLD)
        .align_left();

    let type_label = Label::new("类型")
        .with_text_size(14.0)
        .with_font(FONT_BOLD)
        .align_left();

    let modified_label = Label::new("修改时间")
        .with_text_size(14.0)
        .with_font(FONT_BOLD)
        .align_left();

    Flex::row()
        .with_flex_child(name_label, 0.4)
        .with_flex_child(size_label, 0.2)
        .with_flex_child(type_label, 0.2)
        .with_flex_child(modified_label, 0.2)
        .padding(10.0)
        .background(MID_BACKGROUND)
}

/// 构建文件列表内容
fn build_file_list_content() -> impl Widget<AppState> {
    // 使用lens从AppState提取当前目录内容
    let file_list = List::new(|| file_list_item())
        .lens(AppState::current_dir_files);

    // 使用Scroll包装文件列表，允许水平和垂直滚动
    Scroll::new(file_list)
        .horizontal()
        .vertical()
} 