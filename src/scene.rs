// scene.rs
use raylib::prelude::*;
use crate::block::Block;
use crate::material::Material;
use crate::textures::TextureManager;

/// Carga las texturas que vamos a usar en los bloques estilo Minecraft
pub fn load_minecraft_textures(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    tex_mgr: &mut TextureManager,
) -> Result<(), String> {
    let textures = vec![
        "textures/grass_top.jpg",
        "textures/dirt.jpg",
        "textures/stone.jpg",
        "textures/cobble.png",
        "textures/wood_oak.jpg",
        "textures/wood_oak_log.jpg",
        "textures/leaves_oak.jpg",
    ];

    for path in textures {
        tex_mgr.load_texture(rl, thread, path)?;
    }

    Ok(())
}

/// Crea una escena tipo pueblo pequeño con ~32 bloques variados
pub fn create_optimized_scene() -> Vec<Block> {
    let mut blocks = Vec::new();

    // === SUELO BASE (9 bloques) ===
    // Crear un suelo de 3x3 con césped
    for x in -1..=1 {
        for z in -1..=1 {
            blocks.push(Block::new(
                Vector3::new(x as f32, -1.0, z as f32),
                1.0,
                Material {
                    diffuse: Vector3::new(0.4, 0.8, 0.3),
                    albedo: [0.9, 0.1],
                    specular: 5.0,
                    reflectivity: 0.0,
                    transparency: 0.0,
                    refractive_index: 1.0,
                    texture: Some("textures/grass_top.jpg".to_string()),
                    normal_map_id: None,
                },
            ));
        }
    }

    // === CASA PRINCIPAL (8 bloques) ===
    // Estructura básica de casa con paredes y techo
    let stone_material = Material {
        diffuse: Vector3::new(0.6, 0.6, 0.6),
        albedo: [0.8, 0.2],
        specular: 15.0,
        reflectivity: 0.1,
        transparency: 0.0,
        refractive_index: 1.0,
        texture: Some("textures/cobble.png".to_string()),
        normal_map_id: None,
    };

    // Paredes (4 bloques)
    blocks.push(Block::new(Vector3::new(-1.0, 0.0, -1.0), 1.0, stone_material.clone())); // Esquina
    blocks.push(Block::new(Vector3::new(1.0, 0.0, -1.0), 1.0, stone_material.clone()));  // Esquina
    blocks.push(Block::new(Vector3::new(-1.0, 0.0, 1.0), 1.0, stone_material.clone()));  // Esquina
    blocks.push(Block::new(Vector3::new(1.0, 0.0, 1.0), 1.0, stone_material.clone()));   // Esquina

    // Techo de madera (4 bloques)
    let wood_material = Material {
        diffuse: Vector3::new(0.8, 0.5, 0.2),
        albedo: [0.8, 0.2],
        specular: 8.0,
        reflectivity: 0.0,
        transparency: 0.0,
        refractive_index: 1.0,
        texture: Some("textures/wood_oak.jpg".to_string()),
        normal_map_id: None,
    };

    blocks.push(Block::new(Vector3::new(-1.0, 1.0, -1.0), 1.0, wood_material.clone()));
    blocks.push(Block::new(Vector3::new(1.0, 1.0, -1.0), 1.0, wood_material.clone()));
    blocks.push(Block::new(Vector3::new(-1.0, 1.0, 1.0), 1.0, wood_material.clone()));
    blocks.push(Block::new(Vector3::new(1.0, 1.0, 1.0), 1.0, wood_material.clone()));

    // === TORRE DE OBSERVACIÓN (4 bloques) ===
    let brick_material = Material {
        diffuse: Vector3::new(0.7, 0.3, 0.2),
        albedo: [0.8, 0.2],
        specular: 12.0,
        reflectivity: 0.05,
        transparency: 0.0,
        refractive_index: 1.0,
        texture: Some("textures/brick.png".to_string()),
        normal_map_id: None,
    };

    blocks.push(Block::new(Vector3::new(3.0, 0.0, 0.0), 1.0, brick_material.clone()));
    blocks.push(Block::new(Vector3::new(3.0, 1.0, 0.0), 1.0, brick_material.clone()));
    blocks.push(Block::new(Vector3::new(3.0, 2.0, 0.0), 1.0, brick_material.clone()));
    blocks.push(Block::new(Vector3::new(3.0, 3.0, 0.0), 1.0, brick_material.clone()));

    // === CAMINO DE PIEDRA (3 bloques) ===
    let path_material = Material {
        diffuse: Vector3::new(0.5, 0.5, 0.5),
        albedo: [0.9, 0.1],
        specular: 3.0,
        reflectivity: 0.0,
        transparency: 0.0,
        refractive_index: 1.0,
        texture: Some("textures/stone.jpg".to_string()),
        normal_map_id: None,
    };

    blocks.push(Block::new(Vector3::new(0.0, -0.9, -2.0), 1.0, path_material.clone()));
    blocks.push(Block::new(Vector3::new(1.0, -0.9, -2.0), 1.0, path_material.clone()));
    blocks.push(Block::new(Vector3::new(2.0, -0.9, -1.0), 1.0, path_material.clone()));

    // === JARDÍN CON DIFERENTES MATERIALES (5 bloques) ===
    // Bloque de tierra
    let dirt_material = Material {
        diffuse: Vector3::new(0.4, 0.3, 0.2),
        albedo: [0.9, 0.1],
        specular: 2.0,
        reflectivity: 0.0,
        transparency: 0.0,
        refractive_index: 1.0,
        texture: Some("textures/dirt.jpg".to_string()),
        normal_map_id: None,
    };

    blocks.push(Block::new(Vector3::new(-3.0, -1.0, 0.0), 1.0, dirt_material.clone()));
    blocks.push(Block::new(Vector3::new(-3.0, -1.0, 1.0), 1.0, dirt_material.clone()));

    // Hojas/Plantas
    let leaves_material = Material {
        diffuse: Vector3::new(0.2, 0.6, 0.2),
        albedo: [0.9, 0.1],
        specular: 3.0,
        reflectivity: 0.0,
        transparency: 0.0,
        refractive_index: 1.0,
        texture: Some("textures/leaves_oak.jpg".to_string()),
        normal_map_id: None,
    };

    blocks.push(Block::new(Vector3::new(-3.0, 0.0, 0.0), 1.0, leaves_material.clone()));
    blocks.push(Block::new(Vector3::new(-3.0, 0.0, 1.0), 1.0, leaves_material.clone()));
    blocks.push(Block::new(Vector3::new(-3.0, 1.0, 0.0), 1.0, leaves_material.clone()));

    // === ELEMENTOS ESPECIALES (3 bloques) ===
    // Bloque reflectante (tipo espejo/metal)
    let metal_material = Material {
        diffuse: Vector3::new(0.8, 0.8, 0.9),
        albedo: [0.3, 0.7],
        specular: 100.0,
        reflectivity: 0.8, // Muy reflectante
        transparency: 0.0,
        refractive_index: 1.0,
        texture: Some("textures/stone.jpg".to_string()), // Usar stone como base metálica
        normal_map_id: None,
    };

    blocks.push(Block::new(Vector3::new(0.0, 2.0, 0.0), 1.0, metal_material));

    // Bloque transparente (tipo cristal)
    let glass_material = Material {
        diffuse: Vector3::new(0.9, 0.9, 1.0),
        albedo: [0.1, 0.9],
        specular: 200.0,
        reflectivity: 0.1,
        transparency: 0.8, // Muy transparente
        refractive_index: 1.5, // Índice del vidrio
        texture: Some("textures/glass.png".to_string()),
        normal_map_id: None,
    };

    blocks.push(Block::new(Vector3::new(-2.0, 0.0, -2.0), 1.0, glass_material));

    // Bloque decorativo de madera especial
    let log_material = Material {
        diffuse: Vector3::new(0.4, 0.3, 0.1),
        albedo: [0.8, 0.2],
        specular: 5.0,
        reflectivity: 0.0,
        transparency: 0.0,
        refractive_index: 1.0,
        texture: Some("textures/wood_oak_log.jpg".to_string()),
        normal_map_id: None,
    };

    blocks.push(Block::new(Vector3::new(2.0, 0.0, 2.0), 1.0, log_material));

    println!("Escena creada con {} bloques", blocks.len());
    blocks
}
