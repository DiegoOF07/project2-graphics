// light.rs
use raylib::prelude::*;

/// Representa una luz puntual en la escena.
/// Se define por su posición, color e intensidad.
#[derive(Debug, Clone, Copy)]
pub struct Light {
    /// Posición de la luz en el espacio 3D
    pub position: Vector3,
    /// Color de la luz (RGB en rango 0.0 - 1.0)
    pub color: Vector3,
    /// Intensidad de la luz (factor multiplicador)
    pub intensity: f32,
}

impl Light {
    /// Crea una nueva luz con parámetros personalizados.
    pub fn new(position: Vector3, color: Vector3, intensity: f32) -> Self {
        Self { position, color, intensity }
    }

    /// Devuelve el color de la luz como `raylib::Color` (clamp de 0-255).
    pub fn as_color(&self) -> Color {
        Color::new(
            (self.color.x.clamp(0.0, 1.0) * 255.0) as u8,
            (self.color.y.clamp(0.0, 1.0) * 255.0) as u8,
            (self.color.z.clamp(0.0, 1.0) * 255.0) as u8,
            255,
        )
    }
}

impl Default for Light {
    /// Luz blanca débil en el origen (para debug rápido).
    fn default() -> Self {
        Self {
            position: Vector3::zero(),
            color: Vector3::one(),
            intensity: 1.0,
        }
    }
}
