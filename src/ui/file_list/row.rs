use druid::{
    widget::{Flex, Label, ViewSwitcher, Painter, Controller},
    Widget, WidgetExt, Color, RenderContext, Point, Rect, kurbo::BezPath,
    Event, Command, Target, TimerToken, Cursor
};
use crate::models::FileDetail;
use crate::ui::constants::*;
use crate::commands::{OPEN_FILE, RESET_CURSOR};
use std::time::Duration;
use super::controllers::DirectoryItemController;

/// 文件项控制器，处理双击打开文件
struct FileItemController {
    cursor_timer: Option<TimerToken>,
}

impl FileItemController {
    fn new() -> Self {
        Self {
            cursor_timer: None,
        }
    }
}

impl<W: Widget<FileDetail>> Controller<FileDetail, W> for FileItemController {
    fn event(&mut self, child: &mut W, ctx: &mut druid::EventCtx, event: &Event, data: &mut FileDetail, env: &druid::Env) {
        match event {
            Event::MouseDown(mouse) if mouse.button.is_left() && mouse.count >= 2 => {
                // 双击时打开文件并设置等待光标
                ctx.submit_command(Command::new(
                    OPEN_FILE,
                    data.full_path.clone().to_string_lossy().to_string(),
                    Target::Auto
                ));
                
                // 设置等待光标
                ctx.set_cursor(&Cursor::NotAllowed);
                
                // 创建定时器，2秒后重置光标
                let timer_token = ctx.request_timer(Duration::from_secs(2));
                self.cursor_timer = Some(timer_token);
                
                ctx.set_handled();
            }
            Event::Timer(token) => {
                if Some(*token) == self.cursor_timer {
                    // 定时器触发，重置光标
                    ctx.set_cursor(&Cursor::Arrow);
                    self.cursor_timer = None;
                    ctx.request_update();
                    ctx.set_handled();
                }
            }
            Event::MouseMove(_) => {
                // 鼠标移动时设置悬停效果，但只在没有等待定时器时
                if ctx.is_hot() && self.cursor_timer.is_none() {
                    ctx.set_cursor(&Cursor::Pointer);
                    ctx.request_paint();
                }
            }
            Event::Command(cmd) => {
                if let Some(()) = cmd.get(RESET_CURSOR) {
                    // 收到重置光标命令
                    ctx.set_cursor(&Cursor::Arrow);
                    self.cursor_timer = None;
                    ctx.set_handled();
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
pub fn file_list_item() -> impl Widget<FileDetail> {
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
                let row = match ext {
                    "txt" | "md" | "json" | "xml" | "html" | "css" | "js" | "py" | "rs" | "c" | "cpp" | "h" | "hpp" | "java" | "go" | "php" | "rb" => {
                        create_file_row(TEXT_FILE_COLOR, TEXT_FILE_COLOR, false)
                    }
                    "jpg" | "jpeg" | "png" | "gif" | "bmp" | "svg" | "ico" => {
                        create_file_row(IMAGE_FILE_COLOR, IMAGE_FILE_COLOR, false)
                    }
                    "mp3" | "wav" | "flac" | "ogg" | "aac" => {
                        create_file_row(AUDIO_FILE_COLOR, AUDIO_FILE_COLOR, false)
                    }
                    "mp4" | "avi" | "mkv" | "mov" | "wmv" => {
                        create_file_row(VIDEO_FILE_COLOR, VIDEO_FILE_COLOR, false)
                    }
                    "zip" | "rar" | "7z" | "tar" | "gz" => {
                        create_file_row(ARCHIVE_FILE_COLOR, ARCHIVE_FILE_COLOR, false)
                    }
                    "exe" | "dll" | "so" | "dylib" => {
                        create_file_row(EXECUTABLE_FILE_COLOR, EXECUTABLE_FILE_COLOR, false)
                    }
                    _ => create_file_row(REGULAR_FILE_COLOR, REGULAR_FILE_COLOR, false),
                };
                
                // 为所有文件添加双击打开功能
                Box::new(row.controller(FileItemController::new()))
            } else {
                Box::new(create_file_row(REGULAR_FILE_COLOR, REGULAR_FILE_COLOR, false)
                    .controller(FileItemController::new()))
            }
        },
    )
} 