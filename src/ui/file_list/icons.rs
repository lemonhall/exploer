use druid::{
    widget::Painter,
    Color, RenderContext, Point, Rect, kurbo::BezPath, Widget, WidgetExt
};
use crate::models::FileDetail;
use std::boxed::Box;

/// 创建文件夹图标
pub fn create_folder_icon(icon_color: Color) -> Box<dyn Widget<FileDetail>> {
    let painter = Painter::new(move |ctx, _data: &FileDetail, _env| {
        // 计算图标区域
        let rect = ctx.size().to_rect();
        let icon_size = rect.height() * 0.7;
        let center_y = rect.y0 + rect.height() / 2.0;
        let center_x = rect.x0 + rect.width() / 2.0;
        
        // 绘制文件夹图标
        // 文件夹底部
        let folder_bottom = Rect::from_origin_size(
            Point::new(center_x - icon_size/2.0, center_y - icon_size/2.0 + icon_size*0.2),
            (icon_size, icon_size * 0.6)
        );
        ctx.fill(folder_bottom, &icon_color);
        
        // 文件夹顶部
        let folder_top = Rect::from_origin_size(
            Point::new(center_x - icon_size/2.0, center_y - icon_size/2.0),
            (icon_size * 0.6, icon_size * 0.2)
        );
        ctx.fill(folder_top, &icon_color);
    })
    .fix_size(24.0, 24.0);
    
    Box::new(painter)
}

/// 创建文件图标
pub fn create_file_icon(icon_color: Color) -> Box<dyn Widget<FileDetail>> {
    let painter = Painter::new(move |ctx, _data: &FileDetail, _env| {
        // 计算图标区域
        let rect = ctx.size().to_rect();
        let icon_size = rect.height() * 0.7;
        let center_y = rect.y0 + rect.height() / 2.0;
        let center_x = rect.x0 + rect.width() / 2.0;
        
        // 绘制文件图标
        // 文件主体
        let file_body = Rect::from_origin_size(
            Point::new(center_x - icon_size/2.0, center_y - icon_size/2.0),
            (icon_size * 0.8, icon_size)
        );
        ctx.fill(file_body, &icon_color);
        
        // 文件折角
        let corner_size = icon_size * 0.25;
        let mut path = BezPath::new();
        path.move_to((center_x + icon_size/2.0 - corner_size, center_y - icon_size/2.0));
        path.line_to((center_x + icon_size/2.0 - corner_size, center_y - icon_size/2.0 + corner_size));
        path.line_to((center_x + icon_size/2.0, center_y - icon_size/2.0 + corner_size));
        path.line_to((center_x + icon_size/2.0, center_y - icon_size/2.0));
        path.line_to((center_x + icon_size/2.0 - corner_size, center_y - icon_size/2.0));
        path.close_path();
        ctx.fill(path, &Color::rgb8(220, 220, 220));
        
        // 文件线条（模拟文本）
        for i in 0..3 {
            let line_y = center_y - icon_size/4.0 + i as f64 * (icon_size/3.0);
            let line = Rect::from_origin_size(
                Point::new(center_x - icon_size/3.0, line_y),
                (icon_size * 0.5, icon_size/12.0)
            );
            ctx.fill(line, &Color::rgb8(180, 180, 180));
        }
    })
    .fix_size(24.0, 24.0);
    
    Box::new(painter)
} 