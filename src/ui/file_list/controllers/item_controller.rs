use druid::{
    widget::Controller,
    Widget, Event, Command, Target, TimerToken, Cursor
};
use crate::models::FileDetail;
use crate::commands::{OPEN_FILE, RESET_CURSOR};
use std::time::Duration;

/// 文件项控制器，处理双击打开文件
pub struct FileItemController {
    cursor_timer: Option<TimerToken>,
}

impl FileItemController {
    pub fn new() -> Self {
        Self {
            cursor_timer: None,
        }
    }
}

impl<W: Widget<FileDetail>> Controller<FileDetail, W> for FileItemController {
    fn event(&mut self, child: &mut W, ctx: &mut druid::EventCtx, event: &Event, data: &mut FileDetail, env: &druid::Env) {
        match event {
            Event::MouseDown(mouse) if mouse.button.is_left() && mouse.count >= 2 => {
                // 双击时打开文件并设置等待光标
                ctx.submit_command(Command::new(
                    OPEN_FILE,
                    data.full_path.clone(),
                    Target::Auto
                ));
                
                // 设置等待光标
                ctx.set_cursor(&Cursor::NotAllowed);
                
                // 创建定时器，2秒后重置光标
                let timer_token = ctx.request_timer(Duration::from_secs(2));
                self.cursor_timer = Some(timer_token);
                
                ctx.set_handled();
            }
            Event::Timer(token) => {
                if Some(*token) == self.cursor_timer {
                    // 定时器触发，重置光标
                    ctx.set_cursor(&Cursor::Arrow);
                    self.cursor_timer = None;
                    ctx.request_update();
                    ctx.set_handled();
                }
            }
            Event::MouseMove(_) => {
                // 鼠标移动时设置悬停效果，但只在没有等待定时器时
                if ctx.is_hot() && self.cursor_timer.is_none() {
                    ctx.set_cursor(&Cursor::Pointer);
                    ctx.request_paint();
                }
            }
            Event::Command(cmd) => {
                if let Some(()) = cmd.get(RESET_CURSOR) {
                    // 收到重置光标命令
                    ctx.set_cursor(&Cursor::Arrow);
                    self.cursor_timer = None;
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