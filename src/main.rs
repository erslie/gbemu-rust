use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use core::panic;
use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

use gbemu_rust::bootrom::Bootrom;
use gbemu_rust::gameboy::GameBoy;
use gbemu_rust::cartridge::Cartridge;

fn main() {
    //gameboy::run();
    unsafe {
        backtrace_on_stack_overflow::enable();
    }
    let bootrom_path = PathBuf::from("asset/dmg_bootrom.bin");
    println!("load to {:?}", bootrom_path);
    let boot_vec = file2vec(&"asset/dmg_bootrom.bin".to_string());
    let bootrom = Bootrom::new(boot_vec);
    let cartridge_box = file_to_boxed_slice(&"asset/cpu_instrs.gb").unwrap();
    let cartridge = Cartridge::new(cartridge_box);
    let mut gb = GameBoy::new(bootrom, cartridge);
    gb.run();
}

fn file2vec(fname: &String) -> Vec<u8> {
    if let Ok(mut file) = File::open(fname) {
        let mut ret = vec![];
        file.read_to_end(&mut ret).unwrap();
        ret
    } else {
        panic!("Cannot open {}", fname);
    }
}

fn file_to_boxed_slice(path: &str) -> io::Result<Box<[u8]>> {
    let vec_data = file2vec(&path.to_string());
    let boxed_slice:Box<[u8]> = vec_data.into_boxed_slice();
    Ok(boxed_slice)
}

fn test_sdl2() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Rust SDL2 Test", 800, 600)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode:Some(Keycode::Escape), .. } => {
                    println!("pushed escape");
                    break 'running
                },
                _ => {}
            }
        }
        
    }


}
