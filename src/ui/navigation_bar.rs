use druid::widget::{Button, Flex, TextBox, Label, SizedBox};
use druid::{Widget, WidgetExt, Color, UnitPoint, FontWeight, FontStyle, Data, Lens, EventCtx, Target};
use crate::models::AppState;
use super::constants::*;
use std::path::PathBuf;
use crate::commands::*;
use crate::SELECT_DIRECTORY;

/// 构建导航栏（顶部工具栏）
pub fn build_navigation_bar() -> impl Widget<AppState> {
    // 创建水平布局
    let mut nav_bar = Flex::row()
        .with_child(build_back_button())
        .with_child(build_forward_button())
        .with_child(build_up_button())
        .with_child(build_refresh_button())
        .with_child(build_home_button());

    // 添加地址栏
    let address_box = TextBox::new()
        .with_placeholder("输入路径...")
        .lens(CurrentPathLens)
        .expand_width();

    // 将地址栏添加到导航栏
    nav_bar.add_flex_child(address_box, 1.0);
    
    // 添加转到按钮
    nav_bar.add_child(build_goto_button());

    // 包装导航栏，添加样式
    nav_bar
        .padding(8.0)
        .background(NAV_BAR_BACKGROUND)
        .expand_width()
}

/// 构建后退按钮
fn build_back_button() -> impl Widget<AppState> {
    Button::new("⬅️")
        .on_click(|ctx, data: &mut AppState, _env| {
            if let Some(path) = data.navigate_back() {
                ctx.submit_command(SELECT_DIRECTORY.with(path));
            }
        })
        .padding(5.0)
        .fix_width(36.0)
        .fix_height(36.0)
}

/// 构建前进按钮
fn build_forward_button() -> impl Widget<AppState> {
    Button::new("➡️")
        .on_click(|ctx, data: &mut AppState, _env| {
            if let Some(path) = data.navigate_forward() {
                ctx.submit_command(SELECT_DIRECTORY.with(path));
            }
        })
        .padding(5.0)
        .fix_width(36.0)
        .fix_height(36.0)
}

/// 构建上级目录按钮
fn build_up_button() -> impl Widget<AppState> {
    Button::new("⬆️")
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
        .padding(5.0)
        .fix_width(36.0)
        .fix_height(36.0)
}

/// 构建刷新按钮
fn build_refresh_button() -> impl Widget<AppState> {
    Button::new("🔄")
        .on_click(|ctx, data: &mut AppState, _env| {
            if let Some(path) = &data.selected_path {
                // 重新导航到当前路径，刷新内容
                ctx.submit_command(SELECT_DIRECTORY.with(path.clone()));
            }
        })
        .padding(5.0)
        .fix_width(36.0)
        .fix_height(36.0)
}

/// 构建主目录按钮
fn build_home_button() -> impl Widget<AppState> {
    Button::new("🏠")
        .on_click(|ctx, _data: &mut AppState, _env| {
            if let Some(home_dir) = dirs::home_dir() {
                ctx.submit_command(SELECT_DIRECTORY.with(home_dir));
            }
        })
        .padding(5.0)
        .fix_width(36.0)
        .fix_height(36.0)
}

/// 构建转到按钮
fn build_goto_button() -> impl Widget<AppState> {
    Button::new("➥")
        .on_click(|_ctx, _data: &mut AppState, _env| {
            // 这里暂时不需要操作，因为TextBox的lens已经更新了path
            // 地址变更会自动通过lens处理
        })
        .padding(5.0)
        .fix_width(36.0)
        .fix_height(36.0)
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