use druid::{
    widget::{ViewSwitcher},
    Widget, WidgetExt
};
use std::boxed::Box;
use crate::models::FileDetail;
use crate::ui::constants::*;
use super::item_styles::create_file_row;
use super::controllers::{FileItemController, DirectoryItemController};

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
            } else if file_type == "驱动器" {
                // 为驱动器创建特殊行
                let drive_row = create_file_row(FOLDER_COLOR, FOLDER_COLOR, true)
                    .controller(DirectoryItemController);
                
                Box::new(drive_row)
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