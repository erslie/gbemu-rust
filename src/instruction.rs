use crate::peripherals::{self, Peripherals};
use crate::cpu::*;
use crate::operand::{
    Reg16, 
    Imm16,
    Imm8,
    Cond,
    IO8,
    IO16,
    step,
    go,
};
use std::sync::atomic::{
    AtomicU8, 
    AtomicU16, 
    Ordering::Relaxed,
};


impl Cpu {
    pub fn nop(&mut self, bus: &Peripherals) {
        self.fetch(bus);
    }

    pub fn ld<D: Copy, S: Copy>(&mut self, bus:&mut Peripherals, dst: D, src: S)
    where Self: IO8<D> + IO8<S> {
        step!((), {
            0: if let Some(v) = self.read8(bus, src) {
                VAL8.store(v, Relaxed);
                go!(1);
            },
            1: if self.write8(bus, dst, VAL8.load(Relaxed)).is_some() {
                go!(2);
            },
            2: {
              go!(0);
                self.fetch(bus);
            },      
            });
    }

    pub fn ld16<D: Copy, S: Copy>(&mut self, bus: &mut Peripherals, dst: D, src: S)
    where Self: IO16<D> + IO16<S> {
        step!((), {
            0: if let Some(v) = self.read16(bus, src) {
                VAL16.store(v, Relaxed);
                go!(1);
            },
            1: if self.write16(bus, dst, VAL16.load(Relaxed)).is_some() {
                go!(2);
            },
            2: {go!(0);
            self.fetch(bus);
            }, 
        });
    }

    pub fn cp<S: Copy>(&mut self, bus: &Peripherals, src: S)
    where Self: IO8<S> {
        if let Some(v) = self.read8(bus, src) {
            let (result, carry) = self.regs.a.overflowing_sub(v);
            self.regs.set_zf(result == 0);
            self.regs.set_nf(true);
            self.regs.set_hf((self.regs.a & 0xf) < (v & 0xf));
            self.regs.set_cf(carry);
            self.fetch(bus);
        }
    }

    pub fn inc<S: Copy>(&mut self, bus: &mut Peripherals, src: S)
    where Self: IO8<S> {
        step!((), {
            0: if let Some(v) = self.read8(bus, src) {
                let result = v.wrapping_add(1);
                self.regs.set_zf(result == 0);
                self.regs.set_nf(false);
                self.regs.set_hf(v & 0xf == 0xf);
                VAL8.store(result, Relaxed);
                go!(1);
            },
            1: if self.write8(bus, src, VAL8.load(Relaxed)).is_some() {
                go!(0);
                self.fetch(bus);
            },
        });
    }
    pub fn inc16<S: Copy>(&mut self, bus: &mut Peripherals, src: S)
    where Self: IO16<S> {
        step!((), {
            0: if let Some(v) = self.read16(bus, src) {
                VAL16.store(v.wrapping_add(1), Relaxed);
                go!(1);
            },
            1: if self.write16(bus, src, VAL16.load(Relaxed)).is_some() {
                return go!(2);
            },
            2: {
                go!(0);
                self.fetch(bus);
            },
        });
    }

    pub fn dec<S: Copy>(&mut self, bus: &mut Peripherals, src: S)
    where Self: IO8<S> {
        step!((), {
            0: if let Some(v) = self.read8(bus, src) {
                let result = v.wrapping_sub(1);
                self.regs.set_zf(result == 0);
                self.regs.set_nf(true);
                self.regs.set_hf(v & 0xf == 0);
                VAL8.store(result, Relaxed);
                go!(1);
            },
            1: if self.write8(bus, src, VAL8.load(Relaxed)).is_some() {
                go!(0);
                self.fetch(bus);
            },
        });
    }

    pub fn dec16<S: Copy>(&mut self, bus: &mut Peripherals, src: S)
    where Self: IO16<S> {
        step!((), {
            0: if let Some(v) = self.read16(bus, src) {
                VAL16.store(v.wrapping_sub(1), Relaxed);
                go!(1);
            },
            1: if self.write16(bus, src, VAL16.load(Relaxed)).is_some() {
                return go!(2);
            },
            2: {
                go!(0);
                self.fetch(bus);
            },
        });
    }

    pub fn rl<S: Copy>(&mut self, bus: &mut Peripherals, src: S)
    where Self: IO8<S> {
        step!((), {
            0: if let Some(v) = self.read8(bus, src) {
                let result = (v << 1) | self.regs.cf() as u8;
                self.regs.set_zf(result == 0);
                self.regs.set_nf(false);
                self.regs.set_hf(false);
                self.regs.set_cf(v & 0x80 > 0);
                VAL8.store(result, Relaxed);
                go!(1);
            },
            1: if self.write8(bus, src, VAL8.load(Relaxed)).is_some() {
                go!(0);
                self.fetch(bus);
            },
        });
    }

    pub fn bit<S: Copy>(&mut self, bus: &Peripherals, bit: usize, src: S)
    where  Self: IO8<S> {
        if let Some(mut v) = self.read8(bus, src) {
            v & 1 << bit;
            self.regs.set_zf(v == 0);
            self.regs.set_nf(false);
            self.regs.set_hf(true);
            self.fetch(bus);
        }
    }

    pub fn push16(&mut self, bus: &mut Peripherals, val: u16) -> Option<()> {
        step!(None, {
            0: {
                go!(1);
                return None;
            },
            1: {
                let [lo, hi] = u16::to_le_bytes(val);
                self.regs.sp = self.regs.sp.wrapping_sub(1);
                bus.write(self.regs.sp, hi);
                VAL8.store(lo, Relaxed);
                go!(2);
                return None;
            },
            2: {
                self.regs.sp = self.regs.sp.wrapping_sub(1);
                bus.write(self.regs.sp, VAL8.load(Relaxed));
                go!(3);
                return None;
            },
            3: return Some(go!(0)),
        });
    }

    pub fn push(&mut self, bus: &mut Peripherals, src: Reg16) {
        step!((), {
            0: {
                VAL16.store(self.read16(bus, src).unwrap(), Relaxed);
                go!(1);
            },
            1: if self.push16(bus, VAL16.load(Relaxed)).is_some() {
                go!(2);
            },
            2: {
                go!(0);
                self.fetch(bus);
            },
        });
    }

    pub fn pop16(&mut self, bus: &Peripherals) -> Option<u16> {
        step!(None, {
            0: {
                VAL8.store(bus.read(self.regs.sp), Relaxed);
                self.regs.sp = self.regs.sp.wrapping_add(1);
                go!(1);
                return None;
            },
            1: {
                let hi = bus.read(self.regs.sp);
                self.regs.sp = self.regs.sp.wrapping_add(1);
                VAL16.store(u16::from_le_bytes([VAL8.load(Relaxed), hi]), Relaxed);
                go!(2);
                return None;
            },
            2: {
                go!(0);
                return Some(VAL16.load(Relaxed));
            },
        });
    }

    pub fn pop(&mut self, bus:&mut Peripherals, dst: Reg16) {
        if let Some(v) = self.pop16(bus) {
            self.write16(bus, dst, v);
            self.fetch(bus);
        }
    }

    pub fn jr(&mut self, bus: &Peripherals) {
        step!((), {
            0: if let Some(v) = self.read8(bus, Imm8) {
                self.regs.pc = self.regs.pc.wrapping_add(v as i8 as u16);
                return go!(1); 
            },
            1: {
                go!(0);
                self.fetch(bus);
            },
        });
    }

    fn cond(&self, cond: Cond) -> bool {
        match cond {
            Cond::NZ => !self.regs.zf(),
            Cond::Z => self.regs.zf(),
            Cond::NC => !self.regs.cf(),
            Cond::C => self.regs.cf(),
        }
    }

    pub fn fr_c(&mut self, bus: &Peripherals, c: Cond) {
        step!((), {
            0: if let Some(v) = self.read8(bus, Imm8) {
                go!(1);
                if self.cond(c) {
                    self.regs.pc = self.regs.pc.wrapping_add(v as i8 as u16);
                    return;
                }
            },
            1: {
                go!(0);
                self.fetch(bus);
            },
        });
    }

    pub fn call(&mut self, bus: &mut Peripherals) {
        step!((), {
            0: if let Some(v) = self.read16(bus, Imm16) {
                VAL16.store(v, Relaxed);
                go!(1);
            },
            1: if self.push16(bus, self.regs.pc).is_some() {
                self.regs.pc = VAL16.load(Relaxed);
                go!(0);
                self.fetch(bus); 
            },
        });
    }

    pub fn ret(&mut self, bus: &Peripherals) {
        step!((), {
            0: if let Some(v) = self.pop16(bus) {
                self.regs.pc = v;
                return go!(1);
            },
            1: {
                go!(0);
                self.fetch(bus);
            },
        });
    }
}

#[cfg(test)]
mod test {
    use std::thread::sleep;
    use std::time::Duration;
    use std::vec;
    use crate::{cpu, instruction::*};
    use crate::operand::Reg8;
    use crate::register::Registres;
    use crate::{bootrom::Bootrom, peripherals::Peripherals};

    fn peri() -> Peripherals {
        Peripherals::new(Bootrom::new(vec![0x12, 0x34]))
    }
    fn cpu() -> Cpu {  
        Cpu { regs:Registres::default(), ctx: Ctx::default() }
    }

    //test codes
    #[test]
    fn test_nop() {
        let peri = peri();
        let mut cpu = cpu();
        assert_eq!(0, cpu.regs.pc);
        cpu.nop(&peri);
        assert_eq!(1, cpu.regs.pc);
    }
    #[test]
    fn test_ld() {
        let mut peri = peri();
        let mut cpu = cpu();
        cpu.write8(&mut peri, Reg8::A, 0xFF);
        assert_eq!(0, cpu.regs.pc);
        cpu.ld(&mut peri, Reg8::B, Reg8::A);
        assert_eq!(0xFF, cpu.read8(&mut peri, Reg8::A).unwrap());
    }
    #[test]
    fn test_ld16() {
        let mut peri = peri();
        let mut cpu = cpu();
        cpu.write16(&mut peri, Reg16::BC, 0xFFFF);
        assert_eq!(0, cpu.regs.pc);
        cpu.ld16(&mut peri, Reg16::DE, Reg16::BC);
        assert_eq!(0xFFFF, cpu.read16(&mut peri, Reg16::DE).unwrap());
        assert_eq!(1, cpu.regs.pc);
    }
    #[test]
    fn test_cp() {
        let mut peri = peri();
        let mut cpu = cpu();

        cpu.write8(&mut peri, Reg8::A, 0x0a);
        cpu.write8(&mut peri, Reg8::B, 0x0a);
        cpu.cp(&mut peri, Reg8::B);
        assert!(cpu.regs.zf());
        assert!(cpu.regs.nf());
        assert!(!cpu.regs.hf());
        assert!(!cpu.regs.cf());
        assert_eq!(1, cpu.regs.pc);
        println!("RegA:{}, RegB:{}", cpu.regs.a, cpu.regs.b);

        cpu.write8(&mut peri, Reg8::B, 0x0b);
        cpu.cp(&mut peri, Reg8::B);
        assert!(!cpu.regs.zf());
        assert!(cpu.regs.nf());
        assert!(cpu.regs.hf());
        assert!(cpu.regs.cf());
        assert_eq!(2,cpu.regs.pc);
        println!("RegA:{}, RegB:{}", cpu.regs.a, cpu.regs.b);
    }
    #[test]
    fn test_inc() {
        let mut peri = peri();
        let mut cpu = cpu();

        cpu.write8(&mut peri, Reg8::A, 0x0e);
        cpu.inc(&mut peri, Reg8::A);
        println!("RegA:{}", cpu.regs.a);
        assert_eq!(0x0f, cpu.regs.a);
        cpu.inc(&mut peri, Reg8::A);
        println!("RegA:{}", cpu.regs.a);
        assert_eq!(0x10, cpu.regs.a);
    }

    //HACK: stepのreleaseが出来ていないことが発覚したがbootrom起動チェックまではこのままやってみる
    #[test]
    fn test_inc16() {
        let mut peri = peri();
        let mut cpu = cpu();

        cpu.write16(&mut peri, Reg16::BC, 0xFFFE);
        println!("pc:{}", cpu.regs.pc);
        
        cpu.inc16(&mut peri, Reg16::BC);
        println!("pc:{}", cpu.regs.pc);
        println!("RegBC:{}", cpu.regs.bc());
        assert_eq!(0xFFFF,cpu.regs.bc());

        cpu.inc16(&mut peri, Reg16::BC);
        println!("pc:{}", cpu.regs.pc);
        println!("RegBC:{}", cpu.regs.bc());
        // assert_eq!(0x0000, cpu.regs.bc());
    }
    

}