use raylib::prelude::*;

pub fn handle_camera_input(
    rl: &RaylibHandle,
    pos: &mut Vector3,
    yaw: &mut f32,
    pitch: &mut f32,
) {
    let move_speed = 0.1;
    let rot_speed = 0.03;

    // Dirección hacia adelante según yaw y pitch
    let forward = Vector3::new(yaw.cos(), 0.0, yaw.sin());
    let right = Vector3::new(-yaw.sin(), 0.0, yaw.cos());

    // Movimiento con WASD
    if rl.is_key_down(KeyboardKey::KEY_W) {
        *pos += forward * move_speed;
    }
    if rl.is_key_down(KeyboardKey::KEY_S) {
        *pos -= forward * move_speed;
    }
    if rl.is_key_down(KeyboardKey::KEY_A) {
        *pos -= right * move_speed;
    }
    if rl.is_key_down(KeyboardKey::KEY_D) {
        *pos += right * move_speed;
    }

    // Subir / Bajar
    if rl.is_key_down(KeyboardKey::KEY_SPACE) {
        pos.y += move_speed;
    }
    if rl.is_key_down(KeyboardKey::KEY_LEFT_CONTROL) {
        pos.y -= move_speed;
    }

    // Rotación con flechas
    if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
        *yaw += rot_speed;
    }
    if rl.is_key_down(KeyboardKey::KEY_LEFT) {
        *yaw -= rot_speed;
    }
    if rl.is_key_down(KeyboardKey::KEY_UP) {
        *pitch += rot_speed;
    }
    if rl.is_key_down(KeyboardKey::KEY_DOWN) {
        *pitch -= rot_speed;
    }

    // Limitar pitch para no voltear de más
    let limit = std::f32::consts::FRAC_PI_2 - 0.1;
    if *pitch > limit {
        *pitch = limit;
    }
    if *pitch < -limit {
        *pitch = -limit;
    }
}
