//block.rs - Versión actualizada con coordenadas UV
use raylib::prelude::*;
use crate::material::Material;
use crate::ray_intersect::{Intersect, RayIntersect};

#[derive(Debug, Clone)]
pub struct Block {
    pub position: Vector3,
    pub size: f32,
    pub material: Material,
}

impl Block {
    pub fn new(position: Vector3, size: f32, material: Material) -> Self {
        Self {
            position,
            size,
            material,
        }
    }
}

impl RayIntersect for Block {
    fn ray_intersect(&self, origin: &Vector3, dir: &Vector3) -> Intersect {
        // AABB ray-box intersection (cubo centrado en position)
        let half = self.size * 0.5;
        let min = self.position - Vector3::new(half, half, half);
        let max = self.position + Vector3::new(half, half, half);

        let mut tmin = (min.x - origin.x) / dir.x;
        let mut tmax = (max.x - origin.x) / dir.x;
        if tmin > tmax {
            std::mem::swap(&mut tmin, &mut tmax);
        }

        let mut tymin = (min.y - origin.y) / dir.y;
        let mut tymax = (max.y - origin.y) / dir.y;
        if tymin > tymax {
            std::mem::swap(&mut tymin, &mut tymax);
        }

        if (tmin > tymax) || (tymin > tmax) {
            return Intersect::empty();
        }

        if tymin > tmin { tmin = tymin; }
        if tymax < tmax { tmax = tymax; }

        let mut tzmin = (min.z - origin.z) / dir.z;
        let mut tzmax = (max.z - origin.z) / dir.z;
        if tzmin > tzmax {
            std::mem::swap(&mut tzmin, &mut tzmax);
        }

        if (tmin > tzmax) || (tzmin > tmax) {
            return Intersect::empty();
        }

        if tzmin > tmin { tmin = tzmin; }
        if tzmax < tmax { tmax = tzmax; }

        if tmin < 0.0 && tmax < 0.0 {
            return Intersect::empty();
        }

        let distance = if tmin >= 0.0 { tmin } else { tmax };
        let point = *origin + *dir * distance;

        // Calcular normal y coordenadas UV según la cara del cubo
        let (normal, u, v) = self.calculate_face_normal_and_uv(&point, &min, &max);

        Intersect::new(
            self.material.clone(),
            distance,
            normal,
            point,
            u,
            v,
        )
    }
}

impl Block {
    /// Calcula la normal y coordenadas UV basándose en qué cara del cubo fue impactada
    fn calculate_face_normal_and_uv(&self, point: &Vector3, min: &Vector3, max: &Vector3) -> (Vector3, f32, f32) {
        let epsilon = 1e-4;
        
        // Determinar qué cara fue impactada y calcular UV correspondientes
        if (point.x - min.x).abs() < epsilon {
            // Cara izquierda (-X)
            let u = (point.z - min.z) / (max.z - min.z);
            let v = 1.0 - (point.y - min.y) / (max.y - min.y);
            (Vector3::new(-1.0, 0.0, 0.0), u, v)
        } else if (point.x - max.x).abs() < epsilon {
            // Cara derecha (+X)
            let u = 1.0 - (point.z - min.z) / (max.z - min.z);
            let v = 1.0 - (point.y - min.y) / (max.y - min.y);
            (Vector3::new(1.0, 0.0, 0.0), u, v)
        } else if (point.y - min.y).abs() < epsilon {
            // Cara inferior (-Y)
            let u = (point.x - min.x) / (max.x - min.x);
            let v = (point.z - min.z) / (max.z - min.z);
            (Vector3::new(0.0, -1.0, 0.0), u, v)
        } else if (point.y - max.y).abs() < epsilon {
            // Cara superior (+Y)
            let u = (point.x - min.x) / (max.x - min.x);
            let v = 1.0 - (point.z - min.z) / (max.z - min.z);
            (Vector3::new(0.0, 1.0, 0.0), u, v)
        } else if (point.z - min.z).abs() < epsilon {
            // Cara trasera (-Z)
            let u = 1.0 - (point.x - min.x) / (max.x - min.x);
            let v = 1.0 - (point.y - min.y) / (max.y - min.y);
            (Vector3::new(0.0, 0.0, -1.0), u, v)
        } else if (point.z - max.z).abs() < epsilon {
            // Cara frontal (+Z)
            let u = (point.x - min.x) / (max.x - min.x);
            let v = 1.0 - (point.y - min.y) / (max.y - min.y);
            (Vector3::new(0.0, 0.0, 1.0), u, v)
        } else {
            // Fallback
            (Vector3::new(1.0, 0.0, 0.0), 0.0, 0.0)
        }
    }
}