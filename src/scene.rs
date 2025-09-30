// scene.rs
use raylib::prelude::*;
use crate::block::Block;
use crate::block_types::BlockType;
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
        "textures/deepslate_bricks.jpg",
        "textures/glass.png"
    ];

    for path in textures {
        tex_mgr.load_texture(rl, thread, path)?;
    }

    Ok(())
}

/// Crea una escena sencilla usando los tipos de bloques predefinidos
pub fn create_optimized_scene() -> Vec<Block> {
    let mut blocks = Vec::new();

    // Suelo de 3x3 césped
    for x in -1..=1 {
        for z in -1..=1 {
            blocks.push(BlockType::Grass.to_block(Vector3::new(x as f32, -1.0, z as f32), 1.0));
        }
    }

    // Casa con paredes de cobble y techo de madera
    blocks.push(BlockType::Cobble.to_block(Vector3::new(-1.0, 0.0, -1.0), 1.0));
    blocks.push(BlockType::Cobble.to_block(Vector3::new(1.0, 0.0, -1.0), 1.0));
    blocks.push(BlockType::Cobble.to_block(Vector3::new(-1.0, 0.0, 1.0), 1.0));
    blocks.push(BlockType::Cobble.to_block(Vector3::new(1.0, 0.0, 1.0), 1.0));

    blocks.push(BlockType::WoodPlank.to_block(Vector3::new(-1.0, 1.0, -1.0), 1.0));
    blocks.push(BlockType::WoodPlank.to_block(Vector3::new(1.0, 1.0, -1.0), 1.0));
    blocks.push(BlockType::WoodPlank.to_block(Vector3::new(-1.0, 1.0, 1.0), 1.0));
    blocks.push(BlockType::WoodPlank.to_block(Vector3::new(1.0, 1.0, 1.0), 1.0));

    // Torre de ladrillos deepslate
    for y in 0..4 {
        blocks.push(BlockType::DeepslateBricks.to_block(Vector3::new(3.0, y as f32, 0.0), 1.0));
    }

    // Camino de piedra
    blocks.push(BlockType::Stone.to_block(Vector3::new(0.0, -1.0, -2.0), 1.0));
    blocks.push(BlockType::Stone.to_block(Vector3::new(1.0, -1.0, -2.0), 1.0));
    blocks.push(BlockType::Stone.to_block(Vector3::new(2.0, -1.0, -1.0), 1.0));

    // Jardín
    blocks.push(BlockType::Dirt.to_block(Vector3::new(-3.0, -1.0, 0.0), 1.0));
    blocks.push(BlockType::Dirt.to_block(Vector3::new(-3.0, -1.0, 1.0), 1.0));
    blocks.push(BlockType::Leaves.to_block(Vector3::new(-3.0, 0.0, 0.0), 1.0));
    blocks.push(BlockType::Leaves.to_block(Vector3::new(-3.0, 0.0, 1.0), 1.0));
    blocks.push(BlockType::Leaves.to_block(Vector3::new(-3.0, 1.0, 0.0), 1.0));

    // Elementos especiales
    blocks.push(BlockType::Metal.to_block(Vector3::new(0.0, 2.0, 0.0), 1.0));
    blocks.push(BlockType::Glass.to_block(Vector3::new(-2.0, 0.0, -2.0), 1.0));
    blocks.push(BlockType::WoodLog.to_block(Vector3::new(2.0, 0.0, 2.0), 1.0));

    blocks.push(BlockType::Sun.to_block(Vector3::new(0.0, 10.0, 0.0), 5.0));

    println!("Escena creada con {} bloques", blocks.len());
    blocks
}
