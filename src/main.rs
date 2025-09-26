// === Imports ===
use std::sync::Arc;
use std::thread;

use raylib::prelude::*;

use crate::block::Block;
use crate::framebuffer::{color_to_u32, Framebuffer};
use crate::light::Light;
use crate::material::{vector3_to_color};
use crate::snell::trace_ray_multi_light;
use crate::textures::TextureManager;
use crate::events::handle_camera_input;
use crate::scene::{create_optimized_scene, load_minecraft_textures};


mod block;
mod camera;
mod framebuffer;
mod light;
mod material;
mod ray_intersect;
mod snell;
mod textures;
mod events;
mod scene;
mod block_types;

// === Constantes globales ===
const SCREEN_WIDTH: i32 = 400;
const SCREEN_HEIGHT: i32 = 300;
const RENDER_SCALE: i32 = 2;

fn main() {
    // Inicialización de ventana y Raylib
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH * RENDER_SCALE, SCREEN_HEIGHT * RENDER_SCALE)
        .title("Minecraft Raytracer")
        .build();
    rl.set_target_fps(60);

    // Framebuffer y texturas
    let mut framebuffer = Framebuffer::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32);
    let mut texture_manager = TextureManager::new();
    load_minecraft_textures(&mut rl, &thread, &mut texture_manager);

    // Cámara
    let mut camera_pos = Vector3::new(0.0, 2.0, -6.0);
    let mut camera_yaw = 0.0_f32;
    let mut camera_pitch = -0.2_f32;
    let fov: f32 = std::f32::consts::FRAC_PI_3;
    let aspect_ratio = SCREEN_WIDTH as f32 / SCREEN_HEIGHT as f32;

    // Escena y recursos compartidos
    let scene = Arc::new(create_optimized_scene());
    let lights = Arc::new(vec![
        Light::new(
            Vector3::new(5.0, 8.0, -5.0),   // Luz principal (sol)
            Vector3::new(1.0, 0.95, 0.8),   // Cálida
            2.2,
        ),
        Light::new(
            Vector3::new(-5.0, 6.0, 5.0),   // Luz secundaria
            Vector3::new(0.6, 0.7, 1.0),    // Fría/azulada
            0.8,
        ),
        Light::new(
            Vector3::new(0.0, 6.0, 0.0),    // Luz cenital
            Vector3::new(1.0, 1.0, 0.9),    // Blanca suave
            0.6,
        ),
    ]);
    let texture_manager = Arc::new(texture_manager);

    // Información al usuario
    println!("Controles:");
    println!("WASD - Mover | Flechas - Rotar | Espacio/CTRL - Subir/Bajar | T - Toggle multihilo | ESC - Salir");
    println!(
        "Resolución: {}x{} (escalado {}x)",
        SCREEN_WIDTH, SCREEN_HEIGHT, RENDER_SCALE
    );

    // Variables de estado
    let mut use_multithreading = true;
    let mut frame_count = 0;
    let mut last_fps_update = std::time::Instant::now();

    // === Loop principal ===
    while !rl.window_should_close() {
        // Movimiento de cámara
        handle_camera_input(&rl, &mut camera_pos, &mut camera_yaw, &mut camera_pitch);

        // Toggle multihilo
        if rl.is_key_pressed(KeyboardKey::KEY_T) {
            use_multithreading = !use_multithreading;
            println!("Multihilo: {}", if use_multithreading { "ON" } else { "OFF" });
        }

        framebuffer.clear(color_to_u32(Color::new(135, 206, 250, 255)));

        // Configuración de cámara
        let camera_config = CameraConfig::new(
            camera_pos,
            camera_yaw,
            camera_pitch,
            SCREEN_WIDTH as usize,
            SCREEN_HEIGHT as usize,
            fov,
            aspect_ratio,
        );

        // Render
        let start_time = std::time::Instant::now();
        if use_multithreading {
            render_multithreaded(
                &mut framebuffer,
                &camera_config,
                Arc::clone(&scene),
                Arc::clone(&lights),
                Arc::clone(&texture_manager),
            );
        } else {
            render_single_threaded(
                &mut framebuffer,
                &camera_config,
                &scene,
                &lights,
                &texture_manager,
            );
        }
        let render_time = start_time.elapsed();

        // === Dibujar UI ===
        frame_count += 1;
        let now = std::time::Instant::now();
        let fps_text = if now.duration_since(last_fps_update).as_secs() >= 1 {
            last_fps_update = now;
            let fps = frame_count;
            frame_count = 0;
            format!("FPS: {}", fps)
        } else {
            format!("FPS: {}", rl.get_fps())
        };

        let pos_text =
            format!("Pos: ({:.1}, {:.1}, {:.1})", camera_pos.x, camera_pos.y, camera_pos.z);
        let mode_text = format!(
            "Modo: {}",
            if use_multithreading { "Multi-hilo" } else { "Single-hilo" }
        );
        let render_time_text = format!("Render: {:.1}ms", render_time.as_millis());

        {
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::BLACK);

            let source = Rectangle::new(0.0, 0.0, SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32);
            let dest = Rectangle::new(
                0.0,
                0.0,
                (SCREEN_WIDTH * RENDER_SCALE) as f32,
                (SCREEN_HEIGHT * RENDER_SCALE) as f32,
            );
            framebuffer.present_scaled(&mut d, &thread, source, dest);

            d.draw_text(&fps_text, 10, 10, 20, Color::WHITE);
            d.draw_text(&pos_text, 10, 35, 16, Color::WHITE);
            d.draw_text(&mode_text, 10, 60, 16, Color::WHITE);
            d.draw_text(&render_time_text, 10, 85, 16, Color::WHITE);
            d.draw_text(&format!("Bloques: {}", scene.len()), 10, 110, 16, Color::WHITE);
            d.draw_text("T - Toggle multihilo", 10, 135, 14, Color::LIGHTGRAY);
        }
    }
}

// === Render single thread ===
fn render_single_threaded(
    framebuffer: &mut Framebuffer,
    camera_config: &CameraConfig,
    scene: &[Block],
    lights: &[Light], // Cambio: vector de luces
    texture_manager: &TextureManager,
) {
    for y in 0..camera_config.height {
        for x in 0..camera_config.width {
            let ray_dir = camera_config.get_ray_direction(x, y);
            
            let color_vec = trace_ray_multi_light(
                camera_config.pos, 
                ray_dir, 
                0, 2, 
                scene, 
                lights, // Pasar vector de luces
                texture_manager
            );

            let color = vector3_to_color(color_vec);
            framebuffer.set_pixel(x as u32, y as u32, color_to_u32(color));
        }
    }
}

// === Render multi-thread con múltiples luces ===
fn render_multithreaded(
    framebuffer: &mut Framebuffer,
    camera_config: &CameraConfig,
    scene: Arc<Vec<Block>>,
    lights: Arc<Vec<Light>>, // Cambio: vector de luces
    texture_manager: Arc<TextureManager>,
) {
    let num_threads = thread::available_parallelism().unwrap().get();
    let tile_size = 16usize;

    // Crear tiles
    let mut tiles = Vec::new();
    for ty in (0..camera_config.height).step_by(tile_size) {
        for tx in (0..camera_config.width).step_by(tile_size) {
            let x2 = (tx + tile_size).min(camera_config.width);
            let y2 = (ty + tile_size).min(camera_config.height);
            tiles.push((tx, ty, x2, y2));
        }
    }

    // Distribuir tiles entre hilos
    let tiles_per_thread = (tiles.len() + num_threads - 1) / num_threads;
    let mut handles = Vec::new();
    let tiles_arc = Arc::new(tiles);

    for i in 0..num_threads {
        let scene = Arc::clone(&scene);
        let lights = Arc::clone(&lights); // Cambio: clonar vector de luces
        let texture_manager = Arc::clone(&texture_manager);
        let camera = camera_config.clone();
        let tiles_ref = Arc::clone(&tiles_arc);

        let start = i * tiles_per_thread;
        let end = ((i + 1) * tiles_per_thread).min(tiles_ref.len());

        let handle = thread::spawn(move || {
            let mut local_pixels = Vec::new();
            for &(x1, y1, x2, y2) in &tiles_ref[start..end] {
                for y in y1..y2 {
                    for x in x1..x2 {
                        let ray_dir = camera.get_ray_direction(x, y);
                        
                        // Cambio: calcular iluminación con múltiples luces
                        let color_vec = trace_ray_multi_light(
                            camera.pos, 
                            ray_dir, 
                            0, 
                            2, 
                            &scene, 
                            &lights, // Pasar vector de luces
                            &texture_manager
                        );
                        
                        let color_u32 = color_to_u32(vector3_to_color(color_vec));
                        local_pixels.push((x, y, color_u32));
                    }
                }
            }
            local_pixels
        });
        handles.push(handle);
    }

    // Recoger resultados
    for handle in handles {
        if let Ok(local_pixels) = handle.join() {
            for (x, y, c) in local_pixels {
                framebuffer.set_pixel(x as u32, y as u32, c);
            }
        }
    }
}

// === Cámara ===
#[derive(Clone)]
struct CameraConfig {
    pos: Vector3,
    forward: Vector3,
    right: Vector3,
    up: Vector3,
    width: usize,
    height: usize,
    fov_tan: f32,
    aspect_ratio: f32,
}

impl CameraConfig {
    fn new(
        pos: Vector3,
        yaw: f32,
        pitch: f32,
        width: usize,
        height: usize,
        fov: f32,
        aspect_ratio: f32,
    ) -> Self {
        let forward = Vector3::new(
            yaw.cos() * pitch.cos(),
            pitch.sin(),
            yaw.sin() * pitch.cos(),
        )
        .normalized();
        let right = forward.cross(Vector3::new(0.0, 1.0, 0.0)).normalized();
        let up = right.cross(forward).normalized();
        Self {
            pos,
            forward,
            right,
            up,
            width,
            height,
            fov_tan: (fov / 2.0).tan(),
            aspect_ratio,
        }
    }

    #[inline]
    fn get_ray_direction(&self, x: usize, y: usize) -> Vector3 {
        let px = (2.0 * ((x as f32 + 0.5) / self.width as f32) - 1.0) * self.fov_tan * self.aspect_ratio;
        let py = (1.0 - 2.0 * ((y as f32 + 0.5) / self.height as f32)) * self.fov_tan;
        (self.forward + self.right * px + self.up * py).normalized()
    }
}
