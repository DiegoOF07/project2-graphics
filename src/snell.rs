// snell.rs
use raylib::prelude::*;
use crate::cube::Cube;
use crate::light::Light;
use crate::ray_intersect::{Intersect, RayIntersect};

/// Calcula el vector reflejado según la ley de reflexión.
/// `incident`: dirección del rayo incidente (normalizado).
/// `normal`: normal de la superficie (normalizada).
pub fn reflect(incident: &Vector3, normal: &Vector3) -> Vector3 {
    *incident - *normal * 2.0 * incident.dot(*normal)
}

/// Calcula el vector refractado según la Ley de Snell.
/// `incident`: dirección del rayo incidente (normalizado).
/// `normal`: normal de la superficie (normalizada).
/// `refractive_index`: índice de refracción del material (ej. vidrio ~1.5).
///
/// Devuelve el vector refractado. Si ocurre **reflexión interna total**, 
/// devuelve `Vector3::zero()` para indicar que no hay refracción.
pub fn refract(incident: &Vector3, normal: &Vector3, refractive_index: f32) -> Vector3 {
    // Coseno del ángulo entre el rayo incidente y la normal
    let mut cosi = incident.dot(*normal).clamp(-1.0, 1.0);

    // Índices de refracción
    let mut etai = 1.0; // aire (n ~ 1.0)
    let mut etat = refractive_index;
    let mut n = *normal;

    if cosi > 0.0 {
        // El rayo está dentro del material y sale al aire
        std::mem::swap(&mut etai, &mut etat);
        n = -n;
    } else {
        cosi = -cosi;
    }

    let eta = etai / etat;
    let k = 1.0 - eta * eta * (1.0 - cosi * cosi);

    if k < 0.0 {
        // Reflexión interna total
        Vector3::zero()
    } else {
        *incident * eta + n * (eta * cosi - k.sqrt())
    }
}

/// Traza un rayo en la escena con múltiples rebotes
///
/// # Parámetros
/// - `origin`: punto de inicio del rayo
/// - `dir`: dirección normalizada del rayo
/// - `depth`: profundidad actual de recursión
/// - `max_depth`: límite de rebotes
/// - `scene`: lista de cubos (objetos en la escena)
/// - `light`: fuente de luz
///
/// # Retorno
/// Vector3 con el color calculado (RGB en [0,1])
pub fn trace_ray(
    origin: Vector3,
    dir: Vector3,
    depth: u32,
    max_depth: u32,
    scene: &[Cube],
    light: &Light,
) -> Vector3 {
    if depth > max_depth {
        return Vector3::new(0.2, 0.3, 0.6); // fondo (azul cielo)
    }

    // Buscar intersección más cercana
    let mut closest: Option<Intersect> = None;
    for cube in scene {
        let hit = cube.ray_intersect(&origin, &dir);
        if hit.is_intersecting {
            if closest.is_none() || hit.distance < closest.as_ref().unwrap().distance {
                closest = Some(hit);
            }
        }
    }

    if let Some(intersect) = closest {
        // === Difuso (Lambert) ===
        let light_dir = (light.position - intersect.point).normalized();
        let diff = intersect.normal.dot(light_dir).max(0.0) * light.intensity;
        let mut color = intersect.material.diffuse * diff;

        // === Reflexión ===
        if intersect.material.reflectivity > 0.0 {
            let reflected = reflect(&dir, &intersect.normal).normalized();
            let reflected_color = trace_ray(intersect.point, reflected, depth + 1, max_depth, scene, light);
            color = color * (1.0 - intersect.material.reflectivity)
                + reflected_color * intersect.material.reflectivity;
        }

        color
    } else {
        // === Fondo ===
        Vector3::new(0.2, 0.3, 0.6) // azul cielo
    }
}