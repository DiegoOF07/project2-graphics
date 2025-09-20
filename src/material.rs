// material.rs
use raylib::prelude::*;

/// Define las propiedades físicas y visuales de un material.
/// Se usa para calcular cómo interactúa la luz con la superficie.
#[derive(Debug, Clone)]
pub struct Material {
    /// Color base difuso (en espacio RGB normalizado 0.0–1.0).
    pub diffuse: Vector3,

    /// Coeficientes de mezcla entre el color propio y la luz:
    /// [albedo_difuso, albedo_especular]
    pub albedo: [f32; 2],

    /// Intensidad de brillo especular (mayor = más brillante).
    pub specular: f32,

    /// Reflectividad de la superficie:
    /// 0.0 = no refleja, 1.0 = espejo perfecto.
    pub reflectivity: f32,

    /// Transparencia de la superficie:
    /// 0.0 = opaco, 1.0 = perfectamente transparente.
    pub transparency: f32,

    /// Índice de refracción (ej: aire ≈ 1.0, agua ≈ 1.33, vidrio ≈ 1.5).
    pub refractive_index: f32,

    /// Ruta opcional a la textura difusa.
    pub texture: Option<String>,

    /// Ruta opcional a un normal map.
    pub normal_map_id: Option<String>,
}

impl Material {
    /// Crea un nuevo material con los parámetros especificados.
    pub fn new(
        diffuse: Vector3,
        albedo: [f32; 2],
        specular: f32,
        reflectivity: f32,
        transparency: f32,
        refractive_index: f32,
        texture: Option<String>,
        normal_map_id: Option<String>,
    ) -> Self {
        Self {
            diffuse,
            albedo,
            specular,
            reflectivity,
            transparency,
            refractive_index,
            texture,
            normal_map_id,
        }
    }

    /// Material negro por defecto (sin interacción con la luz).
    pub fn black() -> Self {
        Self {
            diffuse: Vector3::zero(),
            albedo: [0.0, 0.0],
            specular: 0.0,
            reflectivity: 0.0,
            transparency: 0.0,
            refractive_index: 0.0,
            texture: None,
            normal_map_id: None,
        }
    }
}

/// Convierte un `Vector3` (0.0–1.0) en un `Color` de Raylib (0–255).
pub fn vector3_to_color(v: Vector3) -> Color {
    Color::new(
        (v.x * 255.0).clamp(0.0, 255.0) as u8,
        (v.y * 255.0).clamp(0.0, 255.0) as u8,
        (v.z * 255.0).clamp(0.0, 255.0) as u8,
        255,
    )
}

/// Convierte un `Color` de Raylib (0–255) en un `Vector3` normalizado (0.0–1.0).
pub fn color_to_vector3(color: Color) -> Vector3 {
    Vector3::new(
        color.r as f32 / 255.0,
        color.g as f32 / 255.0,
        color.b as f32 / 255.0,
    )
}
