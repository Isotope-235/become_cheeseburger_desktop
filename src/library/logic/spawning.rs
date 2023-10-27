use crate::*;

pub fn num_to_corner(num: i32) -> Vector2 {
    match num % 4 {
        1 => Vector2(0.00, 0.00),
        2 => Vector2(1.00, 0.00),
        3 => Vector2(1.00, 1.00),
        0 => Vector2(0.00, 1.00),
        _ => Vector2::ZERO,
    }
}

pub fn num_to_side(num: i32) -> Vector2 {
    match num % 4 {
        1 => Vector2(1.00, 0.00),
        2 => Vector2(-1.00, 0.00),
        3 => Vector2(0.00, 1.00),
        0 => Vector2(0.00, -1.00),
        _ => Vector2::ZERO,
    }
}

pub fn spawn_pos_vel(side_buffer: f64, edge_buffer: f64) -> (Vector2, Vector2) {
    let direction = get_rand_dir();
    pos_vel(direction, edge_buffer, side_buffer)
}

pub fn spawn_pos_vel_from(side: i32, side_buffer: f64, edge_buffer: f64) -> (Vector2, Vector2) {
    let direction = num_to_side(side);
    pos_vel(direction, edge_buffer, side_buffer)
}

pub fn pos_vel(direction: Vector2, edge_buffer: f64, side_buffer: f64) -> (Vector2, Vector2) {
    let shift = get_shift(direction, edge_buffer);
    let buffer = direction * side_buffer;
    let pos = CENTER + direction.mul_per(CENTER) + buffer;
    (pos + shift, direction.negate())
}

pub fn get_shift(dir: Vector2, edge_buffer: f64) -> Vector2 {
    let rot_dir = dir.rotate_once();
    let shift_range = rot_dir.mul_per(CENTER).len() - edge_buffer;
    rot_dir * (rand(shift_range * 2.00) - shift_range)
}