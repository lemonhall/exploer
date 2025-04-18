use druid::{
    widget::Controller, 
    Widget, Event, Command, Target, Cursor
};
use crate::models::FileDetail;
use crate::commands::{NAVIGATE_TO, RESET_CURSOR};
use std::time::Duration;

/// 目录项控制器，处理悬停和点击事件
pub struct DirectoryItemController;

impl<W: Widget<FileDetail>> Controller<FileDetail, W> for DirectoryItemController {
    fn event(&mut self, child: &mut W, ctx: &mut druid::EventCtx, event: &Event, data: &mut FileDetail, env: &druid::Env) {
        match event {
            Event::MouseDown(mouse) if mouse.button.is_left() => {
                // 导航到该目录
                ctx.submit_command(Command::new(
                    NAVIGATE_TO,
                    data.full_path.clone().to_string_lossy().to_string(),
                    Target::Auto
                ));
                
                // 设置等待光标
                ctx.set_cursor(&Cursor::NotAllowed);
                
                // 1秒后自动重置光标
                ctx.request_timer(Duration::from_secs(1));
                
                ctx.set_handled();
            }
            Event::Timer(_) => {
                // 定时器触发，重置光标
                ctx.set_cursor(&Cursor::Arrow);
                ctx.request_update();
                ctx.set_handled();
            }
            Event::MouseMove(_) => {
                // 鼠标移动时设置悬停效果
                if ctx.is_hot() {
                    ctx.set_cursor(&Cursor::Pointer);
                    ctx.request_paint();
                }
            }
            Event::Command(cmd) => {
                if let Some(()) = cmd.get(RESET_CURSOR) {
                    // 收到重置光标命令
                    ctx.set_cursor(&Cursor::Arrow);
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