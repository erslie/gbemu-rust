mod bootrom;
mod gameboy;
mod hram;
mod wram;
mod peripherals;
mod register;
mod instruction;
mod operand;
mod cpu;


fn main() {
    gameboy::run();
    println!("Hello, world!");
}
