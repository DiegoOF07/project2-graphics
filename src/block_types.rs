use raylib::prelude::*;
use crate::block::Block;
use crate::material::Material;

/// Enum que define los tipos de bloques disponibles
#[derive(Clone)]
pub enum BlockType {
    Grass,
    Dirt,
    Stone,
    Cobble,
    WoodPlank,
    WoodLog,
    Leaves,
    DeepslateBricks,
    Glass,
    Metal,
}

impl BlockType {
    /// Devuelve el material asociado a cada tipo de bloque
    pub fn material(&self) -> Material {
        match self {
            BlockType::Grass => Material {
                diffuse: Vector3::new(0.4, 0.8, 0.3),
                albedo: [0.9, 0.1],
                specular: 5.0,
                reflectivity: 0.0,
                transparency: 0.0,
                refractive_index: 1.0,
                texture: Some("textures/grass_top.jpg".to_string()),
                normal_map_id: None,
            },
            BlockType::Dirt => Material {
                diffuse: Vector3::new(0.4, 0.3, 0.2),
                albedo: [0.9, 0.1],
                specular: 2.0,
                reflectivity: 0.0,
                transparency: 0.0,
                refractive_index: 1.0,
                texture: Some("textures/dirt.jpg".to_string()),
                normal_map_id: None,
            },
            BlockType::Stone => Material {
                diffuse: Vector3::new(0.5, 0.5, 0.5),
                albedo: [0.9, 0.1],
                specular: 3.0,
                reflectivity: 0.0,
                transparency: 0.0,
                refractive_index: 1.0,
                texture: Some("textures/stone.jpg".to_string()),
                normal_map_id: None,
            },
            BlockType::Cobble => Material {
                diffuse: Vector3::new(0.6, 0.6, 0.6),
                albedo: [0.8, 0.2],
                specular: 15.0,
                reflectivity: 0.0,
                transparency: 0.0,
                refractive_index: 1.0,
                texture: Some("textures/cobble.png".to_string()),
                normal_map_id: None,
            },
            BlockType::WoodPlank => Material {
                diffuse: Vector3::new(0.8, 0.5, 0.2),
                albedo: [0.8, 0.2],
                specular: 8.0,
                reflectivity: 0.0,
                transparency: 0.0,
                refractive_index: 1.0,
                texture: Some("textures/wood_oak.jpg".to_string()),
                normal_map_id: None,
            },
            BlockType::WoodLog => Material {
                diffuse: Vector3::new(0.4, 0.3, 0.1),
                albedo: [0.8, 0.2],
                specular: 5.0,
                reflectivity: 0.0,
                transparency: 0.0,
                refractive_index: 1.0,
                texture: Some("textures/wood_oak_log.jpg".to_string()),
                normal_map_id: None,
            },
            BlockType::Leaves => Material {
                diffuse: Vector3::new(0.2, 0.6, 0.2),
                albedo: [0.9, 0.1],
                specular: 3.0,
                reflectivity: 0.0,
                transparency: 0.0,
                refractive_index: 1.0,
                texture: Some("textures/leaves_oak.jpg".to_string()),
                normal_map_id: None,
            },
            BlockType::DeepslateBricks => Material {
                diffuse: Vector3::new(0.7, 0.3, 0.2),
                albedo: [0.8, 0.2],
                specular: 12.0,
                reflectivity: 0.0,
                transparency: 0.0,
                refractive_index: 1.0,
                texture: Some("textures/deepslate_bricks.jpg".to_string()),
                normal_map_id: None,
            },
            BlockType::Glass => Material {
                diffuse: Vector3::new(0.9, 0.9, 1.0),
                albedo: [0.1, 0.9],
                specular: 200.0,
                reflectivity: 0.1,
                transparency: 0.9,
                refractive_index: 1.0,
                texture: Some("textures/glass.png".to_string()),
                normal_map_id: None,
            },
            BlockType::Metal => Material {
                diffuse: Vector3::new(0.8, 0.8, 0.9),
                albedo: [0.3, 0.7],
                specular: 100.0,
                reflectivity: 0.8,
                transparency: 0.0,
                refractive_index: 1.0,
                texture: Some("textures/stone.jpg".to_string()), // Base metálica
                normal_map_id: None,
            },
        }
    }

    /// Crea un bloque de este tipo en una posición dada
    pub fn to_block(&self, position: Vector3, size: f32) -> Block {
        Block::new(position, size, self.material())
    }
}
