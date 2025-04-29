use crate::register::*;
use crate::peripherals::*;

#[derive(Default)]
pub struct Ctx {
    opcode: u8,
    cb: bool,
}
#[derive(Default)]
pub struct Cpu {
    pub regs: Registres,
    pub ctx: Ctx,
}
impl Cpu {
    pub fn fetch(&mut self, bus: &Peripherals) {
        self.ctx.opcode = bus.read(self.regs.pc);
        self.regs.pc = self.regs.pc.wrapping_add(1);
        self.ctx.cb = false;
    }

    pub fn decode(&mut self, bus: &mut Peripherals) {
        match self.ctx.opcode {
            0x00 => self.nop(bus),
            _    => panic!("Not implemented: {:02x}", self.ctx.opcode),
        }
    }

    pub fn emulate_cycle(&mut self, bus: &mut Peripherals) {
        self.decode(bus);
    }
}

#[cfg(test)]
mod unit_test {
    use crate::{bootrom::Bootrom, register::Registres};
    use super::*;

    #[test]
    fn test_fetch() {
        let mut cpu = Cpu { regs:Registres::default(), ctx: Ctx::default() };
        let mut peri = Peripherals::new(Bootrom::new(vec![0,0x00]));
        cpu.fetch(&peri);
        assert_eq!(0x00, cpu.ctx.opcode);
        assert_eq!(1, cpu.regs.pc);
        assert!(!cpu.ctx.cb);
    }

    //decodexeいいテスト思い浮かばない
}