use sdl2::pixels::PixelFormatEnum::BGR24;
use sdl2::render::{Renderer, Texture, TextureAccess};
use sdl2::{init, Sdl};

/// Emulated screen width in pixels
pub const SCREEN_WIDTH: usize = 256;
/// Emulated screen height in pixels
pub const SCREEN_HEIGHT: usize = 240;
/// Screen texture size in bytes
pub const SCREEN_SIZE: usize = SCREEN_WIDTH * SCREEN_HEIGHT * 3;

pub struct Gfx<'a> {
    pub renderer: Box<Renderer<'a>>,
    pub texture: Box<Texture>,
}

impl<'a> Gfx<'a> {
    pub fn new() -> (Gfx<'a>, Sdl) {
        // FIXME: Handle SDL better

        let sdl = init().unwrap();

        let mut window_builder = sdl.video()
            .unwrap()
            .window("neskimo", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32);
        let window = window_builder.position_centered().build().unwrap();

        let renderer = window
            .renderer()
            .accelerated()
            .present_vsync()
            .build()
            .unwrap();
        let texture = renderer
            .create_texture(BGR24,
                            TextureAccess::Streaming,
                            SCREEN_WIDTH as u32,
                            SCREEN_HEIGHT as u32)
            .unwrap();

        (Gfx {
             renderer: Box::new(renderer),
             texture: Box::new(texture),
         },
         sdl)
    }

    /// Copies the overlay onto the given screen and displays it to the SDL window.
    pub fn composite(&mut self, ppu_screen: &[u8; SCREEN_SIZE]) {
        self.blit(ppu_screen);
        self.renderer.clear();
        self.renderer.copy(&self.texture, None, None);
        self.renderer.present();
    }

    /// Updates the window texture with new screen data.
    fn blit(&mut self, ppu_screen: &[u8; SCREEN_SIZE]) {
        self.texture
            .update(None, ppu_screen, SCREEN_WIDTH * 3)
            .unwrap()
    }
}