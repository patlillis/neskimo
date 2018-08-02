use nes::ppu::{SCREEN_HEIGHT, SCREEN_SIZE, SCREEN_WIDTH};
use sdl2::pixels::PixelFormatEnum::BGR24;
use sdl2::render::{Canvas, Texture, TextureAccess, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::{init, EventPump, Sdl};
use time::PreciseTime;

const FONT_HEIGHT: usize = 10;
const FONT_GLYPH_COUNT: usize = 95;
const FONT_GLYPH_LENGTH: usize = FONT_GLYPH_COUNT * FONT_HEIGHT;

const STATUS_LINE_PADDING: usize = 6;
const STATUS_LINE_X: usize = STATUS_LINE_PADDING;
const STATUS_LINE_Y: usize = SCREEN_HEIGHT - STATUS_LINE_PADDING - FONT_HEIGHT;

//
// PT Ronda Seven
//
// (c) Yusuke Kamiyamane, http://pinvoke.com/
//

#[cfg_attr(rustfmt, rustfmt_skip)]
const FONT_GLYPHS: [u8; FONT_GLYPH_LENGTH] = [
      0,   0,   0,   0,   0,   0,   0,   0,   0,   0,  // ' '
      0,  64,  64,  64,  64,  64,   0,  64,   0,   0,  // '!'
      0, 160, 160,   0,   0,   0,   0,   0,   0,   0,  // '"'
      0,  80,  80, 248,  80, 248,  80,  80,   0,   0,  // '#'
     32, 112, 168, 160, 112,  40, 168, 112,  32,   0,  // '$'
      0,  66, 164,  72,  16,  36,  74, 132,   0,   0,  // '%'
      0,  96, 144, 160,  72, 168, 144, 104,   0,   0,  // '&'
      0, 128, 128,   0,   0,   0,   0,   0,   0,   0,  // '''
     32,  64, 128, 128, 128, 128, 128,  64,  32,   0,  // '('
    128,  64,  32,  32,  32,  32,  32,  64, 128,   0,  // ')'
      0,  32, 168, 112, 168,  32,   0,   0,   0,   0,  // '*'
      0,   0,  32,  32, 248,  32,  32,   0,   0,   0,  // '+'
      0,   0,   0,   0,   0,   0,   0,  64,  64, 128,  // ','
      0,   0,   0,   0,   0, 224,   0,   0,   0,   0,  // '-'
      0,   0,   0,   0,   0,   0,   0,  64,   0,   0,  // '.'
      8,   8,  16,  16,  32,  64,  64, 128, 128,   0,  // '/'
      0, 112, 136, 136, 136, 136, 136, 112,   0,   0,  // '0'
      0, 192,  64,  64,  64,  64,  64,  64,   0,   0,  // '1'
      0, 112, 136,   8,  16,  32,  64, 248,   0,   0,  // '2'
      0, 112, 136,   8,  48,   8, 136, 112,   0,   0,  // '3'
      0,  48,  80,  80, 144, 248,  16,  16,   0,   0,  // '4'
      0, 248, 128, 128, 240,   8, 136, 112,   0,   0,  // '5'
      0, 112, 136, 128, 240, 136, 136, 112,   0,   0,  // '6'
      0, 248,   8,  16,  16,  32,  32,  64,   0,   0,  // '7'
      0, 112, 136, 136, 112, 136, 136, 112,   0,   0,  // '8'
      0, 112, 136, 136, 120,   8, 136, 112,   0,   0,  // '9'
      0,   0,   0,  64,   0,   0,   0,  64,   0,   0,  // ':'
      0,   0,   0,  64,   0,   0,   0,  64,  64, 128,  // ';'
      0,   0,  32,  64, 128,  64,  32,   0,   0,   0,  // '<'
      0,   0,   0, 224,   0, 224,   0,   0,   0,   0,  // '='
      0,   0, 128,  64,  32,  64, 128,   0,   0,   0,  // '>'
      0, 112, 136,   8,  16,  32,   0,  32,   0,   0,  // '?'
     60,  66, 157, 165, 165, 173, 149,  66,  56,   0,  // '@'
      0, 112, 136, 136, 248, 136, 136, 136,   0,   0,  // 'A'
      0, 240, 136, 136, 240, 136, 136, 240,   0,   0,  // 'B'
      0, 112, 136, 128, 128, 128, 136, 112,   0,   0,  // 'C'
      0, 240, 136, 136, 136, 136, 136, 240,   0,   0,  // 'D'
      0, 248, 128, 128, 240, 128, 128, 248,   0,   0,  // 'E'
      0, 248, 128, 128, 240, 128, 128, 128,   0,   0,  // 'F'
      0, 112, 136, 128, 184, 136, 152, 104,   0,   0,  // 'G'
      0, 136, 136, 136, 248, 136, 136, 136,   0,   0,  // 'H'
      0, 128, 128, 128, 128, 128, 128, 128,   0,   0,  // 'I'
      0,  16,  16,  16,  16,  16, 144,  96,   0,   0,  // 'J'
      0, 136, 144, 160, 192, 160, 144, 136,   0,   0,  // 'K'
      0, 128, 128, 128, 128, 128, 128, 240,   0,   0,  // 'L'
      0, 130, 198, 170, 146, 130, 130, 130,   0,   0,  // 'M'
      0, 136, 200, 168, 168, 168, 152, 136,   0,   0,  // 'N'
      0, 112, 136, 136, 136, 136, 136, 112,   0,   0,  // 'O'
      0, 240, 136, 136, 240, 128, 128, 128,   0,   0,  // 'P'
      0, 112, 136, 136, 136, 136, 136, 112,   8,   0,  // 'Q'
      0, 240, 136, 136, 240, 160, 144, 136,   0,   0,  // 'R'
      0, 112, 136, 128, 112,   8, 136, 112,   0,   0,  // 'S'
      0, 248,  32,  32,  32,  32,  32,  32,   0,   0,  // 'T'
      0, 136, 136, 136, 136, 136, 136, 112,   0,   0,  // 'U'
      0, 136, 136,  80,  80,  80,  32,  32,   0,   0,  // 'V'
      0, 146, 146, 146, 146, 146, 146, 108,   0,   0,  // 'W'
      0, 136, 136,  80,  32,  80, 136, 136,   0,   0,  // 'X'
      0, 136, 136,  80,  32,  32,  32,  32,   0,   0,  // 'Y'
      0, 248,   8,  16,  32,  64, 128, 248,   0,   0,  // 'Z'
    224, 128, 128, 128, 128, 128, 128, 128, 224,   0,  // '['
    128, 128,  64,  64,  32,  16,  16,   8,   8,   0,  // '\'
    224,  32,  32,  32,  32,  32,  32,  32, 224,   0,  // ']'
      0,  64, 160,   0,   0,   0,   0,   0,   0,   0,  // '^'
      0,   0,   0,   0,   0,   0,   0, 224,   0,   0,  // '_'
      0,   0,   0,   0,   0,   0,   0,   0,   0,   0,  // '`'
      0,   0,   0, 112, 144, 144, 176,  80,   0,   0,  // 'a'
      0, 128, 128, 160, 208, 144, 144, 224,   0,   0,  // 'b'
      0,   0,   0,  96, 144, 128, 144,  96,   0,   0,  // 'c'
      0,  16,  16, 112, 144, 144, 176,  80,   0,   0,  // 'd'
      0,   0,   0,  96, 144, 240, 128,  96,   0,   0,  // 'e'
      0,  96, 128, 192, 128, 128, 128, 128,   0,   0,  // 'f'
      0,   0,   0, 112, 144, 144, 176,  80,  16,  96,  // 'g'
      0, 128, 128, 160, 208, 144, 144, 144,   0,   0,  // 'h'
      0, 128,   0, 128, 128, 128, 128, 128,   0,   0,  // 'i'
      0, 128,   0, 128, 128, 128, 128, 128, 128,   0,  // 'j'
      0, 128, 128, 144, 160, 192, 160, 144,   0,   0,  // 'k'
      0, 128, 128, 128, 128, 128, 128, 128,   0,   0,  // 'l'
      0,   0,   0, 164, 218, 146, 146, 146,   0,   0,  // 'm'
      0,   0,   0, 160, 208, 144, 144, 144,   0,   0,  // 'n'
      0,   0,   0,  96, 144, 144, 144,  96,   0,   0,  // 'o'
      0,   0,   0, 160, 208, 144, 144, 224, 128, 128,  // 'p'
      0,   0,   0, 112, 144, 144, 176,  80,  16,  16,  // 'q'
      0,   0,   0, 160, 192, 128, 128, 128,   0,   0,  // 'r'
      0,   0,   0,  96, 128,  64,  32, 192,   0,   0,  // 's'
      0,  64,  64, 224,  64,  64,  64,  64,   0,   0,  // 't'
      0,   0,   0, 144, 144, 144, 176,  80,   0,   0,  // 'u'
      0,   0,   0, 144, 144, 144, 144,  96,   0,   0,  // 'v'
      0,   0,   0, 146, 146, 146, 146, 108,   0,   0,  // 'w'
      0,   0,   0, 144, 144,  96, 144, 144,   0,   0,  // 'x'
      0,   0,   0, 144, 144, 144, 176,  80,  16,  96,  // 'y'
      0,   0,   0, 224,  32,  64, 128, 224,   0,   0,  // 'z'
     32,  64,  64,  64, 128,  64,  64,  64,  32,   0,  // '{'
    128, 128, 128, 128, 128, 128, 128, 128, 128,   0,  // '|'
    128,  64,  64,  64,  32,  64,  64,  64, 128,   0,  // '}'
      0,  80, 160,   0,   0,   0,   0,   0,   0,   0,  // '~'
];

#[cfg_attr(rustfmt, rustfmt_skip)]
const FONT_ADVANCES: [u8; FONT_GLYPH_COUNT] =
    [3 /*   */, 3 /* ! */, 4 /* " */, 6 /* # */, 6 /* $ */,
     8 /* % */, 6 /* & */, 2 /* ' */, 4 /* ( */, 4 /* ) */,
     6 /* * */, 6 /* + */, 3 /* , */, 4 /* - */, 3 /* . */,
     5 /* / */, 6 /* 0 */, 3 /* 1 */, 6 /* 2 */, 6 /* 3 */,
     6 /* 4 */, 6 /* 5 */, 6 /* 6 */, 6 /* 7 */, 6 /* 8 */,
     6 /* 9 */, 3 /* : */, 3 /* ; */, 4 /* < */, 4 /* = */,
     4 /* > */, 6 /* ? */, 8 /* @ */, 6 /* A */, 6 /* B */,
     6 /* C */, 6 /* D */, 6 /* E */, 6 /* F */, 6 /* G */,
     6 /* H */, 2 /* I */, 5 /* J */, 6 /* K */, 5 /* L */,
     8 /* M */, 6 /* N */, 6 /* O */, 6 /* P */, 6 /* Q */,
     6 /* R */, 6 /* S */, 6 /* T */, 6 /* U */, 6 /* V */,
     8 /* W */, 6 /* X */, 6 /* Y */, 6 /* Z */, 4 /* [ */,
     6 /* \ */, 4 /* ] */, 4 /* ^ */, 4 /* _ */, 3 /* ` */,
     5 /* a */, 5 /* b */, 5 /* c */, 5 /* d */, 5 /* e */,
     3 /* f */, 5 /* g */, 5 /* h */, 2 /* i */, 2 /* j */,
     5 /* k */, 2 /* l */, 8 /* m */, 5 /* n */, 5 /* o */,
     5 /* p */, 5 /* q */, 4 /* r */, 4 /* s */, 4 /* t */,
     5 /* u */, 5 /* v */, 8 /* w */, 5 /* x */, 5 /* y */,
     4 /* z */, 4 /* { */, 2 /* | */, 4 /* } */, 5 /* ~ */];

//
// Text output
//

enum GlyphColor {
    White,
    Black,
}

fn draw_glyph(
    pixels: &mut [u8],
    surface_width: usize,
    x: isize,
    y: isize,
    color: GlyphColor,
    glyph_index: usize,
) {
    let color_byte = match color {
        GlyphColor::White => 0xff,
        GlyphColor::Black => 0x00,
    };
    for y_index in 0..10 {
        let row = FONT_GLYPHS[glyph_index * 10 + y_index as usize];
        for x_index in 0..8 {
            if ((row >> (7 - x_index) as usize) & 1) != 0 {
                for channel in 0..3 {
                    let mut index =
                        (y + y_index) * (surface_width as isize) * 3 + (x + x_index) * 3;
                    index += channel;

                    if index >= 0 && index < pixels.len() as isize {
                        pixels[index as usize] = color_byte;
                    }
                }
            }
        }
    }
}

pub fn draw_text(pixels: &mut [u8], surface_width: usize, mut x: isize, y: isize, string: &str) {
    for i in 0..string.len() {
        let glyph_index = (string.as_bytes()[i] - 32) as usize;
        if glyph_index < FONT_ADVANCES.len() {
            draw_glyph(
                pixels,
                surface_width,
                x,
                y + 1,
                GlyphColor::Black,
                glyph_index,
            ); // Shadow
            draw_glyph(pixels, surface_width, x, y, GlyphColor::White, glyph_index); // Main
            x += FONT_ADVANCES[glyph_index] as isize;
        }
    }
}

pub struct Gfx<'a> {
    pub canvas: Canvas<Window>,
    pub texture: Texture<'a>,
    pub texture_creator: TextureCreator<WindowContext>,
    pub events: EventPump,
    show_fps: bool,
    // Used to measure FPS.
    last_render: PreciseTime,
}

impl<'a> Gfx<'a> {
    pub fn new(show_fps: bool) -> (Gfx<'a>, Sdl) {
        // FIXME: Handle SDL better

        let sdl = init().unwrap();
        let mut window_builder =
            sdl.video()
                .unwrap()
                .window("neskimo", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32);
        let window = window_builder.position_centered().build().unwrap();

        let canvas = window
            .into_canvas()
            .accelerated()
            .present_vsync()
            .build()
            .unwrap();

        let texture_creator = canvas.texture_creator();
        let texture = texture_creator
            .create_texture(
                BGR24,
                TextureAccess::Streaming,
                SCREEN_WIDTH as u32,
                SCREEN_HEIGHT as u32,
            )
            .unwrap();

        let events = sdl.event_pump().unwrap();

        (
            Gfx {
                canvas: canvas,
                texture_creator: texture_creator,
                texture: texture,
                events: events,
                show_fps: show_fps,
                last_render: PreciseTime::now(),
            },
            sdl,
        )
    }

    /// Copies the overlay onto the given screen and displays it to the SDL window.
    pub fn composite(&mut self, ppu_screen: &mut [u8; SCREEN_SIZE]) {
        let new_time = PreciseTime::now();
        let duration_millis = self.last_render.to(new_time).num_milliseconds();
        let duration = duration_millis as f32 / 1000.0;
        let fps = if duration == 0.0 { 0.0 } else { 1.0 / duration };
        self.last_render = new_time;

        if self.show_fps {
            draw_text(
                ppu_screen,
                SCREEN_WIDTH,
                STATUS_LINE_X as isize,
                STATUS_LINE_Y as isize,
                &format!("FPS: {:.1}", fps),
            );
        }

        self.blit(ppu_screen);
        self.canvas.clear();
        self.canvas.copy(&self.texture, None, None).ok();
        self.canvas.present();
    }

    /// Updates the window texture with new screen data.
    fn blit(&mut self, ppu_screen: &[u8; SCREEN_SIZE]) {
        self.texture
            .update(None, ppu_screen, SCREEN_WIDTH * 3)
            .unwrap()
    }
}
