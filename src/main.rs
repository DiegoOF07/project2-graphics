mod framebuffer;

use framebuffer::Framebuffer;
use raylib::prelude::*;

fn main() {
    let window_width = 930;
    let window_height = 630;
    let block_size = 30 as usize;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Raytracer Scene")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    window.set_target_fps(60);
    window.hide_cursor();

    let framebuffer_width = 930;
    let framebuffer_height = 630;
    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height, Color::BLACK);
    while !window.window_should_close() {
        framebuffer.clear();
        framebuffer.swap_buffers(&mut window, &raylib_thread, |d| {
            d.draw_text(&format!("FPS: {}", d.get_fps()), 10, 10, 20, Color::WHITE);
        });
    }
}
