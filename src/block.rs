// block.rs
use raylib::prelude::*;
use crate::material::Material;
use crate::ray_intersect::{Intersect, RayIntersect};
use crate::light::Light;

#[derive(Debug, Clone)]
pub struct Block {
    pub position: Vector3,
    pub size: f32,
    pub material: Material,
    pub emission: Option<Light>,
}

impl Block {
    pub fn new(position: Vector3, size: f32, material: Material) -> Self {
        Self { position, size, material, emission: None }
    }

    pub fn new_emissive(
        position: Vector3,
        size: f32,
        material: Material,
        color: Vector3,
        intensity: f32,
    ) -> Self {
        let light = Light::new(position, color, intensity);
        Self {
            position,
            size,
            material,
            emission: Some(light),
        }
    }

    /// Calcula UV básicos según la cara golpeada y el punto local.
    /// Retorna (u,v) en 0..1.
    fn calc_uv(&self, point: &Vector3, normal: &Vector3) -> (f32, f32) {
        let local = *point - self.position;
        let half = self.size * 0.5;
        // Convertir a rango [0,size]
        let lx = (local.x + half) / self.size;
        let ly = (local.y + half) / self.size;
        let lz = (local.z + half) / self.size;

        if normal.x.abs() > 0.9 {
            // caras +/- X : usar Z vertical = y, horizontal = z
            (lz.clamp(0.0, 1.0), 1.0 - ly.clamp(0.0, 1.0))
        } else if normal.y.abs() > 0.9 {
            // caras +/- Y : usar X horizontal, Z vertical
            (lx.clamp(0.0, 1.0), (if normal.y > 0.0 { lz } else { 1.0 - lz }).clamp(0.0, 1.0))
        } else {
            // caras +/- Z : usar X horizontal y Y vertical
            (if normal.z > 0.0 { 1.0 - lx } else { lx }.clamp(0.0, 1.0), 1.0 - ly.clamp(0.0, 1.0))
        }
    }
}

impl<'a> RayIntersect<'a> for Block {
    fn ray_intersect(&'a self, origin: &Vector3, dir: &Vector3) -> Intersect<'a> {
        // AABB centered on position
        let half = self.size * 0.5;
        let min = self.position - Vector3::new(half, half, half);
        let max = self.position + Vector3::new(half, half, half);

        // Handle possible zero components in dir by using large values (slab method safe)
        let invx = if dir.x.abs() > 1e-8 { 1.0 / dir.x } else { f32::INFINITY };
        let invy = if dir.y.abs() > 1e-8 { 1.0 / dir.y } else { f32::INFINITY };
        let invz = if dir.z.abs() > 1e-8 { 1.0 / dir.z } else { f32::INFINITY };

        let mut tmin = (min.x - origin.x) * invx;
        let mut tmax = (max.x - origin.x) * invx;
        if tmin > tmax { std::mem::swap(&mut tmin, &mut tmax); }

        let mut tymin = (min.y - origin.y) * invy;
        let mut tymax = (max.y - origin.y) * invy;
        if tymin > tymax { std::mem::swap(&mut tymin, &mut tymax); }

        if (tmin > tymax) || (tymin > tmax) { return Intersect::empty(); }
        if tymin > tmin { tmin = tymin; }
        if tymax < tmax { tmax = tymax; }

        let mut tzmin = (min.z - origin.z) * invz;
        let mut tzmax = (max.z - origin.z) * invz;
        if tzmin > tzmax { std::mem::swap(&mut tzmin, &mut tzmax); }

        if (tmin > tzmax) || (tzmin > tmax) { return Intersect::empty(); }
        if tzmin > tmin { tmin = tzmin; }
        if tzmax < tmax { tmax = tzmax; }

        if tmin < 0.0 && tmax < 0.0 { return Intersect::empty(); }

        let distance = if tmin >= 0.0 { tmin } else { tmax };
        let point = *origin + *dir * distance;

        // Determine approximate normal
        let epsilon = 1e-4;
        let mut normal = Vector3::zero();
        if (point.x - min.x).abs() < epsilon { normal = Vector3::new(-1.0, 0.0, 0.0); }
        else if (point.x - max.x).abs() < epsilon { normal = Vector3::new(1.0, 0.0, 0.0); }
        else if (point.y - min.y).abs() < epsilon { normal = Vector3::new(0.0, -1.0, 0.0); }
        else if (point.y - max.y).abs() < epsilon { normal = Vector3::new(0.0, 1.0, 0.0); }
        else if (point.z - min.z).abs() < epsilon { normal = Vector3::new(0.0, 0.0, -1.0); }
        else if (point.z - max.z).abs() < epsilon { normal = Vector3::new(0.0, 0.0, 1.0); }

        let (u, v) = self.calc_uv(&point, &normal);

        Intersect::new(&self.material, distance, normal, point, u, v)
    }
}
