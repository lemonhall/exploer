use druid::Color;

/// 当前选中项的背景颜色（适当加深的蓝色，提高对比度）
pub const SELECTED_COLOR: Color = Color::rgb8(65, 105, 225);

/// 深色主题背景色
pub const DARK_BACKGROUND: Color = Color::rgb8(32, 32, 32);

/// 深色主题文本颜色
pub const DARK_TEXT: Color = Color::rgb8(230, 230, 230);

/// 选中项的文本颜色
pub const SELECTED_TEXT: Color = Color::rgb8(255, 255, 255);

/// 文件夹图标颜色
pub const FOLDER_COLOR: Color = Color::rgb8(255, 209, 94);

/// 文件图标颜色
pub const FILE_COLOR: Color = Color::rgb8(240, 240, 240);

/// 控件图标颜色
pub const ICON_COLOR: Color = Color::rgb8(200, 200, 200);

/// 表头背景色
pub const HEADER_BACKGROUND: Color = Color::rgb8(50, 50, 50);

// 不同文件类型的颜色
/// Rust文件图标颜色
pub const RUST_FILE_COLOR: Color = Color::rgb8(244, 110, 66);

/// HTML文件图标颜色
pub const HTML_FILE_COLOR: Color = Color::rgb8(240, 101, 41);

/// JS文件图标颜色
pub const JS_FILE_COLOR: Color = Color::rgb8(247, 223, 30);

/// CSS文件图标颜色
pub const CSS_FILE_COLOR: Color = Color::rgb8(86, 61, 124);

/// TOML文件图标颜色
pub const TOML_FILE_COLOR: Color = Color::rgb8(180, 180, 180);

/// MD文件图标颜色
pub const MD_FILE_COLOR: Color = Color::rgb8(108, 165, 209);

/// SVG/图片文件图标颜色
pub const IMAGE_FILE_COLOR: Color = Color::rgb8(120, 195, 85);

/// 可执行文件图标颜色
pub const EXE_FILE_COLOR: Color = Color::rgb8(80, 220, 100);

/// ICO文件图标颜色
pub const ICO_FILE_COLOR: Color = Color::rgb8(255, 165, 0); 