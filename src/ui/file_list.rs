use druid::{
    widget::{Flex, Label, List, Scroll, ViewSwitcher, Controller, Painter},
    Command, Widget, WidgetExt, Color, Target, Event, RenderContext, Point, Rect,
    kurbo::BezPath
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
fn create_file_row(color: Color, icon_color: Color, is_dir: bool) -> impl Widget<FileDetail> {
    // 绘制图标
    let icon = Painter::new(move |ctx, _data: &FileDetail, _env| {
        // 计算图标区域
        let rect = ctx.size().to_rect();
        let icon_size = rect.height() * 0.7;
        let center_y = rect.y0 + rect.height() / 2.0;
        let center_x = rect.x0 + rect.width() / 2.0;
        
        if is_dir {
            // 绘制文件夹图标
            // 文件夹底部
            let folder_bottom = Rect::from_origin_size(
                Point::new(center_x - icon_size/2.0, center_y - icon_size/2.0 + icon_size*0.2),
                (icon_size, icon_size * 0.6)
            );
            ctx.fill(folder_bottom, &icon_color);
            
            // 文件夹顶部
            let folder_top = Rect::from_origin_size(
                Point::new(center_x - icon_size/2.0, center_y - icon_size/2.0),
                (icon_size * 0.6, icon_size * 0.2)
            );
            ctx.fill(folder_top, &icon_color);
        } else {
            // 绘制文件图标
            // 文件主体
            let file_body = Rect::from_origin_size(
                Point::new(center_x - icon_size/2.0, center_y - icon_size/2.0),
                (icon_size * 0.8, icon_size)
            );
            ctx.fill(file_body, &icon_color);
            
            // 文件折角
            let corner_size = icon_size * 0.25;
            let mut path = BezPath::new();
            path.move_to((center_x + icon_size/2.0 - corner_size, center_y - icon_size/2.0));
            path.line_to((center_x + icon_size/2.0 - corner_size, center_y - icon_size/2.0 + corner_size));
            path.line_to((center_x + icon_size/2.0, center_y - icon_size/2.0 + corner_size));
            path.line_to((center_x + icon_size/2.0, center_y - icon_size/2.0));
            path.line_to((center_x + icon_size/2.0 - corner_size, center_y - icon_size/2.0));
            path.close_path();
            ctx.fill(path, &Color::rgb8(220, 220, 220));
            
            // 文件线条（模拟文本）
            for i in 0..3 {
                let line_y = center_y - icon_size/4.0 + i as f64 * (icon_size/3.0);
                let line = Rect::from_origin_size(
                    Point::new(center_x - icon_size/3.0, line_y),
                    (icon_size * 0.5, icon_size/12.0)
                );
                ctx.fill(line, &Color::rgb8(180, 180, 180));
            }
        }
    })
    .fix_size(24.0, 24.0);
    
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
        .with_flex_child(name_row, 0.4)
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
                let dir_row = create_file_row(FOLDER_COLOR, FOLDER_COLOR, true)
                    .controller(DirectoryItemController);
                
                Box::new(dir_row)
            } else if file_type.ends_with(" 文件") {
                // 提取文件扩展名并设置相应颜色
                let ext = file_type.split_whitespace().next().unwrap_or("");
                match ext {
                    "txt" | "md" | "json" | "xml" | "html" | "css" | "js" | "py" | "rs" | "c" | "cpp" | "h" | "hpp" | "java" | "go" | "php" | "rb" => {
                        Box::new(create_file_row(TEXT_FILE_COLOR, TEXT_FILE_COLOR, false))
                    }
                    "jpg" | "jpeg" | "png" | "gif" | "bmp" | "svg" | "ico" => {
                        Box::new(create_file_row(IMAGE_FILE_COLOR, IMAGE_FILE_COLOR, false))
                    }
                    "mp3" | "wav" | "flac" | "ogg" | "aac" => {
                        Box::new(create_file_row(AUDIO_FILE_COLOR, AUDIO_FILE_COLOR, false))
                    }
                    "mp4" | "avi" | "mkv" | "mov" | "wmv" => {
                        Box::new(create_file_row(VIDEO_FILE_COLOR, VIDEO_FILE_COLOR, false))
                    }
                    "zip" | "rar" | "7z" | "tar" | "gz" => {
                        Box::new(create_file_row(ARCHIVE_FILE_COLOR, ARCHIVE_FILE_COLOR, false))
                    }
                    "exe" | "dll" | "so" | "dylib" => {
                        Box::new(create_file_row(EXECUTABLE_FILE_COLOR, EXECUTABLE_FILE_COLOR, false))
                    }
                    _ => Box::new(create_file_row(REGULAR_FILE_COLOR, REGULAR_FILE_COLOR, false)),
                }
            } else {
                Box::new(create_file_row(REGULAR_FILE_COLOR, REGULAR_FILE_COLOR, false))
            }
        },
    )
} 