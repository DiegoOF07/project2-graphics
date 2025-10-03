// scene.rs - Isla flotante con casa, jardín, árbol y lago
use crate::block::{self, Block};
use crate::block_types::BlockType;
use crate::textures::TextureManager;
use raylib::prelude::*;

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
        "textures/cherry_log.png",
        "textures/cherry_leaves.png",
        "textures/leaves_oak.jpg",
        "textures/glass.png",
        "textures/sand.png",
        "textures/magma.png",
    ];

    for path in textures {
        tex_mgr.load_texture(rl, thread, path)?;
    }

    Ok(())
}

/// Crea una isla flotante estilo Minecraft con casa, jardín, árbol y lago
pub fn create_optimized_scene() -> Vec<Block> {
    let mut blocks = Vec::new();

    // === CAPA BASE DE LA ISLA (césped y tierra) ===
    // Superficie de césped más grande (7x7)
    for x in -3..=3 {
        for z in -3..=3 {
            blocks.push(BlockType::Grass.to_block(Vector3::new(x as f32, 0.0, z as f32), 1.0));
        }
    }

    // === CASA (3x3 con ventanas) ===
    let house_x = -2.0;
    let house_z = -2.0;

    for y in 1..=2 {
        // --- Pared norte (z = -2, puerta en el centro) ---
        for x in 0..=2 {
            let pos = Vector3::new(house_x + x as f32, y as f32, house_z);

            // puerta de 2 bloques (x=1, y=1 y y=2 libres)
            if !(x == 1) {
                blocks.push(BlockType::Cobble.to_block(pos, 1.0));
            }
        }

        // --- Pared sur (z = 0, ventana en el centro) ---
        for x in 0..=2 {
            let pos = Vector3::new(house_x + x as f32, y as f32, house_z + 2.0);

            if x == 1 && y == 2 {
                // ventana sur
                blocks.push(BlockType::Glass.to_block(pos, 1.0));
            } else {
                blocks.push(BlockType::Cobble.to_block(pos, 1.0));
            }
        }

        // --- Pared oeste (x = -2, ventana en el centro) ---
        for z in 0..=2 {
            let pos = Vector3::new(house_x, y as f32, house_z + z as f32);

            if z == 1 && y == 2 {
                // ventana oeste
                blocks.push(BlockType::Glass.to_block(pos, 1.0));
            } else {
                blocks.push(BlockType::Cobble.to_block(pos, 1.0));
            }
        }

        // --- Pared este (x = 0, ventana en el centro) ---
        for z in 0..=2 {
            let pos = Vector3::new(house_x + 2.0, y as f32, house_z + z as f32);

            if z == 1 && y == 2 {
                // ventana este
                blocks.push(BlockType::Glass.to_block(pos, 1.0));
            } else {
                blocks.push(BlockType::Cobble.to_block(pos, 1.0));
            }
        }
    }

    // --- Techo plano de troncos ---
    for x in 0..=2 {
        for z in 0..=2 {
            blocks.push(BlockType::WoodLog.to_block(
                Vector3::new(house_x + x as f32, 3.0, house_z + z as f32),
                1.0,
            ));
        }
    }

    blocks.push(BlockType::Cobble.to_block(Vector3::new(-2.0, 4.0, 0.0), 1.0)); // Chimenea

    // === ÁRBOL EN EL JARDÍN ===
    let tree_x = 2.0;
    let tree_z = -1.0;

    // Tronco (3 bloques de altura)
    for y in 1..=3 {
        blocks.push(BlockType::WoodLog.to_block(Vector3::new(tree_x, y as f32, tree_z), 1.0));
    }

    // Copa de hojas (forma de cruz en nivel 4)
    blocks.push(BlockType::CherryLeaves.to_block(Vector3::new(tree_x, 4.0, tree_z), 1.0)); // Centro
    blocks.push(BlockType::CherryLeaves.to_block(Vector3::new(tree_x + 1.0, 4.0, tree_z), 1.0));
    blocks.push(BlockType::CherryLeaves.to_block(Vector3::new(tree_x - 1.0, 4.0, tree_z), 1.0));
    blocks.push(BlockType::CherryLeaves.to_block(Vector3::new(tree_x, 4.0, tree_z + 1.0), 1.0));
    blocks.push(BlockType::CherryLeaves.to_block(Vector3::new(tree_x, 4.0, tree_z - 1.0), 1.0));

    // Copa superior (nivel 5, más pequeña)
    blocks.push(BlockType::CherryLeaves.to_block(Vector3::new(tree_x, 5.0, tree_z), 1.0));

    // === LAGO 2x2 CON ARENA ALREDEDOR ===
    let lake_center_x = 1.0;
    let lake_center_z = 2.0;

    // Coordenadas relativas de un lago 2x2
    let lake_coords = vec![(0.0, 0.0), (1.0, 0.0), (0.0, 1.0), (1.0, 1.0)];

    // Agua (Glass) en el nivel 0 y -1
    for (dx, dz) in &lake_coords {
        let lx = lake_center_x + dx;
        let lz = lake_center_z + dz;

        // Superficie del agua
        replace_block(
            &mut blocks,
            BlockType::Reflect.to_block(Vector3::new(lx, 0.0, lz), 1.0),
        );
    }

    // Arena alrededor (un anillo de 4x4 menos el lago central)
    for x in -1..=2 {
        for z in -1..=2 {
            let sx = lake_center_x + x as f32;
            let sz = lake_center_z + z as f32;

            // Si NO forma parte del lago 2x2
            if !lake_coords
                .iter()
                .any(|(dx, dz)| sx == lake_center_x + dx && sz == lake_center_z + dz)
            {
                replace_block(
                    &mut blocks,
                    BlockType::Sand.to_block(Vector3::new(sx, 0.0, sz), 1.0),
                );
            }
        }
    }

    blocks.push(BlockType::Leaves.to_block(Vector3::new(3.0, 1.0, 0.0), 1.0));
    blocks.push(BlockType::Leaves.to_block(Vector3::new(2.0, 1.0, 0.0), 1.0));
    blocks.push(BlockType::Leaves.to_block(Vector3::new(1.0, 1.0, 0.0), 1.0));
    
    blocks.push(BlockType::Leaves.to_block(Vector3::new(1.0, 1.0, -1.0), 1.0));
    blocks.push(BlockType::Leaves.to_block(Vector3::new(3.0, 1.0, -1.0), 1.0));

    blocks.push(BlockType::Leaves.to_block(Vector3::new(3.0, 1.0, -2.0), 1.0));
    blocks.push(BlockType::Leaves.to_block(Vector3::new(2.0, 1.0, -2.0), 1.0));
    blocks.push(BlockType::Leaves.to_block(Vector3::new(1.0, 1.0, -2.0), 1.0));

    blocks.push(BlockType::Stone.to_block(Vector3::new(-1.0, 0.0, 4.0), 1.0));
    blocks.push(BlockType::Stone.to_block(Vector3::new(-2.0, 0.0, 4.0), 1.0));
    blocks.push(BlockType::Stone.to_block(Vector3::new(-2.0, 1.0, 4.0), 1.0));
    blocks.push(BlockType::Stone.to_block(Vector3::new(-3.0, 0.0, 4.0), 1.0));
    blocks.push(BlockType::Stone.to_block(Vector3::new(-4.0, 0.0, 4.0), 1.0));
    blocks.push(BlockType::Stone.to_block(Vector3::new(-4.0, 1.0, 4.0), 1.0));
    blocks.push(BlockType::Leaves.to_block(Vector3::new(-3.0, 1.0, 4.0), 1.0));
    blocks.push(BlockType::Leaves.to_block(Vector3::new(-3.0, 1.0, 3.0), 1.0));
    blocks.push(BlockType::Leaves.to_block(Vector3::new(-3.0, 1.0, 2.0), 1.0));
    blocks.push(BlockType::Leaves.to_block(Vector3::new(-3.0, 1.0, 1.0), 1.0));
    blocks.push(BlockType::Leaves.to_block(Vector3::new(-3.0, 2.0, 4.0), 1.0));
    blocks.push(BlockType::Leaves.to_block(Vector3::new(-3.0, 2.0, 3.0), 1.0));
    blocks.push(BlockType::Leaves.to_block(Vector3::new(-3.0, 2.0, 2.0), 1.0));
    blocks.push(BlockType::Leaves.to_block(Vector3::new(-3.0, 2.0, 1.0), 1.0));

    // // === BLOQUES DE MAGMA repartidos por el suelo (puntos calientes) ===
    let magma_spots = vec![
        (-3.0, 3.0),
        (0.0, 3.0),
        (3.0, -3.0),
        (-1.0,3.0),
        (-1.0, 2.0),
    ];
    for (mx, mz) in magma_spots {
        // poner magma sobre la capa de superficie (y = 0)
        replace_block(
            &mut blocks,
            BlockType::Magma.to_block(Vector3::new(mx, 0.0, mz), 1.0),
        );
    }

    // === SOL EMISIVO (fuente de luz visual) ===
    blocks.push(BlockType::Sun.to_block(Vector3::new(8.0, 10.0, -8.0), 2.0));

    println!("Isla flotante creada con {} bloques", blocks.len());
    println!("- Casa: 3x3 con ventanas y techo");
    println!("- Árbol: 3 bloques de altura con copa");
    println!("- Lago: 3x3 con arena alrededor");
    println!("- Base: isla flotante cónica");

    blocks
}

pub fn replace_block(blocks: &mut Vec<Block>, new_block: Block) {
    let pos = new_block.position;

    // Quitar cualquier bloque existente en esa posición
    blocks.retain(|b| b.position != pos);

    // Insertar el nuevo
    blocks.push(new_block);
}
