use druid::widget::{Flex, Label, Scroll, Container, Painter, SizedBox, CrossAxisAlignment};
use druid::{Widget, WidgetExt, RenderContext, Rect, Point};
use druid_widget_nursery::Tree;
use crate::models::{AppState, FileItem};
use crate::SELECT_DIRECTORY;
use super::constants::*;

/// 构建目录树视图（左侧面板）
pub fn build_directory_tree() -> impl Widget<AppState> {
    // 创建树形控件
    let tree = Tree::new(
        || {
            // 为每个目录项创建水平布局
            let mut row = Flex::row()
                .cross_axis_alignment(CrossAxisAlignment::Center); // 设置交叉轴对齐
                
            // 添加缩进指示器
            row.add_child(SizedBox::empty().fix_width(4.0));
                
            // 添加展开/折叠图标和文件夹图标
            row.add_child(
                Painter::new(|ctx, item: &FileItem, _env| {
                    // 绘制简单的文件夹图标
                    let rect = ctx.size().to_rect();
                    let icon_size = rect.size();
                    
                    // 计算垂直中心点
                    let center_y = rect.y0 + rect.height() / 2.0;
                    
                    // 文件夹底部 - 调整垂直位置
                    let folder_bottom = Rect::from_origin_size(
                        Point::new(rect.x0 + 1.0, center_y - 5.0),
                        (icon_size.width - 2.0, 10.0)
                    );
                    ctx.fill(folder_bottom, &FOLDER_COLOR);
                    
                    // 文件夹顶部 - 调整垂直位置
                    let folder_top = Rect::from_origin_size(
                        Point::new(rect.x0 + 1.0, center_y - 8.0),
                        (icon_size.width * 0.6, 3.0)
                    );
                    ctx.fill(folder_top, &FOLDER_COLOR);
                    
                    // 展开/折叠标记 - 调整垂直位置
                    if item.is_expanded {
                        let mark = Rect::from_origin_size(
                            Point::new(rect.x0 + 5.0, center_y),
                            (6.0, 2.0)
                        );
                        ctx.fill(mark, &ICON_COLOR);
                    } else {
                        let mark_h = Rect::from_origin_size(
                            Point::new(rect.x0 + 5.0, center_y),
                            (6.0, 2.0)
                        );
                        ctx.fill(mark_h, &ICON_COLOR);
                        
                        let mark_v = Rect::from_origin_size(
                            Point::new(rect.x0 + 7.0, center_y - 3.0),
                            (2.0, 6.0)
                        );
                        ctx.fill(mark_v, &ICON_COLOR);
                    }
                })
                .fix_size(16.0, 16.0)
                .padding((2.0, 0.0))
            );
            
            // 添加目录名标签
            row.add_child(
                Label::dynamic(|item: &FileItem, _| item.name.clone())
                .with_text_color(DARK_TEXT)
                .with_text_size(14.0) // 明确设置字体大小
                .padding((4.0, 0.0)) // 调整左右内边距
                // 点击目录时更新当前选中的目录路径
                .on_click(|ctx, data: &mut FileItem, _| {
                    // 获取当前点击的目录路径
                    let path = data.path.clone();
                    // 直接发送自定义命令
                    ctx.submit_command(SELECT_DIRECTORY.with(path));
                })
            );
            
            // 添加背景颜色效果
            row.background(
                Painter::new(|ctx, item: &FileItem, _env| {
                    let rect = ctx.size().to_rect();
                    
                    if item.is_selected {
                        ctx.fill(rect, &SELECTED_COLOR);
                    }
                })
            )
            .expand_width()
            .fix_height(24.0)
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