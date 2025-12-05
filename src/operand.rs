use crate::peripherals::*;
use crate::cpu::*;
use std::sync::atomic::AtomicU8;
use std::sync::atomic::AtomicU16;
use std::sync::atomic::Ordering::Relaxed;

macro_rules! step {
    ($cpu:ident, $d:expr, {$($c:tt : $e:expr,)*}) => {
        $(if $cpu.exec_state.step == $c { $e })* else { return $d; }
    };
} 
pub(crate) use step;
macro_rules! go {
    ($cpu:ident, $d:expr) => {
        $cpu.exec_state.step = $d;
    }
}
pub(crate) use go;

pub trait IO8<T: Copy> {
    fn read8(&mut self, bus:&Peripherals, src: T) -> Option<u8>;
    fn write8(&mut self, bus: &mut Peripherals, dst: T, val: u8) -> Option<()>;
}
pub trait IO16<T: Copy> {
    fn read16(&mut self, bus: &Peripherals, src: T) -> Option<u16>;
    fn write16(&mut self, bus: &mut Peripherals, dst: T, val: u16) -> Option<()>;
}

#[derive(Clone, Copy, Debug)]
pub enum Reg8 { A, B, C, D, E, H, L }
#[derive(Clone, Copy, Debug)]
pub enum Reg16 { AF, BC, DE, HL, SP }
#[derive(Clone, Copy, Debug)]
pub struct Imm8;
#[derive(Clone, Copy, Debug)]
pub struct Imm16;
#[derive(Clone, Copy, Debug)]
pub enum Indirect { BC, DE, HL, CFF, HLD, HLI }
#[derive(Clone, Copy, Debug)]
pub enum Direct8 { D, DFF }
#[derive(Clone, Copy, Debug)]
pub struct Direct16;
#[derive(Clone, Copy, Debug)]
pub enum Cond { NZ, Z, NC, C }

impl IO8<Reg8> for Cpu {
    fn read8(&mut self, _: &Peripherals, src: Reg8) -> Option<u8> {
        Some(match src {
            Reg8::A => self.regs.a,
            Reg8::B => self.regs.b,
            Reg8::C => self.regs.c,
            Reg8::D => self.regs.d,
            Reg8::E => self.regs.e,
            Reg8::H => self.regs.h,
            Reg8::L => self.regs.l,
        })
    }
    fn write8(&mut self, _: &mut Peripherals, dst: Reg8, val: u8) -> Option<()> {
        Some(match dst {
            Reg8::A => self.regs.a = val,
            Reg8::B => self.regs.b = val,
            Reg8::C => self.regs.c = val,
            Reg8::D => self.regs.d = val,
            Reg8::E => self.regs.e = val,
            Reg8::H => self.regs.h = val,
            Reg8::L => self.regs.l = val,
        })
    }
}

impl IO16<Reg16> for Cpu {
    fn read16(&mut self, _: &Peripherals, src: Reg16) -> Option<u16> {
        Some(match src {
            Reg16::AF => self.regs.af(),
            Reg16::BC => self.regs.bc(),
            Reg16::DE => self.regs.de(),
            Reg16::HL => self.regs.hl(),
            Reg16::SP => self.regs.sp,
        })
    }

    fn write16(&mut self, bus: &mut Peripherals, dst: Reg16, val: u16) -> Option<()> {
        Some(match dst {
            Reg16::AF => self.regs.write_af(val),
            Reg16::BC => self.regs.write_bc(val),
            Reg16::DE => self.regs.write_de(val),
            Reg16::HL => self.regs.write_hl(val),
            Reg16::SP => self.regs.sp = val,
        })
    }
}

impl IO8<Imm8> for Cpu {
    fn read8(&mut self, bus:&Peripherals, _: Imm8) -> Option<u8> {
        step!(self, None, {
            0: {
                self.exec_state.val8 = bus.read(&mut self.interrupts, self.regs.pc);
                self.regs.pc = self.regs.pc.wrapping_add(1);
                go!(self, 1);
                return None;
            },
            1: {
                go!(self, 0);
                return Some(self.exec_state.val8);
            },
        });
    }
    fn write8(&mut self, bus: &mut Peripherals, dst: Imm8, val: u8) -> Option<()> {
        unreachable!()
    }
}

impl IO16<Imm16> for Cpu {
    fn read16(&mut self, bus: &Peripherals, _: Imm16) -> Option<u16> {
        step!(self, None, {
            0: if let Some(lo) = self.read8(bus, Imm8) {
                self.exec_state.val8 = lo;
                go!(self, 1);
            },
            1: if let Some(hi) = self.read8(bus, Imm8) {
                self.exec_state.val16 = u16::from_le_bytes([self.exec_state.val8, hi]);
                go!(self, 2);
            },
            2: {
                go!(self, 0);
                return Some(self.exec_state.val16);
            },
        });
    }
    
    fn write16(&mut self, bus: &mut Peripherals, dst: Imm16, val: u16) -> Option<()> {
        unreachable!()
    }
}

impl IO8<Indirect> for Cpu {
    fn read8(&mut self, bus:&Peripherals, src: Indirect) -> Option<u8> {
        step!(self, None, {
            0: {
                self.exec_state.val8 = match src {
                    Indirect::BC => bus.read(&mut self.interrupts, self.regs.bc()),
                    Indirect::DE => bus.read(&mut self.interrupts, self.regs.de()),
                    Indirect::HL => bus.read(&mut self.interrupts, self.regs.hl()),
                    Indirect::CFF => bus.read(&mut self.interrupts, 0xFF00 | (self.regs.c as u16)),
                    Indirect::HLD => {
                        let addr = self.regs.hl();
                        //sub,addは実際の命令だと読み書きの後に行ってそう…
                        self.regs.write_hl(addr.wrapping_sub(1));
                        bus.read(&mut self.interrupts, addr)
                    },
                    Indirect::HLI => {
                        let addr = self.regs.hl();
                        self.regs.write_hl(addr.wrapping_add(1));
                        bus.read(&mut self.interrupts, addr)
                    },
                };
                go!(self, 1);
                return None;
            },
            1: {
                go!(self, 0);
                return Some(self.exec_state.val8);
            },
        });
    }
    
    fn write8(&mut self, bus: &mut Peripherals, dst: Indirect, val: u8) -> Option<()> {
        step!(self, None, {
            0: {
                match dst {
                Indirect::BC => bus.write(&mut self.interrupts, self.regs.bc(), val),
                Indirect::DE => bus.write(&mut self.interrupts, self.regs.de(), val),
                Indirect::HL => bus.write(&mut self.interrupts, self.regs.hl(), val),
                Indirect::CFF => bus.write(&mut self.interrupts, 0xFF00 | (self.regs.c as u16), val),
                Indirect::HLD => {
                    let addr = self.regs.hl();
                    self.regs.write_hl(addr.wrapping_sub(1));
                    bus.write(&mut self.interrupts, addr, val);
                },
                Indirect::HLI => {
                    let addr = self.regs.hl();
                    self.regs.write_hl(addr.wrapping_add(1));
                    bus.write(&mut self.interrupts, addr, val);
                },
            }
            go!(self, 1);
            return None;
        },
        1: return Some(go!(self, 0)),
        });
    }
}

impl IO8<Direct8> for Cpu {
    fn read8(&mut self, bus:&Peripherals, src: Direct8) -> Option<u8> {
        step!(self, None, {
            0: if let Some(lo) = self.read8(bus, Imm8) {
                self.exec_state.val8 = lo;
                go!(self, 1);
                if let Direct8::DFF = src {
                    self.exec_state.val16 = 0xFF00 | (lo as u16);
                    go!(self, 2);
                }
            },
            1: if let Some(hi) = self.read8(bus, Imm8) {
                self.exec_state.val16 = u16::from_le_bytes([self.exec_state.val8, hi]);
                // VAL16.store(u16::from_le_bytes([VAL8.load(Relaxed), hi]), Relaxed);
                go!(self, 2);
            },
            2: {
                self.exec_state.val8 = bus.read(&mut self.interrupts, self.exec_state.val16);
                // VAL8.store(bus.read(&mut self.interrupts, VAL16.load(Relaxed)), Relaxed);
                go!(self, 3);
                return None;
            },
            3: {
                go!(self, 0);
                return Some(self.exec_state.val8);
            },
        });
    }

    fn write8(&mut self, bus: &mut Peripherals, dst: Direct8, val: u8) -> Option<()> {
        step!(self, None, {
            0: if let Some(lo) = self.read8(bus, Imm8) {
                self.exec_state.val8 = lo;
                // VAL8.store(lo, Relaxed);
                go!(self, 1);
                if let Direct8::DFF = dst {
                    self.exec_state.val16 = 0xff | (lo as u16);
                    // VAL16.store(0xFF00 | (lo as u16), Relaxed);
                    go!(self, 2);
                }
            },
            1: if let Some(hi) = self.read8(bus, Imm8) {
                self.exec_state.val16 = u16::from_le_bytes([self.exec_state.val8, hi]);
                // VAL16.store(u16::from_le_bytes([VAL8.load(Relaxed), hi]), Relaxed);
                go!(self, 2); 
            },
            2: {
                bus.write(&mut self.interrupts, self.exec_state.val16, val);
                go!(self, 3);
                return None;
            },
            3: return Some(go!(self, 0)),
        });
    }
}

impl IO16<Direct16> for Cpu {
    fn read16(&mut self, bus: &Peripherals, src: Direct16) -> Option<u16> {
         unreachable!()
    }

    fn write16(&mut self, bus: &mut Peripherals, dst: Direct16, val: u16) -> Option<()> {
        step!(self, None, {
            0: if let Some(lo) = self.read8(bus, Imm8) {
                self.exec_state.val8 = lo;
                // VAL8.store(lo, Relaxed);
                go!(self, 1);
            },
            1: if let Some(hi) = self.read8(bus, Imm8) {
                self.exec_state.val16 = u16::from_le_bytes([self.exec_state.val8, hi]);
                // VAL16.store(u16::from_le_bytes([VAL8.load(Relaxed), hi]), Relaxed);
                go!(self, 2);
            },
            2: {
                bus.write(&mut self.interrupts, self.exec_state.val16, val as u8);
                go!(self, 3);
                return None;
            },
            3: {
                bus.write(&mut self.interrupts, self.exec_state.val16.wrapping_add(1), (val >> 8) as u8);
                go!(self, 4);
                return None;
            },
            4: return Some(go!(self, 0)),
        });
    }
}

#[cfg(test)]
mod test {
    use crate::{cpu::*, operand::Reg16};
    use crate::operand::IO16;
    use crate::register::*;
    use crate::peripherals::Peripherals;
    use super::{Imm8, Reg8, IO8};
    use crate::bootrom::*;

    #[test]
    fn  test_reg8() {
        let mut cpu = Cpu { regs:Registres::default(), ctx: Ctx::default() };
        let mut peri = Peripherals::new(Bootrom::new(vec![0,0x00]));
        cpu.write8(&mut peri, Reg8::A, 0x30);
        assert_eq!(0x30, cpu.read8(&mut peri,Reg8::A).unwrap());
    }
    #[test]
    fn  test_reg16_af() {
        let mut cpu = Cpu { regs:Registres::default(), ctx: Ctx::default() };
        let mut peri = Peripherals::new(Bootrom::new(vec![0,0x00]));
        cpu.write16(&mut peri, Reg16::AF, 0b_0011_0011_1010_1111);
        //Fレジスタが下位4bitを無効化できていれば成功
        assert_eq!(0b0011_0011_1010_0000, cpu.read16(&mut peri,Reg16::AF).unwrap());
    }
    #[test]
    fn test_reg16_bc() {
        let mut cpu = Cpu { regs:Registres::default(), ctx: Ctx::default() };
        let mut peri = Peripherals::new(Bootrom::new(vec![0,0x00]));
        cpu.write16(&mut peri, Reg16::BC, 0b_0011_0011_1010_1111);
        assert_eq!(0b0011_0011_1010_1111, cpu.read16(&mut peri,Reg16::BC).unwrap());
    }
    #[test]
    fn test_imm8 () {
        let mut cpu = Cpu { regs:Registres::default(), ctx: Ctx::default() };
        //二回呼び出ししないとpcのインクリメントが確認できないから少なくとも2バイトはRomに持たせておく必要がある
        let mut peri = Peripherals::new(Bootrom::new(vec![0x12,0x34]));
        let initial_pc = cpu.regs.pc;
        // 最初の read（None を返すはず）
        let result1 = cpu.read8(&peri, Imm8);
        assert_eq!(result1, None);
        assert_eq!(cpu.regs.pc, initial_pc.wrapping_add(1));

        // 2回目の read（Some(0x12) を返すはず）
        let result2 = cpu.read8(&peri, Imm8);
        assert_eq!(result2, Some(0x12));
        assert_eq!(cpu.regs.pc, initial_pc.wrapping_add(1));
    }
}
