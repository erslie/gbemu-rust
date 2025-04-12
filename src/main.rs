mod bootrom;
mod gameboy;
mod hram;
mod wram;
mod peripherals;
mod register;

fn main() {
    gameboy::run();
    println!("Hello, world!");
}
