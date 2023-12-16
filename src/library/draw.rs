use crate::*;

pub fn rec(pos: Vector2, w: i32, h: i32, color: Color) {
    debug_assert!(w % 2 == 0);
    debug_assert!(h % 2 == 0);
    let (half_w, half_h) = (w / 2, h / 2);
    draw_rectangle(
        pos.x() as f32 - half_w as f32,
        pos.y() as f32 - half_h as f32,
        w as f32,
        h as f32,
        color,
    );
}

pub fn rec_top_left(pos: Vector2, w: i32, h: i32, color: Color) {
    draw_rectangle(pos.x() as f32, pos.y() as f32, w as f32, h as f32, color);
}

pub fn copy_texture(texture: &Texture2D, pos: Vector2) {
    texture.set_filter(FilterMode::Nearest);
    draw_texture(
        texture,
        texture.width().mul_add(-0.50, pos.x() as f32),
        texture.height().mul_add(-0.50, pos.y() as f32),
        WHITE,
    );
}

pub fn copy_with_rotation(texture: &Texture2D, pos: Vector2, rotation: f64) {
    texture.set_filter(FilterMode::Nearest);
    draw_texture_ex(
        texture,
        texture.width().mul_add(-0.50, pos.x() as f32),
        texture.height().mul_add(-0.50, pos.y() as f32),
        WHITE,
        DrawTextureParams {
            dest_size: None,
            rotation: rotation as f32,
            ..Default::default()
        },
    );
}
