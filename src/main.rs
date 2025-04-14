mod bootrom;
mod gameboy;
mod hram;
mod wram;
mod peripherals;
mod register;
mod operand;
mod cpu;

fn main() {
    gameboy::run();
    println!("Hello, world!");
}
