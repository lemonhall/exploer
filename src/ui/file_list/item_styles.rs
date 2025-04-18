use druid::{
    widget::{Flex, Label},
    Widget, WidgetExt, Color
};
use std::boxed::Box;
use crate::models::FileDetail;
use super::icons::{create_folder_icon, create_file_icon};

/// 创建一个行视图，包含名称、大小、类型和时间信息
pub fn create_file_row(color: Color, icon_color: Color, is_dir: bool) -> Box<dyn Widget<FileDetail>> {
    // 创建图标
    let icon = if is_dir {
        create_folder_icon(icon_color)
    } else {
        create_file_icon(icon_color)
    };
    
    // 名称列
    let name_label = Label::dynamic(|data: &FileDetail, _| data.name.clone())
        .with_text_size(14.0)
        .with_text_color(color)
        .align_left();
    
    // 名称行布局（图标+文本）
    let name_row = Flex::row()
        .with_child(icon)
        .with_spacer(5.0) // 图标与文本之间的间距
        .with_flex_child(name_label, 1.0);

    // 大小列 - 格式化显示
    let size_label = Label::dynamic(|data: &FileDetail, _| {
        format_file_size(data)
    })
    .with_text_size(14.0)
    .with_text_color(color)
    .align_left();

    // 类型列
    let type_label = Label::dynamic(|data: &FileDetail, _| data.file_type.clone())
        .with_text_size(14.0)
        .with_text_color(color)
        .align_left();

    // 修改时间列
    let modified_label = Label::dynamic(|data: &FileDetail, _| data.modified.clone())
        .with_text_size(14.0)
        .with_text_color(color)
        .align_left();

    let row = Flex::row()
        .with_flex_child(name_row, 0.4)
        .with_flex_child(size_label, 0.2)
        .with_flex_child(type_label, 0.2)
        .with_flex_child(modified_label, 0.2)
        .padding(10.0);
    
    Box::new(row)
}

/// 格式化文件大小显示方式
fn format_file_size(data: &FileDetail) -> String {
    if data.file_type == "目录" {
        "".to_string()
    } else if data.size < 1024 {
        format!("{} B", data.size)
    } else if data.size < 1024 * 1024 {
        format!("{:.1} KB", data.size as f64 / 1024.0)
    } else if data.size < 1024 * 1024 * 1024 {
        format!("{:.1} MB", data.size as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.1} GB", data.size as f64 / (1024.0 * 1024.0 * 1024.0))
    }
} 