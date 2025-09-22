// snell.rs - Versión actualizada con soporte de texturas
use raylib::prelude::*;
use crate::block::Block;
use crate::light::Light;
use crate::ray_intersect::{Intersect, RayIntersect};
use crate::textures::TextureManager;

/// Reflexión: R = I - 2(N·I)N
pub fn reflect(incident: &Vector3, normal: &Vector3) -> Vector3 {
    *incident - *normal * 2.0 * incident.dot(*normal)
}

/// Refracción según la ley de Snell
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

    if k < 0.0 {
        Vector3::zero()
    } else {
        *incident * eta + n * (eta * cosi - k.sqrt())
    }
}

/// Raytracer con soporte de texturas
pub fn trace_ray(
    origin: Vector3,
    dir: Vector3,
    depth: u32,
    max_depth: u32,
    scene: &[Block],
    light: &Light,
    texture_manager: &TextureManager,
) -> Vector3 {
    if depth > max_depth {
        return Vector3::new(0.2, 0.3, 0.6); // Cielo
    }

    let mut closest: Option<Intersect> = None;
    for block in scene {
        let hit = block.ray_intersect(&origin, &dir);
        if hit.is_intersecting {
            if closest.is_none() || hit.distance < closest.as_ref().unwrap().distance {
                closest = Some(hit);
            }
        }
    }

    if let Some(intersect) = closest {
        // Obtener color base del material o textura
        let mut base_color = intersect.material.diffuse;
        
        // Aplicar textura si existe
        if let Some(texture_path) = &intersect.material.texture {
            let texture_color = texture_manager.sample_texture(texture_path, intersect.u, intersect.v);
            base_color = base_color * texture_color; // Multiplicar colores
        }

        // Modificar normal si hay normal map
        let mut surface_normal = intersect.normal;
        if let Some(normal_map_path) = &intersect.material.normal_map_id {
            let map_normal = texture_manager.sample_normal_map(normal_map_path, intersect.u, intersect.v);
            // En una implementación completa, aquí aplicarías la transformación TBN
            // Para simplificar, mezclamos las normales
            surface_normal = (surface_normal + map_normal * 0.5).normalized();
        }

        // Iluminación difusa
        let light_dir = (light.position - intersect.point).normalized();
        let diffuse_intensity = surface_normal.dot(light_dir).max(0.0) * light.intensity;
        
        // Color final con albedo
        let mut final_color = base_color * light.color * diffuse_intensity * intersect.material.albedo[0];

        // Componente especular
        if intersect.material.specular > 0.0 {
            let view_dir = (-dir).normalized();
            let reflect_dir = reflect(&(-light_dir), &surface_normal);
            let spec = view_dir.dot(reflect_dir).max(0.0).powf(intersect.material.specular);
            let specular_color = light.color * spec * intersect.material.albedo[1];
            final_color = final_color + specular_color;
        }

        // Reflexiones
        if intersect.material.reflectivity > 0.0 {
            let reflected_dir = reflect(&dir, &surface_normal).normalized();
            let reflection_origin = intersect.point + surface_normal * 1e-4; // Evitar auto-intersección
            let reflected_color = trace_ray(
                reflection_origin,
                reflected_dir,
                depth + 1,
                max_depth,
                scene,
                light,
                texture_manager,
            );
            final_color = final_color * (1.0 - intersect.material.reflectivity)
                + reflected_color * intersect.material.reflectivity;
        }

        final_color
    } else {
        Vector3::new(0.2, 0.3, 0.6) // Color del cielo
    }
}