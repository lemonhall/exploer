use druid::widget::{Flex, Label, Scroll, Container, List, Painter, SizedBox};
use druid::{Widget, WidgetExt, RenderContext};
use crate::models::{AppState, FileDetail};
use super::constants::*;
use super::utils::format_file_size;

/// 构建文件列表视图（右侧面板）
pub fn build_file_list() -> impl Widget<AppState> {
    // 定义各列的宽度常量，确保表头和内容使用相同的宽度
    const ICON_WIDTH: f64 = 30.0;
    const NAME_WIDTH: f64 = 200.0;
    const SIZE_WIDTH: f64 = 80.0;
    const TYPE_WIDTH: f64 = 100.0;
    const DATE_WIDTH: f64 = 120.0;
    
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
                .center()
                .fix_width(ICON_WIDTH)
            )
            // 添加文件名
            .with_child(
                Label::dynamic(|item: &FileDetail, _| item.name.clone())
                .with_text_color(DARK_TEXT)
                .padding(5.0)
                .fix_width(NAME_WIDTH)
            )
            // 添加文件大小
            .with_child(
                Label::dynamic(|item: &FileDetail, _| {
                    if item.file_type == "目录" {
                        "-".to_string()
                    } else {
                        format_file_size(item.size)
                    }
                })
                .with_text_color(DARK_TEXT)
                .padding(5.0)
                .fix_width(SIZE_WIDTH)
            )
            // 添加文件类型
            .with_child(
                Label::dynamic(|item: &FileDetail, _| item.file_type.clone())
                .with_text_color(DARK_TEXT)
                .padding(5.0)
                .fix_width(TYPE_WIDTH)
            )
            // 添加修改时间
            .with_child(
                Label::dynamic(|item: &FileDetail, _| item.modified.clone())
                .with_text_color(DARK_TEXT)
                .padding(5.0)
                .fix_width(DATE_WIDTH)
            )
    })
    .lens(AppState::current_dir_files);

    // 计算内容区域总宽度
    let content_width = ICON_WIDTH + NAME_WIDTH + SIZE_WIDTH + TYPE_WIDTH + DATE_WIDTH;
    
    // 给列表添加标题行 - 使用相同的宽度常量
    let header_row = Flex::row()
        .with_child(SizedBox::empty().fix_width(ICON_WIDTH))
        .with_child(Label::new("名称").with_text_color(DARK_TEXT).padding(5.0).fix_width(NAME_WIDTH))
        .with_child(Label::new("大小").with_text_color(DARK_TEXT).padding(5.0).fix_width(SIZE_WIDTH))
        .with_child(Label::new("类型").with_text_color(DARK_TEXT).padding(5.0).fix_width(TYPE_WIDTH))
        .with_child(Label::new("修改日期").with_text_color(DARK_TEXT).padding(5.0).fix_width(DATE_WIDTH))
        .background(HEADER_BACKGROUND);
    
    // 创建滚动区域
    let scrollable_list = Scroll::new(file_list)
        .fix_width(content_width); // 固定宽度，防止滚动条影响布局
    
    // 组合标题行和文件列表
    let file_view = Flex::column()
        .with_child(header_row.fix_width(content_width)) // 确保标题行宽度固定
        .with_flex_child(scrollable_list, 1.0);

    // 添加内边距并返回
    Container::new(file_view)
        .padding(10.0)
        .background(DARK_BACKGROUND)
        .expand()
} 