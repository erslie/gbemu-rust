use sdl2::pixels::PixelFormatEnum;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;

use crate::ppu::LCD_WIDTH;
use crate::ppu::LCD_HEIGHT;

pub struct LCD(Canvas<Window>);
impl LCD {
    pub fn new(sdl: &Sdl, scale: u32) -> LCD {
        let window = sdl
            .video()
            .expect("failed to initialize SDL video subsystem")
            .window(" gb-emu test", 800, 600)
            .position_centered()
            .build()
            .unwrap();
        let  canvas = window.into_canvas().build().unwrap();
        Self(canvas)
    }

    pub fn draw(&mut self, pixels: Box<[u8]>) {
        let texture_creator = self.0.texture_creator();
        let mut texture = texture_creator
            .create_texture_streaming(PixelFormatEnum::RGB24, LCD_WIDTH as u32, LCD_HEIGHT as u32)
            .unwrap();
        texture.update(None, &pixels, 480).unwrap();
        self.0.clear();
        self.0.copy(&texture, None, None).unwrap();
        self.0.present();
        println!("DRAW CALLED!");
    }
}