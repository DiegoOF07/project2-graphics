// snell.rs
use raylib::prelude::*;
use crate::block::Block;
use crate::light::Light;
use crate::ray_intersect::{Intersect, RayIntersect};
use crate::textures::TextureManager;

/// Reflexión
pub fn reflect(incident: &Vector3, normal: &Vector3) -> Vector3 {
    *incident - *normal * 2.0 * incident.dot(*normal)
}

/// Refracción simple (igual que antes)
pub fn refract(incident: &Vector3, normal: &Vector3, refractive_index: f32) -> Vector3 {
    let mut cosi = incident.dot(*normal).clamp(-1.0, 1.0);
    let mut etai = 1.0;
    let mut etat = refractive_index;
    let mut n = *normal;
    if cosi > 0.0 {
        std::mem::swap(&mut etai, &mut etat);
        n = -n;
    } else {
        cosi = -cosi;
    }
    let eta = etai / etat;
    let k = 1.0 - eta * eta * (1.0 - cosi * cosi);
    if k < 0.0 { Vector3::zero() } else { *incident * eta + n * (eta * cosi - k.sqrt()) }
}

/// trace_ray ahora recibe `scene: &[Block]` y `tex_manager` para muestreo de texturas.
/// Devuelve color en Vector3 [0..1].
pub fn trace_ray(
    origin: Vector3,
    dir: Vector3,
    depth: u32,
    max_depth: u32,
    scene: &[Block],
    light: &Light,
    tex_manager: &TextureManager,
) -> Vector3 {
    if depth > max_depth {
        return Vector3::new(0.2, 0.3, 0.6); // fondo
    }

    // Buscar intersección más cercana
    let mut closest: Option<Intersect> = None;
    for block in scene {
        let hit = block.ray_intersect(&origin, &dir);
        if hit.is_intersecting {
            if closest.is_none() || hit.distance < closest.as_ref().unwrap().distance {
                closest = Some(hit);
            }
        }
    }

    if let Some(hit) = closest {
        // shading lambert
        let light_dir = (light.position - hit.point).normalized();
        let diff = hit.normal.dot(light_dir).max(0.0) * light.intensity;
        // base color from material (could be texture)
        let mut base_color = hit.material.unwrap().diffuse; // safe unwrap: material present when is_intersecting

        // If material has texture, sample it using hit.u, hit.v
        if let Some(path) = &hit.material.unwrap().texture {
            // Texture coordinates -> pixel indices. We'll assume texture size sampling is handled in manager.
            // For simplicity, map u,v to texture size inside TextureManager.
            let tex_color = tex_manager.get_pixel_color(path, (hit.u *  (tex_manager.width_of(path) as f32)).floor() as i32, (hit.v * (tex_manager.height_of(path) as f32)).floor() as i32);
            base_color = tex_color;
        }

        let mut color = base_color * diff;

        // reflection
        if hit.material.unwrap().reflectivity > 0.0 {
            let reflected = reflect(&dir, &hit.normal).normalized();
            let reflected_color = trace_ray(hit.point + reflected * 1e-4, reflected, depth + 1, max_depth, scene, light, tex_manager);
            color = color * (1.0 - hit.material.unwrap().reflectivity) + reflected_color * hit.material.unwrap().reflectivity;
        }

        color
    } else {
        Vector3::new(0.2, 0.3, 0.6)
    }
}
