use crate::peripherals::Peripherals;


/*
            IE/IF bit | ISR addr
VBLANK ->   0           0x0040
STAT   ->   1           0x0048
TIMER  ->   2           0x0050
SERIAL ->   3           0x0058
JOYPAD ->   4           0x0060
*/
pub const VBLANK: u8 = 1 << 0;
pub const STAT: u8 = 1 << 1;
pub const TIMER: u8 = 1 << 2;
pub const SERIAL: u8 = 1 << 3;
pub const JOYPAD: u8 = 1 << 4;


#[derive(Default)]
pub struct Interrupts {
    pub ime: bool,
    pub int_flags: u8,
    pub int_enable: u8,
}
impl Interrupts {
    pub fn irq(&mut self, val: u8) {
        self.int_flags |= val;
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0xFF0F => self.int_flags,
            0xFFFF => self.int_enable,
            _ => unreachable!(),
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0xFF0F => self.int_flags = val,
            0xFFFF => self.int_enable = val,
            _ => unreachable!(),
        }
    }

    pub fn get_interrupt(&self) -> u8 {
        self.int_flags & self.int_enable & 0b11111
    }

}

