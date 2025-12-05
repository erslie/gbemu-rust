use std::time;
pub const CPU_CLOCK_HZ: u128 = 4_194_304;//一秒間に4194304クロック
pub const M_CYCLE_CLOCK: u128 = 4;//gbマシンサイクルが4クロック
const M_CYCLE_NANOS: u128 = M_CYCLE_CLOCK * 1_000_000_000 / CPU_CLOCK_HZ;//1マシンサイクル

use crate::cartridge::Cartridge;
use crate::cpu::Cpu;
use crate::peripherals::Peripherals;
use crate::lcd::LCD;
use crate::bootrom::Bootrom;

pub struct GameBoy {
    cpu: Cpu,
    peripherals: Peripherals,
    lcd: LCD,
}
impl GameBoy {
    pub fn new(bootrom: Bootrom, cartridge: Cartridge) -> Self {
        let sdl = sdl2::init().expect("failed to initialize SDL");
        let lcd = LCD::new(&sdl, 4);
        let peripherals = Peripherals::new(bootrom, cartridge);
        let cpu = Cpu::default();
        Self {
            cpu,
            peripherals,
            lcd,
        }
    }

    pub fn run(&mut self) {
        let time = time::Instant::now();
        let mut elapsed = 0;
        loop {
            let e = time.elapsed().as_nanos();
            for _ in 0..(e - elapsed) / M_CYCLE_NANOS {
                self.cpu.emulate_cycle(&mut self.peripherals);
                if self.peripherals.ppu.emulate_cycle() {
                    self.lcd.draw(self.peripherals.ppu.pixel_buffer());
                }
                println!("{}", elapsed);
                elapsed += M_CYCLE_NANOS;
            }
        }
    }

}
