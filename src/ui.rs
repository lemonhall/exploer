use druid::widget::{Flex, Label, Scroll, Container};
use druid::{Widget, WidgetExt};
use druid_widget_nursery::Tree;
use crate::models::{AppState, FileItem};

/// 构建应用程序的UI界面
pub fn build_ui() -> impl Widget<AppState> {
    // 创建树形控件
    let tree = Tree::new(
        || {
            // 为每个文件项创建水平布局
            Flex::row()
                // 添加展开/折叠/空白图标
                .with_child(Label::dynamic(|item: &FileItem, _| {
                    if item.children.is_empty() {
                        // 文件项没有子项，显示空白占位符
                        "    ".to_string()
                    } else if item.is_expanded {
                        // 目录已展开，显示向下箭头
                        "▼ ".to_string()
                    } else {
                        // 目录未展开，显示向右箭头
                        "► ".to_string()
                    }
                }))
                // 添加文件名标签
                .with_child(Label::dynamic(|item: &FileItem, _| item.name.clone())
                          .padding((0.0, 5.0, 5.0, 5.0)))
        },
        // 使用is_expanded属性作为展开/折叠的依据
        FileItem::is_expanded,
    )
    // 将树形控件连接到应用程序状态的root属性
    .lens(AppState::root);

    // 使用Container包装Tree控件，添加内边距
    let tree_with_padding = Container::new(tree)
        .padding((10.0, 10.0, 10.0, 20.0)); // 左、上、右、下边距，底部边距更大

    // 使用Scroll包装带边距的树形控件，使其可滚动
    Scroll::new(tree_with_padding)
        .vertical() // 允许垂直滚动
        .expand() // 扩展填充可用空间
} 