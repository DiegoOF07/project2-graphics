// ray_intersect.rs
use raylib::prelude::Vector3;
use crate::material::Material;

/// Resultado de una intersección. Contiene una referencia al material
/// para evitar clonados de Material por cada rayo.
#[derive(Debug, Clone, Copy)]
pub struct Intersect<'a> {
    /// Referencia al material (None si no hay impacto)
    pub material: Option<&'a Material>,

    /// Distancia desde el origen del rayo hasta el punto de impacto.
    pub distance: f32,

    /// Indica si hubo impacto.
    pub is_intersecting: bool,

    /// Normal en el punto de impacto (unitaria)
    pub normal: Vector3,

    /// Punto de impacto en coordenadas del mundo
    pub point: Vector3,

    /// Coordenadas UV (0..1) si aplica
    pub u: f32,
    pub v: f32,
}

impl<'a> Intersect<'a> {
    /// Intersección válida con referencia al material
    pub fn new(material: &'a Material, distance: f32, normal: Vector3, point: Vector3, u: f32, v: f32) -> Self {
        Intersect {
            material: Some(material),
            distance,
            is_intersecting: true,
            normal,
            point,
            u,
            v,
        }
    }

    /// Intersección vacía (no impactó)
    pub fn empty() -> Self {
        Intersect {
            material: None,
            distance: f32::INFINITY,
            is_intersecting: false,
            normal: Vector3::zero(),
            point: Vector3::zero(),
            u: 0.0,
            v: 0.0,
        }
    }
}

/// Trait que define la capacidad de ser intersectado por un rayo.
/// Ahora parametrizado por lifetime para devolver referencias al material.
pub trait RayIntersect<'a> {
    fn ray_intersect(&'a self, ray_origin: &Vector3, ray_direction: &Vector3) -> Intersect<'a>;
}
