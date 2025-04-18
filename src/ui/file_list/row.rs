use druid::{
    widget::{Flex, Label, ViewSwitcher, Painter},
    Widget, WidgetExt, Color, RenderContext, Point, Rect, kurbo::BezPath
};
use crate::models::FileDetail;
use crate::ui::constants::*;
use super::controllers::DirectoryItemController;

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