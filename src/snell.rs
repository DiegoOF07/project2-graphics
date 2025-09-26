// snell.rs - Módulo de raytracing
use raylib::prelude::*;
use crate::block::Block;
use crate::light::Light;
use crate::ray_intersect::{Intersect, RayIntersect};
use crate::textures::TextureManager;

// === CONSTANTES ===
const MAX_DISTANCE: f32 = 50.0;
const EPSILON: f32 = 1e-4;
const MIN_REFLECTION_THRESHOLD: f32 = 0.05;
const MIN_SPECULAR_THRESHOLD: f32 = 5.0;

// === FUNCIONES DE FÍSICA ÓPTICA ===

/// Calcula la reflexión de un rayo: R = I - 2(N·I)N
#[inline]
pub fn reflect(incident: &Vector3, normal: &Vector3) -> Vector3 {
    *incident - *normal * 2.0 * incident.dot(*normal)
}

/// Calcula la refracción usando la ley de Snell
pub fn refract(incident: &Vector3, normal: &Vector3, refractive_index: f32) -> Vector3 {
    let mut cosi = incident.dot(*normal).clamp(-1.0, 1.0);
    let mut etai = 1.0;
    let mut etat = refractive_index;
    let mut n = *normal;

    // Determinar si entramos o salimos del material
    if cosi > 0.0 {
        std::mem::swap(&mut etai, &mut etat);
        n = -n;
    } else {
        cosi = -cosi;
    }

    let eta = etai / etat;
    let k = 1.0 - eta * eta * (1.0 - cosi * cosi);

    // Reflexión interna total si k < 0
    if k < 0.0 {
        Vector3::zero()
    } else {
        *incident * eta + n * (eta * cosi - k.sqrt())
    }
}

// === FUNCIONES DE INTERSECCIÓN ===

/// Encuentra la intersección más cercana en la escena
#[inline]
fn find_closest_intersection<'a>(origin: &Vector3, dir: &Vector3, scene: &'a [Block]) -> Option<Intersect<'a>> {
    let mut closest: Option<Intersect<'a>> = None;
    let mut min_distance = MAX_DISTANCE;

    for block in scene {
        let hit = block.ray_intersect(origin, dir);
        if hit.is_intersecting && hit.distance < min_distance {
            min_distance = hit.distance;
            
            // Early termination para objetos muy cercanos
            if hit.distance < 0.1 {
                return Some(hit);
            }
            
            closest = Some(hit);
        }
    }

    closest
}

// === FUNCIONES DE SHADING ===

/// Calcula la contribución de una luz individual
fn calculate_light_contribution<'a>(
    intersect: &Intersect<'a>,
    light: &Light,
    base_color: &Vector3,
    view_dir: &Vector3,
) -> Vector3 {
    // Verificar que el material existe
    let material = match intersect.material {
        Some(mat) => mat,
        None => return Vector3::zero(), // Sin material, sin contribución
    };

    let light_dir = (light.position - intersect.point).normalized();
    let light_distance = (light.position - intersect.point).length();
    
    // Atenuación cuadrática por distancia
    let attenuation = 1.0 / (1.0 + 0.01 * light_distance * light_distance);
    
    // Componente difusa (Lambert)
    let n_dot_l = intersect.normal.dot(light_dir).max(0.0);
    let diffuse_intensity = n_dot_l * light.intensity * attenuation;
    
    let mut color = *base_color * light.color * diffuse_intensity * material.albedo[0];
    
    // Componente especular (Blinn-Phong) solo si es significativo
    if material.specular > MIN_SPECULAR_THRESHOLD && diffuse_intensity > 0.1 {
        let view_direction = (-*view_dir).normalized();
        let half_vector = (light_dir + view_direction).normalized();
        let n_dot_h = intersect.normal.dot(half_vector).max(0.0);
        let spec = n_dot_h.powf(material.specular);
        
        color = color + light.color * spec * material.albedo[1] * attenuation;
    }
    
    color
}

/// Obtiene el color base del material, aplicando texturas si existen
#[inline]
fn get_material_color<'a>(intersect: &Intersect<'a>, texture_manager: &TextureManager) -> Vector3 {
    // Verificar que el material existe
    let material = match intersect.material {
        Some(mat) => mat,
        None => return Vector3::one(), // Color blanco por defecto si no hay material
    };

    let mut base_color = material.diffuse;
    
    // Aplicar textura si existe
    if let Some(texture_path) = &material.texture {
        let texture_color = texture_manager.sample_texture(texture_path, intersect.u, intersect.v);
        base_color = base_color * texture_color;
    }
    
    base_color
}

/// Color del cielo con gradiente basado en la dirección del rayo
#[inline]
fn sky_color(dir: &Vector3) -> Vector3 {
    let t = (dir.y * 0.5 + 0.5).clamp(0.0, 1.0); // Mapear [-1,1] a [0,1]
    
    // Gradiente de horizonte (naranja) a cenit (azul)
    let horizon_color = Vector3::new(0.8, 0.6, 0.4);
    let zenith_color = Vector3::new(0.2, 0.4, 0.8);
    
    horizon_color * (1.0 - t) + zenith_color * t
}

// === FUNCIONES PRINCIPALES DE RAYTRACING ===

/// Raytracer principal con múltiples luces y optimizaciones
pub fn trace_ray_multi_light(
    origin: Vector3,
    dir: Vector3,
    depth: u32,
    max_depth: u32,
    scene: &[Block],
    lights: &[Light],
    texture_manager: &TextureManager,
) -> Vector3 {
    // Early termination por profundidad
    if depth > max_depth {
        return sky_color(&dir);
    }

    // Buscar intersección más cercana
    let intersect = match find_closest_intersection(&origin, &dir, scene) {
        Some(hit) => hit,
        None => return sky_color(&dir),
    };

    // Obtener color base del material
    let base_color = get_material_color(&intersect, texture_manager);

    // Acumular iluminación de todas las luces
    let mut final_color = Vector3::zero();
    
    for light in lights {
        let light_contribution = calculate_light_contribution(
            &intersect,
            light,
            &base_color,
            &dir,
        );
        final_color = final_color + light_contribution;
    }
    
    // Evitar división por cero y normalizar
    if !lights.is_empty() {
        final_color = final_color / lights.len() as f32;
    }

    // Añadir color ambiente mínimo para evitar sombras completamente negras
    let ambient = base_color * 0.1;
    final_color = final_color + ambient;

    // Calcular reflexiones solo si son significativas
    if let Some(material) = intersect.material {
        if material.reflectivity > MIN_REFLECTION_THRESHOLD && depth < max_depth {
            let reflected_dir = reflect(&dir, &intersect.normal).normalized();
            let reflection_origin = intersect.point + intersect.normal * EPSILON;
            
            let reflected_color = trace_ray_multi_light(
                reflection_origin,
                reflected_dir,
                depth + 1,
                max_depth,
                scene,
                lights,
                texture_manager,
            );
            
            // Mezclar color directo con reflexión
            let reflectivity = material.reflectivity;
            final_color = final_color * (1.0 - reflectivity) + reflected_color * reflectivity;
        }
    }

    // Clamp final para evitar valores fuera de rango
    Vector3::new(
        final_color.x.clamp(0.0, 1.0),
        final_color.y.clamp(0.0, 1.0),
        final_color.z.clamp(0.0, 1.0),
    )
}

// === FUNCIÓN LEGACY PARA COMPATIBILIDAD ===

/// Función legacy para compatibilidad con código existente (una sola luz)
#[deprecated(note = "Use trace_ray_multi_light instead")]
pub fn trace_ray(
    origin: Vector3,
    dir: Vector3,
    depth: u32,
    max_depth: u32,
    scene: &[Block],
    light: &Light,
    texture_manager: &TextureManager,
) -> Vector3 {
    trace_ray_multi_light(origin, dir, depth, max_depth, scene, &[*light], texture_manager)
}

// === FUNCIONES DE UTILIDAD ===

/// Calcula la Fresnel reflection coefficient (para materiales transparentes)
#[allow(dead_code)]
pub fn fresnel(cos_i: f32, refractive_index: f32) -> f32 {
    let mut etai = 1.0;
    let mut etat = refractive_index;
    let mut cos_i = cos_i;
    
    if cos_i > 0.0 {
        std::mem::swap(&mut etai, &mut etat);
    }
    
    cos_i = cos_i.abs();
    let eta = etai / etat;
    let sin_t_sq = eta * eta * (1.0 - cos_i * cos_i);
    
    if sin_t_sq >= 1.0 {
        return 1.0; // Reflexión interna total
    }
    
    let cos_t = (1.0 - sin_t_sq).sqrt();
    let rs = ((etat * cos_i) - (etai * cos_t)) / ((etat * cos_i) + (etai * cos_t));
    let rp = ((etai * cos_i) - (etat * cos_t)) / ((etai * cos_i) + (etat * cos_t));
    
    (rs * rs + rp * rp) * 0.5
}