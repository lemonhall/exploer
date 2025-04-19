use druid::widget::{Button, Flex, TextBox, Align};
use druid::{Widget, WidgetExt, Color, Data, Lens};
use crate::models::AppState;
use super::constants::*;
use std::path::PathBuf;
use crate::SELECT_DIRECTORY;

/// 构建导航栏（顶部工具栏）
pub fn build_navigation_bar() -> impl Widget<AppState> {
    // 创建水平布局
    let mut nav_bar = Flex::row()
        .with_spacer(5.0) // 左侧边距
        .with_child(build_back_button())
        .with_spacer(2.0) // 按钮之间的间距
        .with_child(build_forward_button())
        .with_spacer(2.0)
        .with_child(build_up_button())
        .with_spacer(2.0)
        .with_child(build_refresh_button())
        .with_spacer(2.0)
        .with_child(build_home_button())
        .with_spacer(8.0); // 地址栏前的更大间距

    // 添加地址栏
    let address_box = TextBox::new()
        .with_placeholder("输入路径...")
        .lens(CurrentPathLens)
        .expand_width();

    // 将地址栏添加到导航栏
    nav_bar.add_flex_child(address_box, 1.0);
    
    // 添加转到按钮和右侧间距
    nav_bar.add_spacer(5.0);
    nav_bar.add_child(build_goto_button());
    nav_bar.add_spacer(5.0);

    // 包装导航栏，添加样式
    nav_bar
        .padding((0.0, 8.0)) // 垂直方向增加内边距
        .background(NAV_BAR_BACKGROUND)
        .expand_width()
}

/// 创建导航图标的标签文本
fn build_icon_label(text: &str) -> String {
    text.to_string()
}

/// 构建后退按钮
fn build_back_button() -> impl Widget<AppState> {
    Align::centered(
        Button::new("⬅")
            .on_click(|ctx, data: &mut AppState, _env| {
                if let Some(path) = data.navigate_back() {
                    ctx.submit_command(SELECT_DIRECTORY.with(path));
                }
            })
            .fix_width(36.0)
            .fix_height(36.0)
            .border(Color::TRANSPARENT, 0.0)
    )
}

/// 构建前进按钮
fn build_forward_button() -> impl Widget<AppState> {
    Align::centered(
        Button::new("➡")
            .on_click(|ctx, data: &mut AppState, _env| {
                if let Some(path) = data.navigate_forward() {
                    ctx.submit_command(SELECT_DIRECTORY.with(path));
                }
            })
            .fix_width(36.0)
            .fix_height(36.0)
            .border(Color::TRANSPARENT, 0.0)
    )
}

/// 构建上级目录按钮
fn build_up_button() -> impl Widget<AppState> {
    Align::centered(
        Button::new("⬆")
            .on_click(|ctx, data: &mut AppState, _env| {
                if let Some(current_path) = data.selected_path.clone() {
                    if let Some(parent) = current_path.parent() {
                        // 创建父目录路径的拷贝
                        let parent_path = parent.to_path_buf();
                        
                        // 添加到历史记录
                        data.add_to_history(parent_path.clone());
                        
                        // 发送命令导航到父目录
                        ctx.submit_command(SELECT_DIRECTORY.with(parent_path));
                    }
                }
            })
            .fix_width(36.0)
            .fix_height(36.0)
            .border(Color::TRANSPARENT, 0.0)
    )
}

/// 构建刷新按钮
fn build_refresh_button() -> impl Widget<AppState> {
    Align::centered(
        Button::new("🔄")
            .on_click(|ctx, data: &mut AppState, _env| {
                if let Some(path) = &data.selected_path {
                    // 重新导航到当前路径，刷新内容
                    ctx.submit_command(SELECT_DIRECTORY.with(path.clone()));
                }
            })
            .fix_width(36.0)
            .fix_height(36.0)
            .border(Color::TRANSPARENT, 0.0)
    )
}

/// 构建主目录按钮
fn build_home_button() -> impl Widget<AppState> {
    Align::centered(
        Button::new("🏠")
            .on_click(|ctx, _data: &mut AppState, _env| {
                if let Some(home_dir) = dirs::home_dir() {
                    ctx.submit_command(SELECT_DIRECTORY.with(home_dir));
                }
            })
            .fix_width(36.0)
            .fix_height(36.0)
            .border(Color::TRANSPARENT, 0.0)
    )
}

/// 构建转到按钮
fn build_goto_button() -> impl Widget<AppState> {
    Align::centered(
        Button::new("➥")
            .on_click(|_ctx, _data: &mut AppState, _env| {
                // 这里暂时不需要操作，因为TextBox的lens已经更新了path
                // 地址变更会自动通过lens处理
            })
            .fix_width(36.0)
            .fix_height(36.0)
            .border(Color::TRANSPARENT, 0.0)
    )
}

/// 为当前路径字符串创建Lens
#[derive(Clone, Data)]
pub struct CurrentPathLens;

impl Lens<AppState, String> for CurrentPathLens {
    fn with<V, F: FnOnce(&String) -> V>(&self, data: &AppState, f: F) -> V {
        let path_string = match &data.selected_path {
            Some(path) => path.to_string_lossy().into_owned(),
            None => String::new(),
        };
        f(&path_string)
    }

    fn with_mut<V, F: FnOnce(&mut String) -> V>(&self, data: &mut AppState, f: F) -> V {
        let mut path_string = match &data.selected_path {
            Some(path) => path.to_string_lossy().into_owned(),
            None => String::new(),
        };
        
        let result = f(&mut path_string);
        
        // 只有当路径字符串发生变化时才更新
        if let Some(ref old_path) = data.selected_path {
            if old_path.to_string_lossy() != path_string {
                let new_path = PathBuf::from(&path_string);
                data.add_to_history(new_path.clone());
                data.selected_path = Some(new_path);
                // 导航逻辑将通过委托处理
            }
        } else if !path_string.is_empty() {
            let new_path = PathBuf::from(&path_string);
            data.add_to_history(new_path.clone());
            data.selected_path = Some(new_path);
        }
        
        result
    }
} 