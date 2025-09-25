// textures.rs - Versión mejorada
use raylib::prelude::*;
use std::collections::HashMap;

/// Textura cargada en memoria de CPU con interpolación bilinear
struct CpuTexture {
    width: i32,
    height: i32,
    pixels: Vec<Vector3>, // Valores normalizados [0,1]
}

impl CpuTexture {
    /// Convierte una `Image` de Raylib en una textura CPU-friendly
    fn from_image(image: &Image) -> Self {
        let colors = image.get_image_data();
        let pixels = colors
            .iter()
            .map(|c| Vector3::new(
                c.r as f32 / 255.0,
                c.g as f32 / 255.0,
                c.b as f32 / 255.0,
            ))
            .collect();

        Self {
            width: image.width,
            height: image.height,
            pixels,
        }
    }

    /// Obtiene color con interpolación bilinear para mejores resultados
    fn sample_bilinear(&self, u: f32, v: f32) -> Vector3 {
        // Clamp UV coordinates
        let u = u.clamp(0.0, 1.0);
        let v = v.clamp(0.0, 1.0);
        
        // Convert to texture coordinates
        let x = u * (self.width - 1) as f32;
        let y = v * (self.height - 1) as f32;
        
        // Get integer and fractional parts
        let x0 = x.floor() as i32;
        let y0 = y.floor() as i32;
        let x1 = (x0 + 1).min(self.width - 1);
        let y1 = (y0 + 1).min(self.height - 1);
        
        let fx = x - x0 as f32;
        let fy = y - y0 as f32;
        
        // Sample four corners
        let c00 = self.get_pixel_clamped(x0, y0);
        let c10 = self.get_pixel_clamped(x1, y0);
        let c01 = self.get_pixel_clamped(x0, y1);
        let c11 = self.get_pixel_clamped(x1, y1);
        
        // Bilinear interpolation
        let c0 = c00 + (c10 - c00) * fx;
        let c1 = c01 + (c11 - c01) * fx;
        c0 + (c1 - c0) * fy
    }

    /// Obtiene pixel con clamping a bordes
    fn get_pixel_clamped(&self, x: i32, y: i32) -> Vector3 {
        let x = x.clamp(0, self.width - 1);
        let y = y.clamp(0, self.height - 1);
        let idx = (y * self.width + x) as usize;
        
        self.pixels.get(idx).copied().unwrap_or(Vector3::one())
    }

    /// Convierte textura en normal map
    fn sample_normal(&self, u: f32, v: f32) -> Vector3 {
        let color = self.sample_bilinear(u, v);
        Vector3::new(
            color.x * 2.0 - 1.0,
            color.y * 2.0 - 1.0,
            color.z.max(0.0), // Mantener Z positivo para normal maps
        ).normalized()
    }
}

/// Gestor de texturas mejorado
pub struct TextureManager {
    cpu_textures: HashMap<String, CpuTexture>,
    gpu_textures: HashMap<String, Texture2D>,
}

impl TextureManager {
    pub fn new() -> Self {
        Self::default()
    }

    /// Carga una textura desde archivo
    pub fn load_texture(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread, path: &str) -> Result<(), String> {
        if self.gpu_textures.contains_key(path) {
            return Ok(()); // Ya está cargada
        }

        let image = Image::load_image(path)
            .map_err(|_| format!("No se pudo cargar la imagen: {}", path))?;

        let texture = rl
            .load_texture_from_image(thread, &image)
            .map_err(|_| format!("No se pudo crear la textura: {}", path))?;

        self.cpu_textures.insert(path.to_string(), CpuTexture::from_image(&image));
        self.gpu_textures.insert(path.to_string(), texture);
        
        Ok(())
    }

    /// Obtiene color con interpolación bilinear (para raytracer)
    pub fn sample_texture(&self, path: &str, u: f32, v: f32) -> Vector3 {
        self.cpu_textures
            .get(path)
            .map(|tex| tex.sample_bilinear(u, v))
            .unwrap_or(Vector3::one()) // Color blanco por defecto
    }

    /// Obtiene normal desde normal map
    pub fn sample_normal_map(&self, path: &str, u: f32, v: f32) -> Vector3 {
        self.cpu_textures
            .get(path)
            .map(|tex| tex.sample_normal(u, v))
            .unwrap_or(Vector3::new(0.0, 0.0, 1.0)) // Normal hacia arriba por defecto
    }

    /// Obtiene textura de GPU para rendering directo
    pub fn get_gpu_texture(&self, path: &str) -> Option<&Texture2D> {
        self.gpu_textures.get(path)
    }

    /// Obtiene un pixel exacto de la textura en coordenadas (x,y)
    /// Devuelve blanco si no existe
    pub fn get_pixel_color(&self, path: &str, x: i32, y: i32) -> Vector3 {
        if let Some(tex) = self.cpu_textures.get(path) {
            tex.get_pixel_clamped(x, y)
        } else {
            Vector3::one() // fallback blanco
        }
    }

    /// devuelve (width,height) o (0,0) si no existe
    pub fn size_of(&self, path: &str) -> Option<(u32,u32)> {
        self.cpu_textures.get(path).map(|t| (t.width as u32, t.height as u32))
    }

    pub fn width_of(&self, path: &str) -> u32 {
        self.cpu_textures.get(path).map(|t| t.width as u32).unwrap_or(0)
    }

    pub fn height_of(&self, path: &str) -> u32 {
        self.cpu_textures.get(path).map(|t| t.height as u32).unwrap_or(0)
    }
}


impl Default for TextureManager {
    fn default() -> Self {
        Self {
            cpu_textures: HashMap::new(),
            gpu_textures: HashMap::new(),
        }
    }
}