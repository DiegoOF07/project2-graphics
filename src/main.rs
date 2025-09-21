use raylib::prelude::*;
use crate::cube::Cube;
use crate::material::Material;
use crate::framebuffer::{Framebuffer, color_to_u32};
use crate::light::Light;
use crate::snell::trace_ray;
use crate::material::vector3_to_color;

mod cube;
mod material;
mod ray_intersect;
mod framebuffer;
mod light;
mod snell;

fn main() {
    let screen_width = 800;
    let screen_height = 600;

    let (mut rl, thread) = raylib::init()
        .size(screen_width, screen_height)
        .title("Mini Ray Tracer - Multiple Cubes with Reflection")
        .build();

    // Framebuffer
    let mut framebuffer = Framebuffer::new(screen_width as u32, screen_height as u32);

    // Cámara en coordenadas esféricas
    let mut camera_pos = Vector3::new(0.0, 1.0, -6.0); // posición inicial
    let mut camera_yaw = 0.0_f32;   // ángulo horizontal
    let mut camera_pitch = 0.0_f32; // ángulo vertical
    let fov: f32 = std::f32::consts::FRAC_PI_3; // 60°
    let aspect_ratio = screen_width as f32 / screen_height as f32;

    // Escena con múltiples cubos
    let scene = vec![
        // Cubo celeste (difuso)
        Cube::new(
            Vector3::new(-1.5, 0.0, 0.0),
            2.0,
            Material::new(
                Vector3::new(0.2, 0.7, 0.9), // color difuso
                [0.6, 0.4],
                50.0,
                0.0,   // reflectividad = 0
                0.0,
                1.0,
                None,
                None,
            ),
        ),
        // Cubo reflectivo
        Cube::new(
            Vector3::new(2.0, 0.0, 0.0),
            2.0,
            Material::new(
                Vector3::new(0.8, 0.8, 0.8), // gris
                [0.6, 0.4],
                100.0,
                0.8,   // reflectividad alta
                0.0,
                1.0,
                None,
                None,
            ),
        ),
    ];

    // Una luz puntual
    let light = Light::new(
        Vector3::new(5.0, 5.0, -5.0),
        Vector3::new(1.0, 1.0, 1.0),
        1.0,
    );

    // Loop principal
    while !rl.window_should_close() {
        // === Entrada ===
        // Rotación de cámara con flechas
        if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            camera_yaw -= 0.02;
        }
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            camera_yaw += 0.02;
        }
        if rl.is_key_down(KeyboardKey::KEY_UP) {
            camera_pitch += 0.02;
        }
        if rl.is_key_down(KeyboardKey::KEY_DOWN) {
            camera_pitch -= 0.02;
        }
        camera_pitch = camera_pitch.clamp(-1.5, 1.5);

        // Dirección hacia adelante en base a yaw y pitch
        let forward = Vector3::new(
            camera_yaw.cos() * camera_pitch.cos(),
            camera_pitch.sin(),
            camera_yaw.sin() * camera_pitch.cos(),
        ).normalized();

        let right = forward.cross(Vector3::new(0.0, 1.0, 0.0)).normalized();
        let up = right.cross(forward).normalized();

        // Movimiento con WASD + SPACE/CTRL
        let speed = 0.1;
        if rl.is_key_down(KeyboardKey::KEY_W) {
            camera_pos += forward * speed;
        }
        if rl.is_key_down(KeyboardKey::KEY_S) {
            camera_pos -= forward * speed;
        }
        if rl.is_key_down(KeyboardKey::KEY_A) {
            camera_pos -= right * speed;
        }
        if rl.is_key_down(KeyboardKey::KEY_D) {
            camera_pos += right * speed;
        }
        if rl.is_key_down(KeyboardKey::KEY_SPACE) {
            camera_pos += up * speed;
        }
        if rl.is_key_down(KeyboardKey::KEY_LEFT_CONTROL) {
            camera_pos -= up * speed;
        }

        // === Raytracing por píxel ===
        framebuffer.clear(color_to_u32(Color::new(50, 50, 80, 255))); // fondo azul oscuro

        for y in 0..screen_height {
            for x in 0..screen_width {
                let px = (2.0 * ((x as f32 + 0.5) / screen_width as f32) - 1.0)
                    * (fov / 2.0).tan() * aspect_ratio;
                let py = (1.0 - 2.0 * ((y as f32 + 0.5) / screen_height as f32))
                    * (fov / 2.0).tan();

                // Dirección de rayo desde cámara hacia el plano de proyección
                let ray_dir = (forward + right * px + up * py).normalized();

                let color_vec = trace_ray(camera_pos, ray_dir, 0, 3, &scene, &light);
                let color = vector3_to_color(color_vec);
                framebuffer.set_pixel(x as u32, y as u32, color_to_u32(color));
            }
        }

        // === Dibujar ===
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        framebuffer.present(&mut d, &thread);
    }
}
