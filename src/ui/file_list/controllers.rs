use druid::{
    widget::Controller, 
    Widget, Event, Command, Target
};
use crate::models::FileDetail;
use crate::commands::NAVIGATE_TO;

/// 目录项控制器，处理悬停和点击事件
pub struct DirectoryItemController;

impl<W: Widget<FileDetail>> Controller<FileDetail, W> for DirectoryItemController {
    fn event(&mut self, child: &mut W, ctx: &mut druid::EventCtx, event: &Event, data: &mut FileDetail, env: &druid::Env) {
        match event {
            Event::MouseDown(mouse) if mouse.button.is_left() => {
                // 导航到该目录
                ctx.submit_command(Command::new(
                    NAVIGATE_TO,
                    data.full_path.clone(),
                    Target::Auto
                ));
                ctx.set_handled();
            }
            Event::MouseMove(_) => {
                // 鼠标移动时设置悬停效果
                if ctx.is_hot() {
                    ctx.set_cursor(&druid::Cursor::Pointer);
                    ctx.request_paint();
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