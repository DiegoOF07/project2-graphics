//ray_intersect.rs
use raylib::prelude::Vector3;
use crate::{material::Material, ray_intersect};

/// Representa el resultado de un rayo al intersectar con un objeto en la escena.
/// Contiene toda la información necesaria para el shading.
#[derive(Debug, Clone)]
pub struct Intersect {
    /// Material del objeto con el que se produjo la intersección.
    pub material: Material,

    /// Distancia desde el origen del rayo hasta el punto de impacto.
    /// Sirve para saber qué intersección es la más cercana.
    pub distance: f32,

    /// Indica si el rayo realmente intersectó o no con algo.
    pub is_intersecting: bool,

    /// Vector normal en el punto de intersección (unitario).
    /// Necesario para calcular iluminación, reflexiones, refracciones.
    pub normal: Vector3,

    /// Punto exacto de la intersección en coordenadas del mundo.
    pub point: Vector3,

    /// Coordenadas de textura (u,v), si el objeto tiene texturas.
    pub u: f32,
    pub v: f32,
}

impl Intersect {
    /// Constructor de una intersección válida.
    /// - `material`: material del objeto intersectado.
    /// - `distance`: distancia desde el rayo.
    /// - `normal`: normal unitario en el punto.
    /// - `point`: posición del impacto.
    /// - `(u, v)`: coordenadas de textura.
    pub fn new(
        material: Material,
        distance: f32,
        normal: Vector3,
        point: Vector3,
        u: f32,
        v: f32,
    ) -> Self {
        Intersect {
            material,
            distance,
            is_intersecting: true,
            normal,
            point,
            u,
            v,
        }
    }

    /// Constructor de una intersección vacía (no hubo impacto).
    /// Se usa como valor por defecto antes de comparar intersecciones.
    pub fn empty() -> Self {
        Intersect {
            material: Material {
                diffuse: Vector3::zero(),
                albedo: [0.0, 0.0],
                specular: 0.0,
                reflectivity: 0.0,
                transparency: 0.0,
                refractive_index: 0.0,
                texture: None,
                normal_map_id: None,
            },
            distance: f32::INFINITY, // Importante: no 0, sino infinito
            is_intersecting: false,
            normal: Vector3::zero(),
            point: Vector3::zero(),
            u: 0.0,
            v: 0.0,
        }
    }
}

/// Trait que define la capacidad de ser intersectado por un rayo.
/// Cualquier objeto geométrico (esfera, plano, triángulo, etc.)
/// deberá implementar este trait para ser renderizable.
pub trait RayIntersect {
    /// Calcula la intersección entre el objeto y un rayo.
    ///
    /// - `ray_origin`: punto de origen del rayo.
    /// - `ray_direction`: dirección normalizada del rayo.
    ///
    /// Devuelve un `Intersect` con la información del impacto.
    fn ray_intersect(&self, ray_origin: &Vector3, ray_direction: &Vector3) -> Intersect;
}