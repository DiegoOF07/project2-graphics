// snell.rs - Módulo de raytracing optimizado y reorganizado
use crate::block::Block;
use crate::light::Light;
use crate::ray_intersect::{Intersect, RayIntersect};
use crate::textures::TextureManager;
use raylib::prelude::*;

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
fn find_closest_intersection<'a>(
    origin: &Vector3,
    dir: &Vector3,
    scene: &'a [Block],
) -> Option<Intersect<'a>> {
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

/// Raytracer principal con múltiples luces, reflexiones y transparencia + fake glow
pub fn trace_ray_multi_light(
    origin: Vector3,
    dir: Vector3,
    depth: u32,
    max_depth: u32,
    scene: &[Block],
    lights: &[Light],
    texture_manager: &TextureManager,
) -> Vector3 {
    if depth > max_depth {
        return sky_color(&dir);
    }

    let intersect = match find_closest_intersection(&origin, &dir, scene) {
        Some(hit) => hit,
        None => return sky_color(&dir),
    };

    let material = match intersect.material {
        Some(mat) => mat,
        None => return sky_color(&dir),
    };

    let base_color = get_material_color(&intersect, texture_manager);

    // === iluminación directa ===
    let mut final_color = Vector3::zero();
    for light in lights {
        final_color =
            final_color + calculate_light_contribution(&intersect, light, &base_color, &dir);
    }
    if !lights.is_empty() {
        final_color = final_color / lights.len() as f32;
    }

    // === Emisión normal ===
    if let Some(emission) = &material.emission_color {
        final_color = final_color + *emission * material.emission_strength;

        // --- Fake glow extra ---
        let glow_strength = material.emission_strength;

        // Ángulo entre la normal y la dirección de la cámara
        let view_dir = -dir.normalized();
        let angle_factor = intersect.normal.dot(view_dir).clamp(0.0, 1.0).powf(2.0);

        // Atenuación por distancia
        let dist = (intersect.point - origin).length();
        let dist_factor = 1.0 / (1.0 + 0.15 * dist);

        // Añadir glow (más suave que la emisión directa)
        final_color = final_color + *emission * glow_strength * angle_factor * dist_factor * 2.0;
    }

    final_color = final_color + base_color * 0.08; // ambiente sutil

    // === reflexión y refracción ===
    let mut reflection_color = Vector3::zero();
    let mut refraction_color = Vector3::zero();
    let mut fresnel = 0.0;

    // Reflexión
    if material.reflectivity > MIN_REFLECTION_THRESHOLD && depth < max_depth {
        let reflected_dir = reflect(&dir, &intersect.normal).normalized();
        let reflect_origin = intersect.point + intersect.normal * EPSILON;
        reflection_color = trace_ray_multi_light(
            reflect_origin,
            reflected_dir,
            depth + 1,
            max_depth,
            scene,
            lights,
            texture_manager,
        );
    }

    // Refracción
    if material.transparency > 0.01 && depth < max_depth {
        let refracted_dir = refract(&dir, &intersect.normal, material.refractive_index);
        if refracted_dir.dot(refracted_dir) > 1e-6 {
            let refract_origin = if dir.dot(intersect.normal) < 0.0 {
                intersect.point - intersect.normal * EPSILON
            } else {
                intersect.point + intersect.normal * EPSILON
            };
            refraction_color = trace_ray_multi_light(
                refract_origin,
                refracted_dir.normalized(),
                depth + 1,
                max_depth,
                scene,
                lights,
                texture_manager,
            );

            // Fresnel (Schlick)
            let cos_i = (-dir.dot(intersect.normal)).abs().clamp(0.0, 1.0);
            fresnel = calculate_fresnel(cos_i, material.refractive_index);
        } else {
            fresnel = 1.0; // reflexión interna total
        }
    }

    // === combinación final ===
    if material.transparency > 0.01 && material.reflectivity > MIN_REFLECTION_THRESHOLD {
        // Caso 3: Material con transparencia + reflectividad (vidrio espejado)
        let direct = final_color * (1.0 - material.transparency) * (1.0 - material.reflectivity);
        let reflect = reflection_color * fresnel * material.reflectivity;
        let refract = refraction_color * material.transparency * (1.0 - fresnel);
        final_color = direct + reflect + refract;
    } else if material.transparency > 0.01 {
        // Caso 1: Solo transparente
        let direct = final_color * (1.0 - material.transparency);
        let reflect = reflection_color * fresnel;
        let refract = refraction_color * material.transparency * (1.0 - fresnel);
        final_color = direct + reflect + refract;
    } else if material.reflectivity > MIN_REFLECTION_THRESHOLD {
        // Caso 2: Solo reflectivo
        final_color =
            final_color * (1.0 - material.reflectivity) + reflection_color * material.reflectivity;
    }

    Vector3::new(
        final_color.x.clamp(0.0, 1.0),
        final_color.y.clamp(0.0, 1.0),
        final_color.z.clamp(0.0, 1.0),
    )
}


/// Calcula el coeficiente de reflexión de Fresnel
fn calculate_fresnel(cos_i: f32, refractive_index: f32) -> f32 {
    let n1 = 1.0;
    let n2 = refractive_index;
    let r0 = ((n1 - n2) / (n1 + n2)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cos_i).powi(5) // Schlick
}
