use druid::{
    widget::{Flex, Label, List, Scroll, ViewSwitcher, Controller},
    Command, Widget, WidgetExt, Color, Target, Event, RenderContext
};
use crate::models::{AppState, FileDetail};
use crate::ui::constants::*;
use crate::commands::NAVIGATE_TO;

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

/// 目录项控制器，处理悬停和点击事件
struct DirectoryItemController;

impl<W: Widget<FileDetail>> Controller<FileDetail, W> for DirectoryItemController {
    fn event(&mut self, child: &mut W, ctx: &mut druid::EventCtx, event: &Event, data: &mut FileDetail, env: &druid::Env) {
        match event {
            Event::MouseDown(mouse) if mouse.button.is_left() => {
                // 导航到该目录
                ctx.submit_command(Command::new(
                    NAVIGATE_TO,
                    data.full_path.clone(),
                    Target::Auto
                ));
                ctx.set_handled();
            }
            Event::MouseMove(_) => {
                // 鼠标移动时设置悬停效果
                if ctx.is_hot() {
                    ctx.set_cursor(&druid::Cursor::Pointer);
                    ctx.request_paint();
                }
            }
            _ => {}
        }
        child.event(ctx, event, data, env);
    }

    fn update(&mut self, child: &mut W, ctx: &mut druid::UpdateCtx, old_data: &FileDetail, data: &FileDetail, env: &druid::Env) {
        child.update(ctx, old_data, data, env);
    }

    fn lifecycle(&mut self, child: &mut W, ctx: &mut druid::LifeCycleCtx, event: &druid::LifeCycle, data: &FileDetail, env: &druid::Env) {
        child.lifecycle(ctx, event, data, env);
    }
}

/// 创建一个文件列表行，根据给定的颜色
fn create_file_row(color: Color) -> impl Widget<FileDetail> {
    // 名称列
    let name_label = Label::dynamic(|data: &FileDetail, _| data.name.clone())
        .with_text_size(14.0)
        .with_text_color(color)
        .align_left();

    // 大小列 - 格式化显示
    let size_label = Label::dynamic(|data: &FileDetail, _| {
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

    Flex::row()
        .with_flex_child(name_label, 0.4)
        .with_flex_child(size_label, 0.2)
        .with_flex_child(type_label, 0.2)
        .with_flex_child(modified_label, 0.2)
        .padding(10.0)
}

/// 构建文件列表中的单个文件项
fn file_list_item() -> impl Widget<FileDetail> {
    // 使用ViewSwitcher为不同类型的文件设置不同的颜色
    ViewSwitcher::new(
        |data: &FileDetail, _env| data.file_type.clone(),
        |file_type, _data, _env| {
            if file_type == "目录" {
                // 为目录创建带有特殊交互的行
                let dir_row = create_file_row(FOLDER_COLOR)
                    .controller(DirectoryItemController);
                
                Box::new(dir_row)
            } else if file_type.ends_with(" 文件") {
                // 提取文件扩展名并设置相应颜色
                let ext = file_type.split_whitespace().next().unwrap_or("");
                match ext {
                    "txt" | "md" | "json" | "xml" | "html" | "css" | "js" | "py" | "rs" | "c" | "cpp" | "h" | "hpp" | "java" | "go" | "php" | "rb" => {
                        Box::new(create_file_row(TEXT_FILE_COLOR))
                    }
                    "jpg" | "jpeg" | "png" | "gif" | "bmp" | "svg" | "ico" => {
                        Box::new(create_file_row(IMAGE_FILE_COLOR))
                    }
                    "mp3" | "wav" | "flac" | "ogg" | "aac" => {
                        Box::new(create_file_row(AUDIO_FILE_COLOR))
                    }
                    "mp4" | "avi" | "mkv" | "mov" | "wmv" => {
                        Box::new(create_file_row(VIDEO_FILE_COLOR))
                    }
                    "zip" | "rar" | "7z" | "tar" | "gz" => {
                        Box::new(create_file_row(ARCHIVE_FILE_COLOR))
                    }
                    "exe" | "dll" | "so" | "dylib" => {
                        Box::new(create_file_row(EXECUTABLE_FILE_COLOR))
                    }
                    _ => Box::new(create_file_row(REGULAR_FILE_COLOR)),
                }
            } else {
                Box::new(create_file_row(REGULAR_FILE_COLOR))
            }
        },
    )
} 