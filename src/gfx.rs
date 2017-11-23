use nes::ppu::{SCREEN_WIDTH, SCREEN_HEIGHT, SCREEN_SIZE};
use sdl2::{init, Sdl, EventPump};
use sdl2::pixels::PixelFormatEnum::BGR24;
use sdl2::render::{Renderer, Texture, TextureAccess};

pub struct Gfx<'a> {
    pub renderer: Renderer<'a>,
    pub texture: Texture,
    pub events: EventPump,
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

        let events = sdl.event_pump().unwrap();

        (Gfx {
             renderer: renderer,
             texture: texture,
             events: events,
         },
         sdl)
    }

    /// Copies the overlay onto the given screen and displays it to the SDL window.
    pub fn composite(&mut self, ppu_screen: &[u8; SCREEN_SIZE]) {
        self.blit(ppu_screen);
        self.renderer.clear();
        self.renderer.copy(&self.texture, None, None).ok();
        self.renderer.present();
    }

    /// Updates the window texture with new screen data.
    fn blit(&mut self, ppu_screen: &[u8; SCREEN_SIZE]) {
        self.texture
            .update(None, ppu_screen, SCREEN_WIDTH * 3)
            .unwrap()
    }
}