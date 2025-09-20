use crate::material::Material;
use crate::ray_intersect::{Intersect, RayIntersect};
use raylib::prelude::Vector3;

/// Representa un cubo axis-aligned (AABB) en el espacio.
/// Se usa para formar bloques estilo "Minecraft".
pub struct Cube {
    /// Centro del cubo.
    pub center: Vector3,

    /// Longitud de sus lados.
    pub size: f32,

    /// Material asociado al cubo.
    pub material: Material,
}

impl Cube {
    /// Crea un nuevo cubo con centro, tamaño y material.
    pub fn new(center: Vector3, size: f32, material: Material) -> Self {
        Self {
            center,
            size,
            material,
        }
    }

    /// Calcula coordenadas UV de textura para un punto en la superficie.
    /// La proyección depende de la normal (qué cara del cubo golpea).
    fn get_uv(&self, point: &Vector3, normal: &Vector3) -> (f32, f32) {
        let local_point = *point - self.center;
        let half_size = self.size * 0.5;

        // Determinar qué cara está golpeando
        let (u, v) = if normal.x.abs() > 0.9 {
            // Cara ±X
            (
                (local_point.z + half_size) / self.size,
                1.0 - (local_point.y + half_size) / self.size,
            )
        } else if normal.y.abs() > 0.9 {
            // Cara ±Y
            (
                (local_point.x + half_size) / self.size,
                if normal.y > 0.0 {
                    (local_point.z + half_size) / self.size
                } else {
                    1.0 - (local_point.z + half_size) / self.size
                },
            )
        } else {
            // Cara ±Z
            (
                if normal.z > 0.0 {
                    1.0 - (local_point.x + half_size) / self.size
                } else {
                    (local_point.x + half_size) / self.size
                },
                1.0 - (local_point.y + half_size) / self.size,
            )
        };

        (u.clamp(0.0, 1.0), v.clamp(0.0, 1.0))
    }
}

impl RayIntersect for Cube {
    /// Calcula la intersección entre un rayo y el cubo.
    /// Devuelve un `Intersect` con la información de impacto, o vacío si no golpea.
    fn ray_intersect(&self, ray_origin: &Vector3, ray_direction: &Vector3) -> Intersect {
        let half_size = self.size * 0.5;
        let min_bounds = self.center - Vector3::new(half_size, half_size, half_size);
        let max_bounds = self.center + Vector3::new(half_size, half_size, half_size);

        let mut tmin = f32::NEG_INFINITY;
        let mut tmax = f32::INFINITY;
        let mut normal = Vector3::zero();

        // Probar contra cada eje
        for (axis_index, (axis_value, (min_b, max_b))) in [
            (ray_direction.x, (min_bounds.x, max_bounds.x)),
            (ray_direction.y, (min_bounds.y, max_bounds.y)),
            (ray_direction.z, (min_bounds.z, max_bounds.z)),
        ]
        .into_iter()
        .enumerate()
        {
            let origin_component = match axis_index {
                0 => ray_origin.x,
                1 => ray_origin.y,
                _ => ray_origin.z,
            };

            if axis_value.abs() > 1e-8 {
                let t1 = (min_b - origin_component) / axis_value;
                let t2 = (max_b - origin_component) / axis_value;
                let (t_near, t_far) = if t1 < t2 { (t1, t2) } else { (t2, t1) };

                if t_near > tmin {
                    tmin = t_near;
                    normal = match axis_index {
                        0 => {
                            if t1 < t2 {
                                Vector3::new(-1.0, 0.0, 0.0)
                            } else {
                                Vector3::new(1.0, 0.0, 0.0)
                            }
                        }
                        1 => {
                            if t1 < t2 {
                                Vector3::new(0.0, -1.0, 0.0)
                            } else {
                                Vector3::new(0.0, 1.0, 0.0)
                            }
                        }
                        _ => {
                            if t1 < t2 {
                                Vector3::new(0.0, 0.0, -1.0)
                            } else {
                                Vector3::new(0.0, 0.0, 1.0)
                            }
                        }
                    };
                }
                if t_far < tmax {
                    tmax = t_far;
                }
            } else {
                // Si el rayo es paralelo y fuera de los límites, no hay intersección
                if origin_component < min_b || origin_component > max_b {
                    return Intersect::empty();
                }
            }

            if tmin > tmax {
                return Intersect::empty();
            }
        }

        // Tomar la intersección más cercana válida
        let t = if tmin > 1e-6 {
            tmin
        } else if tmax > 1e-6 {
            tmax
        } else {
            return Intersect::empty();
        };

        let point = *ray_origin + *ray_direction * t;
        let (u, v) = self.get_uv(&point, &normal);

        Intersect::new(self.material.clone(), t, normal, point, u, v)
    }
}

/// Helper para evitar repetición de `abs` en ejes
#[inline]
fn axis_value_abs(v: f32) -> f32 {
    v.abs()
}
