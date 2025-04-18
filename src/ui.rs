use druid::widget::{Flex, Label, Scroll, Container, Split, List, Painter, SizedBox};
use druid::{Widget, WidgetExt, Color, RenderContext, Rect, Point};
use druid_widget_nursery::Tree;
use crate::models::{AppState, FileItem, FileDetail};
use crate::SELECT_DIRECTORY;

/// 当前选中项的背景颜色（浅蓝色）
const SELECTED_COLOR: Color = Color::rgb8(204, 232, 255);

/// 鼠标悬停项的背景颜色（更浅的蓝色）
const HOVERED_COLOR: Color = Color::rgb8(229, 243, 255);

/// 深色主题背景色
const DARK_BACKGROUND: Color = Color::rgb8(32, 32, 32);

/// 深色主题文本颜色
const DARK_TEXT: Color = Color::rgb8(230, 230, 230);

/// 构建目录树视图（左侧面板）
fn build_directory_tree() -> impl Widget<AppState> {
    // 创建树形控件
    let tree = Tree::new(
        || {
            // 为每个目录项创建水平布局
            Flex::row()
                // 添加缩进指示器
                .with_child(SizedBox::empty().fix_width(4.0))
                // 添加展开/折叠图标和文件夹图标
                .with_child(
                    Painter::new(|ctx, item: &FileItem, _env| {
                        // 绘制简单的文件夹图标
                        let rect = ctx.size().to_rect();
                        let icon_size = rect.size();
                        
                        // 文件夹底部
                        let folder_bottom = Rect::from_origin_size(
                            Point::new(rect.x0 + 1.0, rect.y0 + 3.0),
                            (icon_size.width - 2.0, icon_size.height - 4.0)
                        );
                        ctx.fill(folder_bottom, &Color::rgb8(255, 209, 94));
                        
                        // 文件夹顶部
                        let folder_top = Rect::from_origin_size(
                            Point::new(rect.x0 + 1.0, rect.y0 + 1.0),
                            (icon_size.width * 0.6, 3.0)
                        );
                        ctx.fill(folder_top, &Color::rgb8(255, 209, 94));
                        
                        // 展开/折叠标记
                        if item.is_expanded {
                            let mark = Rect::from_origin_size(
                                Point::new(rect.x0 + 5.0, rect.y0 + 7.0),
                                (6.0, 2.0)
                            );
                            ctx.fill(mark, &Color::rgb8(200, 200, 200));
                        } else {
                            let mark_h = Rect::from_origin_size(
                                Point::new(rect.x0 + 5.0, rect.y0 + 7.0),
                                (6.0, 2.0)
                            );
                            ctx.fill(mark_h, &Color::rgb8(200, 200, 200));
                            
                            let mark_v = Rect::from_origin_size(
                                Point::new(rect.x0 + 7.0, rect.y0 + 5.0),
                                (2.0, 6.0)
                            );
                            ctx.fill(mark_v, &Color::rgb8(200, 200, 200));
                        }
                    })
                    .fix_size(16.0, 16.0)
                    .padding((2.0, 2.0))
                )
                // 添加目录名标签
                .with_child(
                    Label::dynamic(|item: &FileItem, _| item.name.clone())
                    .with_text_color(DARK_TEXT)
                    .padding((0.0, 5.0, 5.0, 5.0))
                    // 点击目录时更新当前选中的目录路径
                    .on_click(|ctx, data: &mut FileItem, _| {
                        // 获取当前点击的目录路径
                        let path = data.path.clone();
                        // 直接发送自定义命令
                        ctx.submit_command(SELECT_DIRECTORY.with(path));
                    })
                )
                // 添加背景颜色效果
                .background(
                    Painter::new(|ctx, item: &FileItem, _env| {
                        let rect = ctx.size().to_rect();
                        
                        if item.is_selected {
                            ctx.fill(rect, &SELECTED_COLOR);
                        }
                    })
                )
                .expand_width()
                .height(24.0) // 固定高度使布局更整齐
        },
        FileItem::is_expanded,
    )
    .lens(AppState::root);

    // 使用Container包装Tree控件，添加内边距和背景色
    let tree_with_padding = Container::new(tree)
        .padding((10.0, 10.0, 10.0, 20.0))
        .background(DARK_BACKGROUND); // 使用深色背景

    // 使用Scroll包装带边距的树形控件，使其可滚动
    Scroll::new(tree_with_padding)
        .content_must_fill(true)  // 限制滚动范围
        .vertical()
        .expand()
}

/// 构建文件列表视图（右侧面板）
fn build_file_list() -> impl Widget<AppState> {
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
                        ctx.fill(rect, &Color::rgb8(255, 223, 128));
                    } else {
                        // 绘制文件图标（简单的白色纸张）
                        ctx.fill(rect, &Color::rgb8(240, 240, 240));
                    }
                })
                .fix_size(16.0, 16.0)
                .padding((5.0, 5.0))
            )
            // 添加文件名
            .with_child(
                Label::dynamic(|item: &FileDetail, _| item.name.clone())
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
                .padding(5.0)
                .fix_width(100.0)
            )
            // 添加文件类型
            .with_child(
                Label::dynamic(|item: &FileDetail, _| item.file_type.clone())
                .padding(5.0)
                .fix_width(100.0)
            )
            // 添加修改时间
            .with_child(
                Label::dynamic(|item: &FileDetail, _| item.modified.clone())
                .padding(5.0)
                .fix_width(150.0)
            )
    })
    .lens(AppState::current_dir_files);

    // 给列表添加标题行
    let header_row = Flex::row()
        .with_child(Label::new("").fix_size(26.0, 20.0))
        .with_child(Label::new("名称").expand_width().padding(5.0))
        .with_child(Label::new("大小").padding(5.0).fix_width(100.0))
        .with_child(Label::new("类型").padding(5.0).fix_width(100.0))
        .with_child(Label::new("修改日期").padding(5.0).fix_width(150.0))
        .background(Color::rgb8(50, 50, 50));
    
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

/// 格式化文件大小显示
fn format_file_size(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if size < KB {
        format!("{} B", size)
    } else if size < MB {
        format!("{:.2} KB", size as f64 / KB as f64)
    } else if size < GB {
        format!("{:.2} MB", size as f64 / MB as f64)
    } else {
        format!("{:.2} GB", size as f64 / GB as f64)
    }
}

/// 构建应用程序的UI界面
pub fn build_ui() -> impl Widget<AppState> {
    // 创建分割视图，左侧是目录树，右侧是文件列表
    let split = Split::columns(
        build_directory_tree(),
        build_file_list()
    )
    .split_point(0.3)  // 左侧面板占30%的宽度
    .draggable(true)   // 允许调整分割位置
    .solid_bar(true);  // 使用实心分隔条

    // 使用Container包装分割视图，提供边距
    Container::new(split)
        .padding(5.0)
        .background(DARK_BACKGROUND) // 整个应用使用深色背景
        .expand()
} 