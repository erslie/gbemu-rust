use crate::bootrom::Bootrom;
use crate::cartridge::Cartridge;
use crate::hram::HRam;
use crate::interruputs::{self, Interrupts};
use crate::wram::WRam;//atode seiri
use crate::ppu::Ppu;
pub struct Peripherals {
    bootrom: Bootrom,
    wram: WRam,
    cartridge: Cartridge,
    hram: HRam,
    pub ppu: Ppu,
}
impl Peripherals {
    pub fn new(bootrom: Bootrom, cartridge: Cartridge) -> Self {
        Self { 
            bootrom,
            cartridge,
            wram: WRam::new(),
            hram: HRam::new(),
            ppu: Ppu::new(),
        }
    }

    pub fn read(&self, interrupts: &Interrupts, addr: u16) -> u8 {
        match addr {
            0x0000..=0x00FF => if self.bootrom.is_active() {
                self.bootrom.read(addr)
            } else {
                self.cartridge.read(addr)
            },
            0x0100..=0x7FFF => self.cartridge.read(addr),
            0xA000..=0xBFFF => self.cartridge.read(addr),
            0x8000..=0x9FFF => self.ppu.read(addr),
            0xFE00..=0xFE9F => self.ppu.read(addr),
            0xFF40..=0xFF4B => self.ppu.read(addr),
            0xC000..=0xFDFF => self.wram.read(addr),
            0xFF80..=0xFFFE => self.hram.read(addr),
            0xFF0F => interrupts.read(addr),
            0xFFFF => interrupts.read(addr),
            _ => 0xFF,
            }
        }
        
        pub fn write(&mut self, interrupts: &mut Interrupts, addr: u16, val: u8) {
            match addr {
                0x0000..=0x00FF => if !self.bootrom.is_active() {
                    self.cartridge.write(addr, val)
                }
                0x0100..=0x7FFF => self.cartridge.write(addr, val),
                0xA000..=0xBFFF => self.cartridge.write(addr, val),
                0x8000..=0x9FFF => self.ppu.write(addr, val),
                0xFE00..=0xFE9F => self.ppu.write(addr, val),
                0xFF40..=0xFF4B => self.ppu.write(addr, val),
                0xC000..=0xFDFF => self.wram.write(addr,val),
                0xFF50          => self.bootrom.write(addr, val),
                0xFF80..=0xFFFE  => self.hram.write(addr, val),
                0xFF0F => interrupts.write(addr, val),
                0xFFFF => interrupts.write(addr, val),
                _ => (),
            }
        }
}


#[cfg(test)]
mod test {
    use crate::bootrom::Bootrom;
    use super::Peripherals;

    #[test]
    fn test_readwrite_wram() {
        let mut peri = Peripherals::new(Bootrom::new(vec![0,0]));
        peri.write(0xC000, 0x42);
        assert_eq!(0x42, peri.read(0xC000));
    }
    #[test]
    fn test_readwrite_hram() {
        let mut peri = Peripherals::new(Bootrom::new(vec![0,0]));
        peri.write(0xFF80, 0x42);
        assert_eq!(0x42, peri.read(0xFF80));
    }
    #[test]
    fn test_readwrite_bootrom() {
        let mut peri = Peripherals::new(Bootrom::new(vec![0,0]));
        peri.write(0xFF50, 1);
        assert_eq!(false, peri.bootrom.is_active());
    }
}