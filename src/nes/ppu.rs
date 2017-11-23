/// Emulated screen width in pixels.
pub const SCREEN_WIDTH: usize = 256;
/// Emulated screen height in pixels.
pub const SCREEN_HEIGHT: usize = 240;
/// Number of pixels in the emulated screen.
pub const PIXEL_COUNT: usize = SCREEN_WIDTH * SCREEN_HEIGHT;
/// Screen texture size in bytes.
pub const SCREEN_SIZE: usize = PIXEL_COUNT * 3;

pub struct Ppu {
    pub cycle: usize,
    pub screen: Box<[u8; SCREEN_SIZE]>,
}

impl Ppu {
    pub fn new() -> Ppu {
        Ppu {
            cycle: 0,
            screen: Box::new([0x00; SCREEN_SIZE]),
        }
    }

    pub fn step(&mut self) {
        for i in 0..(PIXEL_COUNT - 1) {
            self.screen[i * 3] = self.cycle as u8;
            self.screen[i * 3 + 1] = 0;
            self.screen[i * 3 + 2] = 0;
        }

        self.screen[115 * SCREEN_WIDTH + (self.cycle * 3)] = 255;
        self.screen[115 * SCREEN_WIDTH + (self.cycle * 3) + 1] = 255;
        self.screen[115 * SCREEN_WIDTH + (self.cycle * 3) + 2] = 255;

        self.cycle = self.cycle + 1;;
    }
}