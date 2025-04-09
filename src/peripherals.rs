use crate::bootrom::Bootrom;
use crate::hram::HRam;
use crate::wram::WRam;//atode seiri
pub struct Peripherals {
    bootrom: Bootrom,
    wram: WRam,
    hram: HRam,
}
impl Peripherals {
    pub fn new(bootrom: Bootrom) -> Self {
        Self { 
            bootrom,
            wram: WRam::new(),
            hram: HRam::new(),
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x00FF => if self.bootrom.is_active() {
                self.bootrom.read(addr)
            } else {
                0xFF
            },
            0xC000..=0xFDFF => self.wram.read(addr),
            0xFF80..=0xFFFE => self.hram.read(addr),
            _ => 0xFF,
            }
        }
        pub fn write(&mut self, addr: u16, val: u8) {
            match addr {
                0xC000..=0xFDFF => self.wram.write(addr,val),
                0xFF50          => self.bootrom.write(addr, val),
                0xFF80..0xFFFF  => self.hram.write(addr, val),
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