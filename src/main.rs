// main.rs
use raylib::prelude::*;
use crate::cube::Cube;
use crate::material::{Material, vector3_to_color};
use crate::ray_intersect::RayIntersect;
use crate::framebuffer::Framebuffer;

mod cube;
mod material;
mod ray_intersect;
mod framebuffer;

fn main() {
    let screen_width = 800;
    let screen_height = 600;

    // Inicializar Raylib
    let (mut rl, thread) = raylib::init()
        .size(screen_width, screen_height)
        .title("Mini Ray Tracer - Cube Scene")
        .build();

    // Crear framebuffer con color de fondo negro
    let mut fb = Framebuffer::new(screen_width as u32, screen_height as u32);

    // Cámara simple
    let fov: f32 = std::f32::consts::FRAC_PI_3; // 60 grados
    let aspect_ratio = screen_width as f32 / screen_height as f32;
    let camera_origin = Vector3::new(0.0, 0.0, -5.0);

    // Escena: un solo cubo
    let cube = Cube::new(
        Vector3::new(0.0, 0.0, 0.0),
        2.0,
        Material::new(
            Vector3::new(0.2, 0.7, 0.9), // Azul celeste
            [0.6, 0.4],                  // mezcla luz/objeto
            50.0,                        // brillo
            0.1,                         // reflectividad
            0.0,                         // transparencia
            1.0,                         // índice refracción
            None,
            None,
        ),
    );

    // ================================
    // Generar rayos y rellenar framebuffer
    // ================================
    for y in 0..screen_height {
        for x in 0..screen_width {
            // Coordenadas de píxel normalizadas [-1, 1]
            let px = (2.0 * ((x as f32 + 0.5) / screen_width as f32) - 1.0) 
                     * (fov / 2.0).tan() * aspect_ratio;
            let py = (1.0 - 2.0 * ((y as f32 + 0.5) / screen_height as f32)) 
                     * (fov / 2.0).tan();

            let ray_dir = Vector3::new(px, py, 1.0).normalized();

            // Intersección con el cubo
            let intersect = cube.ray_intersect(&camera_origin, &ray_dir);

            // Color según intersección
            let color = if intersect.is_intersecting {
                vector3_to_color(intersect.material.diffuse)
            } else {
                // color de fondo
                Color::new(30, 30, 30, 255) // gris oscuro
            };

            fb.set_pixel(x as u32, y as u32, framebuffer::color_to_u32(color));
        }
    }

    // ================================
    // Loop principal de Raylib
    // ================================
    while !rl.window_should_close() {
        fb.present(&mut rl, &thread);
    }
}
