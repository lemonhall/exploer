use druid::widget::{Flex, Label, Scroll, Container, List, Painter};
use druid::{Widget, WidgetExt, RenderContext};
use crate::models::{AppState, FileDetail};
use super::constants::*;
use super::utils::format_file_size;

/// 构建文件列表视图（右侧面板）
pub fn build_file_list() -> impl Widget<AppState> {
    // 创建文件列表
    let file_list = List::new(|| {
        // 为每个文件或目录项创建一行
        Flex::row()
            // 添加图标（目录或文件）
            .with_child(
                Painter::new(|ctx, data: &FileDetail, _env| {
                    // 简单的图标绘制逻辑
                    let rect = ctx.size().to_rect();
                    let is_dir = data.file_type == "目录";
                    
                    if is_dir {
                        // 绘制目录图标（简单的黄色文件夹）
                        ctx.fill(rect, &FOLDER_COLOR);
                    } else {
                        // 绘制文件图标（简单的白色纸张）
                        ctx.fill(rect, &FILE_COLOR);
                    }
                })
                .fix_size(16.0, 16.0)
                .padding((5.0, 5.0))
            )
            // 添加文件名
            .with_child(
                Label::dynamic(|item: &FileDetail, _| item.name.clone())
                .with_text_color(DARK_TEXT)
                .expand_width()
                .padding(5.0)
            )
            // 添加文件大小
            .with_child(
                Label::dynamic(|item: &FileDetail, _| {
                    if item.file_type == "目录" {
                        "".to_string()
                    } else {
                        format_file_size(item.size)
                    }
                })
                .with_text_color(DARK_TEXT)
                .padding(5.0)
                .fix_width(100.0)
            )
            // 添加文件类型
            .with_child(
                Label::dynamic(|item: &FileDetail, _| item.file_type.clone())
                .with_text_color(DARK_TEXT)
                .padding(5.0)
                .fix_width(100.0)
            )
            // 添加修改时间
            .with_child(
                Label::dynamic(|item: &FileDetail, _| item.modified.clone())
                .with_text_color(DARK_TEXT)
                .padding(5.0)
                .fix_width(150.0)
            )
    })
    .lens(AppState::current_dir_files);

    // 给列表添加标题行
    let header_row = Flex::row()
        .with_child(Label::new("").fix_size(26.0, 20.0))
        .with_child(Label::new("名称").with_text_color(DARK_TEXT).expand_width().padding(5.0))
        .with_child(Label::new("大小").with_text_color(DARK_TEXT).padding(5.0).fix_width(100.0))
        .with_child(Label::new("类型").with_text_color(DARK_TEXT).padding(5.0).fix_width(100.0))
        .with_child(Label::new("修改日期").with_text_color(DARK_TEXT).padding(5.0).fix_width(150.0))
        .background(HEADER_BACKGROUND);
    
    // 组合标题行和文件列表
    let file_view = Flex::column()
        .with_child(header_row)
        .with_flex_child(Scroll::new(file_list), 1.0);  // 直接在列表上应用滚动

    // 添加内边距并返回
    Container::new(file_view)
        .padding(10.0)
        .background(DARK_BACKGROUND) // 右侧也使用相同的深色背景
        .expand()
} 