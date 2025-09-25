use raylib::prelude::*;

pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    buffer: Vec<u32>,
    texture: Option<Texture2D>,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            buffer: vec![0; (width * height) as usize],
            texture: None,
        }
    }

    #[inline]
    pub fn clear(&mut self, color: u32) {
        self.buffer.fill(color);
    }

    #[inline]
    pub fn set_pixel(&mut self, x: u32, y: u32, color: u32) {
        if x < self.width && y < self.height {
            let idx = (y * self.width + x) as usize;
            self.buffer[idx] = color;
        }
    }

    fn ensure_texture(&mut self, d: &mut RaylibDrawHandle, thread: &RaylibThread) {
        if self.texture.is_none() {
            let img = Image::gen_image_color(self.width as i32, self.height as i32, Color::BLACK);
            self.texture = Some(
                d.load_texture_from_image(thread, &img)
                    .expect("No se pudo crear textura"),
            );
        }
    }

    pub fn present(&mut self, d: &mut RaylibDrawHandle, thread: &RaylibThread) {
        self.ensure_texture(d, thread);

        if let Some(ref mut texture) = self.texture {
            unsafe {
                let raw = std::slice::from_raw_parts(
                    self.buffer.as_ptr() as *const u8,
                    self.buffer.len() * 4,
                );
                raylib::ffi::UpdateTexture(*texture.as_ref(), raw.as_ptr() as *const _);
            }

            d.draw_texture(texture, 0, 0, Color::WHITE);
        }
    }

    pub fn present_scaled(
        &mut self, 
        d: &mut RaylibDrawHandle, 
        thread: &RaylibThread, 
        source: Rectangle, 
        dest: Rectangle
    ) {
        self.ensure_texture(d, thread);

        if let Some(ref mut texture) = self.texture {
            unsafe {
                let raw = std::slice::from_raw_parts(
                    self.buffer.as_ptr() as *const u8,
                    self.buffer.len() * 4,
                );
                raylib::ffi::UpdateTexture(*texture.as_ref(), raw.as_ptr() as *const _);
            }

            d.draw_texture_pro(texture, source, dest, Vector2::zero(), 0.0, Color::WHITE);
        }
    }
}

#[inline]
pub fn color_to_u32(c: Color) -> u32 {
    ((c.a as u32) << 24) | ((c.b as u32) << 16) | ((c.g as u32) << 8) | (c.r as u32)
}