mod bootrom;
mod gameboy;
mod hram;
mod wram;

fn main() {
    gameboy::run();
    println!("Hello, world!");
}
