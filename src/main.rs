mod bootrom;
mod gameboy;
mod hram;
mod wram;
mod peripherals;

fn main() {
    gameboy::run();
    println!("Hello, world!");
}
