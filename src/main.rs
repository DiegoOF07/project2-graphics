use raylib::prelude::*;
use crate::material::Material;
use crate::framebuffer::{Framebuffer, color_to_u32};
use crate::light::Light;
use crate::snell::trace_ray;
use crate::material::vector3_to_color;
use crate::block::Block;
use crate::textures::TextureManager;

mod block;
mod material;
mod ray_intersect;
mod framebuffer;
mod light;
mod snell;
mod textures;
mod camera;

fn main() {
    let screen_width = 800;
    let screen_height = 600;

    let (mut rl, thread) = raylib::init()
        .size(screen_width, screen_height)
        .title("Minecraft Raytracer - Bloques con Texturas")
        .build();

    rl.set_target_fps(60);

    let mut framebuffer = Framebuffer::new(screen_width as u32, screen_height as u32);
    let mut texture_manager = TextureManager::new();

    // Cargar todas las texturas necesarias
    load_minecraft_textures(&mut texture_manager, &mut rl, &thread);

    // Configuración de cámara inicial
    let mut camera_pos = Vector3::new(0.0, 3.0, -8.0);
    let mut camera_yaw = std::f32::consts::FRAC_PI_2; // Mirar hacia el centro
    let mut camera_pitch = -0.3_f32; // Mirar ligeramente hacia abajo
    let fov: f32 = std::f32::consts::FRAC_PI_3;
    let aspect_ratio = screen_width as f32 / screen_height as f32;

    // Crear escena tipo diorama de Minecraft
    let scene = create_minecraft_diorama();

    // Configurar múltiples luces para mejor iluminación
    let lights = vec![
        Light::new(
            Vector3::new(8.0, 10.0, -8.0),  // Luz principal (sol)
            Vector3::new(1.0, 0.95, 0.8),   // Ligeramente cálida
            1.2,
        ),
        Light::new(
            Vector3::new(-5.0, 6.0, 5.0),   // Luz secundaria
            Vector3::new(0.7, 0.8, 1.0),    // Ligeramente azulada
            0.6,
        ),
    ];

    println!("Controles:");
    println!("WASD - Mover cámara");
    println!("Flechas - Rotar cámara");
    println!("Espacio/Ctrl - Subir/Bajar");
    println!("ESC - Salir");

    while !rl.window_should_close() {
        // Controles de cámara mejorados
        handle_camera_input(&rl, &mut camera_pos, &mut camera_yaw, &mut camera_pitch);

        // Limpiar framebuffer con color del cielo
        framebuffer.clear(color_to_u32(Color::new(135, 206, 235, 255))); // Sky blue

        // Calcular vectores de la cámara
        let forward = Vector3::new(
            camera_yaw.cos() * camera_pitch.cos(),
            camera_pitch.sin(),
            camera_yaw.sin() * camera_pitch.cos(),
        ).normalized();
        let right = forward.cross(Vector3::new(0.0, 1.0, 0.0)).normalized();
        let up = right.cross(forward).normalized();

        // Renderizar cada pixel
        for y in 0..screen_height {
            for x in 0..screen_width {
                let px = (2.0 * ((x as f32 + 0.5) / screen_width as f32) - 1.0)
                    * (fov / 2.0).tan() * aspect_ratio;
                let py = (1.0 - 2.0 * ((y as f32 + 0.5) / screen_height as f32))
                    * (fov / 2.0).tan();

                let ray_dir = (forward + right * px + up * py).normalized();
                
                // Usar múltiples luces para iluminación más realista
                let mut final_color = Vector3::zero();
                for light in &lights {
                    let color_contrib = trace_ray(
                        camera_pos, 
                        ray_dir, 
                        0, 
                        4, // Aumentar profundidad para mejores reflexiones
                        &scene, 
                        light, 
                        &texture_manager
                    );
                    final_color = final_color + color_contrib * light.intensity;
                }
                
                // Normalizar el color final
                final_color = final_color / lights.len() as f32;
                final_color = Vector3::new(
                    final_color.x.min(1.0),
                    final_color.y.min(1.0),
                    final_color.z.min(1.0),
                );

                let color = vector3_to_color(final_color);
                framebuffer.set_pixel(x as u32, y as u32, color_to_u32(color));
            }
        }

        // Obtener información para mostrar antes de usar rl mutably
        let fps = rl.get_fps();
        let pos_text = format!("Pos: ({:.1}, {:.1}, {:.1})", camera_pos.x, camera_pos.y, camera_pos.z);
        
        // Presentar el frame y mostrar información
        {
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::BLACK);
            framebuffer.present(&mut d, &thread);
            d.draw_text(&format!("FPS: {}", fps), 10, 10, 20, Color::WHITE);
            d.draw_text(&pos_text, 10, 35, 16, Color::WHITE);
        }
    }
}

fn load_minecraft_textures(texture_manager: &mut TextureManager, rl: &mut RaylibHandle, thread: &RaylibThread) {
    println!("Cargando texturas...");
    
    // Lista de texturas a cargar (puedes crear estas imágenes o descargar texturas de Minecraft)
    let textures = [
        "textures/grass_top.jpg",
        "textures/grass_side.jpg", 
        "textures/dirt.jpg",
        "textures/stone.jpg",
        "textures/cobble.png",
        "textures/wood_oak.jpg",
        "textures/wood_oak_log.jpg",
        "textures/leaves_oak.jpg",
        "assets/textures/sand.png",
        "assets/textures/water.png",
        "assets/textures/glass.png",
        "assets/textures/brick.png",
    ];

    for texture_path in &textures {
        match texture_manager.load_texture(rl, thread, texture_path) {
            Ok(_) => println!("Cargada: {}", texture_path),
            Err(e) => println!("Error cargando {}: {}", texture_path, e),
        }
    }
    
    println!("Texturas cargadas!");
}

fn create_minecraft_diorama() -> Vec<Block> {
    let mut blocks = Vec::new();
    
    // === SUELO BASE DE TIERRA ===
    for x in -6..=6 {
        for z in -6..=6 {
            let dirt_material = Material::new(
                Vector3::new(0.4, 0.3, 0.2), // Color marrón tierra
                [0.8, 0.1],
                5.0,
                0.0,
                0.0,
                1.0,
                Some("textures/dirt.jpg".to_string()),
                None,
            );
            
            blocks.push(Block::new(
                Vector3::new(x as f32, -2.0, z as f32),
                1.0,
                dirt_material,
            ));
        }
    }

    // === CAPA DE CÉSPED ===
    for x in -5..=5 {
        for z in -5..=5 {
            let grass_material = Material::new(
                Vector3::new(0.3, 0.8, 0.2), // Verde césped
                [0.9, 0.1],
                8.0,
                0.0,
                0.0,
                1.0,
                Some("textures/grass_top.jpg".to_string()),
                None,
            );
            
            blocks.push(Block::new(
                Vector3::new(x as f32, -1.0, z as f32),
                1.0,
                grass_material,
            ));
        }
    }

    // === CASA SIMPLE ===
    // Paredes de piedra
    let stone_material = Material::new(
        Vector3::new(0.6, 0.6, 0.6),
        [0.7, 0.3],
        20.0,
        0.1,
        0.0,
        1.0,
        Some("textures/cobble.png".to_string()),
        None,
    );

    // Pared frontal
    for x in -1..=1 {
        for y in 0..=2 {
            if !(x == 0 && y == 0) { // Dejar espacio para la puerta
                blocks.push(Block::new(
                    Vector3::new(x as f32, y as f32, 2.0),
                    1.0,
                    stone_material.clone(),
                ));
            }
        }
    }

    // Paredes laterales
    for z in 0..=2 {
        for y in 0..=2 {
            blocks.push(Block::new(Vector3::new(-1.0, y as f32, z as f32), 1.0, stone_material.clone()));
            blocks.push(Block::new(Vector3::new(1.0, y as f32, z as f32), 1.0, stone_material.clone()));
        }
    }

    // Pared trasera
    for x in -1..=1 {
        for y in 0..=2 {
            blocks.push(Block::new(
                Vector3::new(x as f32, y as f32, 0.0),
                1.0,
                stone_material.clone(),
            ));
        }
    }

    // === TECHO DE MADERA ===
    let wood_material = Material::new(
        Vector3::new(0.8, 0.5, 0.2),
        [0.8, 0.2],
        10.0,
        0.0,
        0.0,
        1.0,
        Some("textures/wood_oak_log.jpg".to_string()),
        None,
    );

    for x in -1..=1 {
        for z in 0..=2 {
            blocks.push(Block::new(
                Vector3::new(x as f32, 3.0, z as f32),
                1.0,
                wood_material.clone(),
            ));
        }
    }

    // === ÁRBOL ===
    // Tronco
    let log_material = Material::new(
        Vector3::new(0.4, 0.3, 0.1),
        [0.8, 0.2],
        5.0,
        0.0,
        0.0,
        1.0,
        Some("textures/wood_oak.jpg".to_string()),
        None,
    );

    for y in 0..=4 {
        blocks.push(Block::new(
            Vector3::new(4.0, y as f32, -2.0),
            1.0,
            log_material.clone(),
        ));
    }

    // Hojas
    let leaves_material = Material::new(
        Vector3::new(0.2, 0.6, 0.2),
        [0.9, 0.1],
        5.0,
        0.0,
        0.0,
        1.0,
        Some("textures/leaves_oak.jpg".to_string()),
        None,
    );

    // Copa del árbol
    for x in 3..=5 {
        for y in 4..=6 {
            for z in -3..=-1 {
                // Crear forma más orgánica del árbol
                let distance_from_center = ((x - 4) * (x - 4) + (z + 2) * (z + 2)) as f32;
                if distance_from_center <= 2.0 || (y == 4 && distance_from_center <= 4.0) {
                    blocks.push(Block::new(
                        Vector3::new(x as f32, y as f32, z as f32),
                        1.0,
                        leaves_material.clone(),
                    ));
                }
            }
        }
    }

    // === PEQUEÑO LAGO ===
    let water_material = Material::new(
        Vector3::new(0.1, 0.3, 0.8),
        [0.2, 0.8],
        100.0,
        0.7, // Muy reflectante
        0.3, // Ligeramente transparente
        1.33, // Índice de refracción del agua
        Some("textures/water.png".to_string()),
        None,
    );

    for x in -4..=-2 {
        for z in 3..=5 {
            blocks.push(Block::new(
                Vector3::new(x as f32, -1.5, z as f32),
                1.0,
                water_material.clone(),
            ));
        }
    }

    // === ALGUNOS BLOQUES DECORATIVOS ===
    // Bloque de vidrio brillante
    let glass_material = Material::new(
        Vector3::new(0.9, 0.9, 0.9),
        [0.1, 0.9],
        200.0,
        0.1,
        0.8, // Muy transparente
        1.5, // Índice de refracción del vidrio
        Some("textures/glass.png".to_string()),
        None,
    );

    blocks.push(Block::new(
        Vector3::new(-4.0, 0.0, -4.0),
        1.0,
        glass_material,
    ));

    // Torre de ladrillos
    let brick_material = Material::new(
        Vector3::new(0.7, 0.3, 0.2),
        [0.8, 0.2],
        15.0,
        0.0,
        0.0,
        1.0,
        Some("textures/brick.png".to_string()),
        None,
    );

    for y in 0..=3 {
        blocks.push(Block::new(
            Vector3::new(5.0, y as f32, 4.0),
            1.0,
            brick_material.clone(),
        ));
    }

    blocks
}

fn handle_camera_input(rl: &RaylibHandle, camera_pos: &mut Vector3, camera_yaw: &mut f32, camera_pitch: &mut f32) {
    // Rotación de cámara
    let rotation_speed = if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) { 0.03 } else { 0.015 };
    
    if rl.is_key_down(KeyboardKey::KEY_LEFT) { *camera_yaw -= rotation_speed; }
    if rl.is_key_down(KeyboardKey::KEY_RIGHT) { *camera_yaw += rotation_speed; }
    if rl.is_key_down(KeyboardKey::KEY_UP) { *camera_pitch += rotation_speed; }
    if rl.is_key_down(KeyboardKey::KEY_DOWN) { *camera_pitch -= rotation_speed; }
    
    *camera_pitch = camera_pitch.clamp(-1.4, 1.4); // Limitar pitch

    // Movimiento de cámara
    let forward = Vector3::new(
        camera_yaw.cos() * camera_pitch.cos(),
        camera_pitch.sin(),
        camera_yaw.sin() * camera_pitch.cos(),
    ).normalized();
    let right = forward.cross(Vector3::new(0.0, 1.0, 0.0)).normalized();
    let up = Vector3::new(0.0, 1.0, 0.0);

    let speed = if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) { 0.2 } else { 0.08 };
    
    if rl.is_key_down(KeyboardKey::KEY_W) { *camera_pos += forward * speed; }
    if rl.is_key_down(KeyboardKey::KEY_S) { *camera_pos -= forward * speed; }
    if rl.is_key_down(KeyboardKey::KEY_A) { *camera_pos -= right * speed; }
    if rl.is_key_down(KeyboardKey::KEY_D) { *camera_pos += right * speed; }
    if rl.is_key_down(KeyboardKey::KEY_SPACE) { *camera_pos += up * speed; }
    if rl.is_key_down(KeyboardKey::KEY_LEFT_CONTROL) { *camera_pos -= up * speed; }
}