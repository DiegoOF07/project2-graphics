use raylib::prelude::*;

/// Cámara básica para un raytracer.
/// - `eye`: posición de la cámara (desde dónde miramos)
/// - `center`: punto al que la cámara mira
/// - `up`: vector que define la orientación vertical de la cámara
/// - `forward`, `right`: base ortonormal que se recalcula cuando la cámara cambia
pub struct Camera {
    pub eye: Vector3,
    pub center: Vector3,
    pub up: Vector3,
    pub forward: Vector3,
    pub right: Vector3,
    changed: bool, // indica si la cámara fue modificada desde la última consulta
}

impl Camera {
    /// Crea una cámara en la posición `eye`, mirando hacia `center`,
    /// con orientación vertical definida por `up`.
    pub fn new(eye: Vector3, center: Vector3, up: Vector3) -> Self {
        let mut cam = Self {
            eye,
            center,
            up,
            forward: Vector3::zero(),
            right: Vector3::zero(),
            changed: true,
        };
        cam.update_basis();
        cam
    }

    /// Recalcula la base ortonormal de la cámara (forward, right, up).
    /// Debe llamarse siempre que `eye` o `center` cambien.
    #[inline]
    pub fn update_basis(&mut self) {
        self.forward = (self.center - self.eye).normalized();
        self.right = self.forward.cross(self.up).normalized();
        self.up = self.right.cross(self.forward); // asegura ortogonalidad
        self.changed = true;
    }

    /// Realiza un movimiento orbital alrededor del `center`.
    /// - `yaw`: rotación horizontal (rad)
    /// - `pitch`: rotación vertical (rad)
    pub fn orbit(&mut self, yaw: f32, pitch: f32) {
        let rel = self.eye - self.center; // vector desde el centro hacia la cámara
        let radius = rel.length();

        // ángulos actuales en coordenadas esféricas
        let current_yaw = rel.z.atan2(rel.x);
        let current_pitch = (rel.y / radius).asin();

        // aplicar desplazamientos
        let new_yaw = current_yaw + yaw;
        let new_pitch = (current_pitch + pitch).clamp(-1.5, 1.5);

        // convertir de nuevo a coordenadas cartesianas
        let new_rel = Vector3::new(
            radius * new_pitch.cos() * new_yaw.cos(),
            radius * new_pitch.sin(),
            radius * new_pitch.cos() * new_yaw.sin(),
        );

        self.eye = self.center + new_rel;
        self.update_basis();
    }

    /// Acerca o aleja la cámara hacia/desde el `center`.
    pub fn zoom(&mut self, amount: f32) {
        let dir = (self.center - self.eye).normalized();
        self.eye += dir * amount;
        self.update_basis();
    }

    /// Devuelve `true` si la cámara cambió desde la última consulta.
    pub fn is_changed(&mut self) -> bool {
        let was_changed = self.changed;
        self.changed = false; // resetea bandera
        was_changed
    }

    /// Convierte un punto en espacio local de la cámara a coordenadas
    /// en el sistema de la cámara (right, up, forward).
    #[inline]
    pub fn basis_change(&self, p: &Vector3) -> Vector3 {
        Vector3::new(
            p.x * self.right.x + p.y * self.up.x - p.z * self.forward.x,
            p.x * self.right.y + p.y * self.up.y - p.z * self.forward.y,
            p.x * self.right.z + p.y * self.up.z - p.z * self.forward.z,
        )
    }
}
