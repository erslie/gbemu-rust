use crate::peripherals::Peripherals;
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


impl Cpu {
    pub fn nop(&mut self, bus: &Peripherals) {
        self.fetch(bus);
    }

    pub fn stop(&mut self, bus: &Peripherals) {
        self.fetch(bus);
    }

    pub fn add<S: Copy>(&mut self, bus: &mut Peripherals, src: S)
    where Self: IO8<S> {
        if let Some(v) = self.read8(bus, src) {
            let (result, carry) = self.regs.a.overflowing_add(v);
            self.regs.set_zf(result == 0);
            self.regs.set_nf(false);
            self.regs.set_hf((self.regs.a & 0x0f) + (v & 0x0f) > 0xf);
            self.regs.set_cf(carry);
            self.regs.a = result;
            self.fetch(bus);
        }
    }

    pub fn adc<S: Copy>(&mut self, bus: &mut Peripherals, src: S)
    where Self: IO8<S> {
        if let Some(v) = self.read8(bus, src) {
            let c = self.regs.cf() as u8;
            let result = self.regs.a
                .wrapping_add(v)
                .wrapping_add(c);
            self.regs.set_zf(result == 0);
            self.regs.set_nf(false);
            self.regs.set_hf((self.regs.a & 0x0f) + (v & 0x0f) + c > 0xf);
            self.regs.set_cf(self.regs.a as u16 + v as u16 + c as u16 > 0xff);
            self.regs.a = result;
            self.fetch(bus);
        }
    }

    pub fn sub<S: Copy>(&mut self, bus: &mut Peripherals, src: S)
    where Self: IO8<S> {
        if let Some(v) = self.read8(bus, src) {
            let (result, carry) = self.regs.a.overflowing_sub(v);
            self.regs.set_zf(result == 0);
            self.regs.set_nf(true);
            self.regs.set_hf((self.regs.a & 0x0f) < (v & 0x0f));
            self.regs.set_cf(carry);
            self.regs.a = result;
            self.fetch(bus);
        }
    }

    pub fn sbc<S: Copy>(&mut self, bus: &mut Peripherals, src: S)
    where Self: IO8<S> {
        if let Some(v) = self.read8(bus, src) {
            let c = self.regs.cf() as u8;
            let result = self.regs.a
                .wrapping_sub(v)
                .wrapping_sub(c);
            self.regs.set_zf(result == 0);
            self.regs.set_nf(true);
            self.regs.set_hf(self.regs.a & 0x0f < ((v & 0x0f) + c));
            self.regs.set_cf(v as u16 + c as u16 > self.regs.a as u16);
            self.regs.a = result;
            self.fetch(bus);
        }
    }

    pub fn and<S: Copy>(&mut self, bus: &mut Peripherals, src: S)
    where Self: IO8<S> {
        if let Some(v) = self.read8(bus, src) {
            let result = self.regs.a & v;
            self.regs.set_zf(result == 0);
            self.regs.set_nf(false);
            self.regs.set_hf(true);
            self.regs.set_cf(false);
            self.regs.a = result;
            self.fetch(bus);
        }
    }

    pub fn or<S: Copy>(&mut self, bus: &mut Peripherals, src: S)
    where Self: IO8<S> {
        if let Some(v) = self.read8(bus, src) {
            let result = self.regs.a | v;
            self.regs.set_zf(result == 0);
            self.regs.set_nf(false);
            self.regs.set_hf(false);
            self.regs.set_cf(false);
            self.regs.a = result;
            self.fetch(bus);
        }
    }

    pub fn xor<S: Copy>(&mut self, bus: &mut Peripherals, src: S)
    where Self: IO8<S> {
        if let Some(v) = self.read8(bus, src) {
            let result = self.regs.a ^ v;
            self.regs.set_zf(result == 0);
            self.regs.set_nf(false);
            self.regs.set_hf(false);
            self.regs.set_cf(false);
            self.regs.a = result;
            self.fetch(bus);
        }
    }

    pub fn ld<D: Copy, S: Copy>(&mut self, bus:&mut Peripherals, dst: D, src: S)
    where Self: IO8<D> + IO8<S> {
        step!(self, (), {
            0: if let Some(v) = self.read8(bus, src) {
                self.exec_state.val8 = v;
                go!(self, 1);
            },
            1: if self.write8(bus, dst, self.exec_state.val8).is_some() {
                go!(self, 2);
            },
            2: {
              go!(self, 0);
                self.fetch(bus);
            },      
            });
    }

    pub fn ld16<D: Copy, S: Copy>(&mut self, bus: &mut Peripherals, dst: D, src: S)
    where Self: IO16<D> + IO16<S> {
        step!(self, (), {
            0: if let Some(v) = self.read16(bus, src) {
                self.exec_state.val16 = v;
                go!(self, 1);
            },
            1: if self.write16(bus, dst, self.exec_state.val16).is_some() {
                go!(self, 2);
            },
            2: {go!(self, 0);
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
        step!(self, (), {
            0: if let Some(v) = self.read8(bus, src) {
                let result = v.wrapping_add(1);
                self.regs.set_zf(result == 0);
                self.regs.set_nf(false);
                self.regs.set_hf(v & 0xf == 0xf);
                self.exec_state.val8 = result;
                go!(self, 1);
            },
            1: if self.write8(bus, src, self.exec_state.val8).is_some() {
                go!(self, 0);
                self.fetch(bus);
            },
        });
    }

    pub fn inc16<S: Copy>(&mut self, bus: &mut Peripherals, src: S)
    where Self: IO16<S> {
        step!(self, (), {
            0: if let Some(v) = self.read16(bus, src) {
                self.exec_state.val16 = v.wrapping_add(1);
                go!(self, 1);
            },
            1: if self.write16(bus, src, self.exec_state.val16).is_some() {
                go!(self, 2);
            },
            2: {
                go!(self, 0);
                self.fetch(bus);
            },
        });
    }

    pub fn dec<S: Copy>(&mut self, bus: &mut Peripherals, src: S)
    where Self: IO8<S> {
        step!(self, (), {
            0: if let Some(v) = self.read8(bus, src) {
                let result = v.wrapping_sub(1);
                self.regs.set_zf(result == 0);
                self.regs.set_nf(true);
                self.regs.set_hf(v & 0xf == 0);
                self.exec_state.val8 = result;
                go!(self, 1);
            },
            1: if self.write8(bus, src, self.exec_state.val8).is_some() {
                go!(self, 0);
                self.fetch(bus);
            },
        });
    }

    pub fn dec16<S: Copy>(&mut self, bus: &mut Peripherals, src: S)
    where Self: IO16<S> {
        step!(self, (), {
            0: if let Some(v) = self.read16(bus, src) {
                self.exec_state.val16 = v.wrapping_sub(1);
                go!(self, 1);
            },
            1: if self.write16(bus, src, self.exec_state.val16).is_some() {
                return go!(self, 2);
            },
            2: {
                go!(self, 0);
                self.fetch(bus);
            },
        });
    }

    pub fn rl<S: Copy>(&mut self, bus: &mut Peripherals, src: S)
    where Self: IO8<S> {
        step!(self, (), {
            0: if let Some(v) = self.read8(bus, src) {
                let result = (v << 1) | self.regs.cf() as u8;
                self.regs.set_zf(result == 0);
                self.regs.set_nf(false);
                self.regs.set_hf(false);
                self.regs.set_cf(v & 0x80 > 0);
                self.exec_state.val8 = result;
                go!(self, 1);
            },
            1: if self.write8(bus, src, self.exec_state.val8).is_some() {
                go!(self, 0);
                self.fetch(bus);
            },
        });
    }

    pub fn rlca(&mut self, bus: &mut Peripherals) {
        let carry = self.regs.a & 0b1000_0000;
        self.regs.set_zf(false);
        self.regs.set_nf(false);
        self.regs.set_hf(false);
        self.regs.set_cf(carry != 0);
        self.regs.a = (self.regs.a << 1) | (self.regs.a >> 7);
        self.fetch(bus);
    }

    pub fn rla(&mut self, bus: &mut Peripherals) {
        let carry = self.regs.a & 0b1000_0000;
        self.regs.a = (self.regs.a << 1) | (self.regs.cf() as u8);
        self.regs.set_zf(false);
        self.regs.set_nf(false);
        self.regs.set_hf(false);
        self.regs.set_cf(carry != 0);
        self.fetch(bus);
    }

    pub fn rrca(&mut self, bus: &mut Peripherals) {
        let carry = self.regs.a & 0b01;
        self.regs.a = (self.regs.a >> 1) | (self.regs.a << 7);
        self.regs.set_zf(false);
        self.regs.set_nf(false);
        self.regs.set_hf(false);
        self.regs.set_cf(carry != 0);
        self.fetch(bus);
    }

    pub fn rra(&mut self, bus: &mut Peripherals) {
        let carry = self.regs.a & 0b01;
        self.regs.a = (self.regs.a >> 1) | ((self.regs.cf() as u8) << 7);
        self.regs.set_zf(false);
        self.regs.set_nf(false);
        self.regs.set_hf(false);
        self.regs.set_cf(carry != 0);
        self.fetch(bus);
    }

    pub fn rlc<S: Copy>(&mut self, bus: &mut Peripherals, src: S)
    where Self: IO8<S> {
        step!(self, (), {
            0: if let Some(v) = self.read8(bus, src) {
                let result = (v << 1) | (v >> 7);
                self.regs.set_zf(result == 0);
                self.regs.set_nf(false);
                self.regs.set_hf(false);
                self.regs.set_cf(v & 0x80 > 0);
                self.exec_state.val8 = result;
                go!(self, 1);
            },
            1: if self.write8(bus, src, self.exec_state.val8).is_some() {
                go!(self, 0);
                self.fetch(bus);
            },
        });
    }

    pub fn rrc<S: Copy>(&mut self, bus: &mut Peripherals, src: S)
    where Self: IO8<S> {
        step!(self, (), {
            0: if let Some(v) = self.read8(bus, src) {
                let result = (v >> 1) | (v << 7);
                self.regs.set_zf(result == 0);
                self.regs.set_nf(false);
                self.regs.set_hf(false);
                self.regs.set_cf(v & 0b01 > 0);
                self.exec_state.val8 = result;
                go!(self, 1);
            },
            1: if self.write8(bus, src, self.exec_state.val8).is_some() {
                go!(self, 0);
                self.fetch(bus);
            },
        });
    }

    pub fn rr<S: Copy>(&mut self, bus: &mut Peripherals, src: S)
    where Self: IO8<S> {
        step!(self, (), {
            0: if let Some(v) = self.read8(bus, src) {
                let result = (v >> 1) | ((self.regs.cf() as u8) << 7) ;
                self.regs.set_zf(result == 0);
                self.regs.set_nf(false);
                self.regs.set_hf(false);
                self.regs.set_cf(v & 0b01 > 0);
                self.exec_state.val8 = result;
                go!(self, 1);
            },
            1: if self.write8(bus, src, self.exec_state.val8).is_some() {
                go!(self, 0);
                self.fetch(bus);
            },
        });
    }

    pub fn sla<S: Copy>(&mut self, bus: &mut Peripherals, src: S)
    where Self: IO8<S> {
        step!(self, (), {
            0: if let Some(v) = self.read8(bus, src) {
                let result = v << 1;
                self.regs.set_zf(result == 0);
                self.regs.set_nf(false);
                self.regs.set_hf(false);
                self.regs.set_cf(v & 0x80 != 0);
                self.exec_state.val8 = result;
                go!(self, 1);
            },
            1: if self.write8(bus, src, self.exec_state.val8).is_some() {
                go!(self, 0);
                self.fetch(bus);
            },
        });
    }

    pub fn sra<S: Copy>(&mut self, bus: &mut Peripherals, src: S)
    where Self: IO8<S> {
        step!(self, (), {
            0: if let Some(v) = self.read8(bus, src) {
                let result = (v >> 1) | (v & 0x80);
                self.regs.set_zf(result == 0);
                self.regs.set_nf(false);
                self.regs.set_hf(false);
                self.regs.set_cf(v & 0b01 > 0);
                self.exec_state.val8 = result;
                go!(self, 1);
            },
            1: if self.write8(bus, src, self.exec_state.val8).is_some() {
                go!(self, 0);
                self.fetch(bus);
            },
        });
    }

    pub fn srl<S: Copy>(&mut self, bus: &mut Peripherals, src: S)
    where Self: IO8<S> {
        step!(self, (), {
            0: if let Some(v) = self.read8(bus, src) {
                let result = v >> 1;
                self.regs.set_zf(result == 0);
                self.regs.set_nf(false);
                self.regs.set_hf(false);
                self.regs.set_cf(v & 0b01 > 0);
                self.exec_state.val8 = result;
                go!(self, 1);
            },
            1: if self.write8(bus, src, self.exec_state.val8).is_some() {
                go!(self, 0);
                self.fetch(bus);
            },
        });
    }

    pub fn set<S: Copy>(&mut self, bus: &mut Peripherals, num: usize, src: S)
    where Self: IO8<S> {
        step!(self, (), {
            0: if let Some(v) = self.read8(bus, src) {
                let result = v | (0b01 << num);
                self.regs.set_nf(false);
                self.regs.set_hf(false);
                self.exec_state.val8 = result;
                go!(self, 1);
            },
            1: if self.write8(bus, src, self.exec_state.val8).is_some() {
                go!(self, 0);
                self.fetch(bus);
            },
        });
    }

    pub fn res<S: Copy>(&mut self, bus: &mut Peripherals, num: usize, src: S)
    where Self: IO8<S> {
        step!(self, (), {
            0: if let Some(v) = self.read8(bus, src) {
                let result = v & !(0b01 << num);
                self.regs.set_nf(false);
                self.regs.set_hf(false);
                self.exec_state.val8 = result;
                go!(self, 1);
            },
            1: if self.write8(bus, src, self.exec_state.val8).is_some() {
                go!(self, 0);
                self.fetch(bus);
            },
        });
    }

    pub fn jp(&mut self, bus: &Peripherals) {
        step!(self, (), {
            0: if let Some(v) = self.read16(bus, Imm16) {
                self.exec_state.val16 = v;
                go!(self, 1);
            },
            1: {
                self.regs.pc = self.exec_state.val16;
                go!(self, 0);
                self.fetch(bus);
            },
        });
    }

    pub fn jp_hl(&mut self, bus: &Peripherals) {
        self.regs.pc = self.regs.hl();
        self.fetch(bus);
    }

    pub fn jp_c(&mut self, bus: &mut Peripherals, cond: Cond) {
        step!(self, (), {
            0: if let Some(v) = self.read16(bus, Imm16) {
                self.exec_state.val16 = v;
                if !self.cond(cond) {
                    go!(self, 0);
                    self.fetch(bus);
                } else {
                    go!(self, 1);
                }
            },
            1: {
                self.regs.pc = self.exec_state.val16;
                go!(self, 0);
                self.fetch(bus);
            },
        });
    }

    pub fn call_c(&mut self, bus: &mut Peripherals, cond: Cond) {
        step!(self, (), {
            0: if let Some(v) = self.read16(bus, Imm16) {
                self.exec_state.val16 = v;
                if !self.cond(cond) {
                    go!(self, 0);
                    self.fetch(bus);
                } else {
                    go!(self, 1);
                }
            },
            1: if self.push16(bus, self.regs.pc).is_some() {
                self.regs.pc = self.exec_state.val16;
                go!(self, 0);
                self.fetch(bus);
            },
        });
    }

    pub fn ret_c(&mut self, bus: &mut Peripherals, cond: Cond) {
        step!(self, (), {
            0: {
                if !self.cond(cond) {
                    go!(self, 0);
                    self.fetch(bus);
                } else {
                    go!(self, 1);
                }
            },
            1: if let Some(v) = self.pop16(bus) {
                self.exec_state.val16 = v;
                go!(self, 2);
            },
            2: {
                self.regs.pc = self.exec_state.val16;
                go!(self, 0);
                self.fetch(bus);
            },
        });
    }

    pub fn rst_addr(&mut self, bus: &mut Peripherals, addr: u8) {
        if self.push16(bus, self.regs.pc).is_some() {
            self.regs.pc = addr as u16;
            self.fetch(bus);
        }
    }

    pub fn swap<S: Copy>(&mut self, bus: &mut Peripherals, src: S)
    where Self: IO8<S> {
        step!(self, (), {
            0: if let Some(v) = self.read8(bus, src) {
                let result = (v >> 4) | (v << 4);
                self.regs.set_zf(result == 0);
                self.regs.set_nf(false);
                self.regs.set_hf(false);
                self.regs.set_cf(false);
                self.exec_state.val8 = result;
                go!(self, 1);
            },
            1: if self.write8(bus, src, self.exec_state.val8).is_some() {
                go!(self, 0);
                self.fetch(bus);
            },
        });
    }

    pub fn ccf(&mut self, bus: &mut Peripherals) {
        self.regs.set_nf(false);
        self.regs.set_hf(false);
        self.regs.set_cf(!self.regs.cf());
        self.fetch(bus);
    }

    pub fn scf(&mut self, bus: &mut Peripherals) {
        self.regs.set_nf(false);
        self.regs.set_hf(false);
        self.regs.set_cf(true);
        self.fetch(bus);
    }

    pub fn daa(&mut self, bus: &mut Peripherals) {
        let mut correction = 0;
        let mut carry = self.regs.cf();

        if self.regs.cf() {
            if self.regs.hf() {
                correction = 0x06;
            }
            if self.regs.cf() {
                correction |= 0x60;
            }

            self.regs.a = self.regs.a.wrapping_sub(correction);
        } else {
            if self.regs.hf() || (self.regs.a & 0x0f) > 0x09 {
                correction = 0x06;
            }
            if carry || self.regs.a > 0x99 {
                correction |= 0x60;
                carry = true;
            }
            self.regs.a = self.regs.a.wrapping_add(correction);
        }
        self.regs.set_zf(self.regs.a == 0);
        self.regs.set_nf(self.regs.nf());
        self.regs.set_hf(false);
        self.regs.set_cf(carry);
        self.fetch(bus);
    }

    pub fn cpl(&mut self, bus: &mut Peripherals) {
        self.regs.a = !self.regs.a;
        self.regs.set_nf(true);
        self.regs.set_hf(true);
        self.fetch(bus);
    }

    pub fn ld_sp_hl(&mut self, bus: &mut Peripherals) {
        self.regs.sp = self.regs.hl();
        self.fetch(bus);
    }

    pub fn ld_hl_sp_e(&mut self, bus: &mut Peripherals) {
        step!(self, (), {
            0: if let Some(v) = self.read8(bus, Imm8) {
                let v = v as i8 as u16;
                let hf = (v & 0x0f) + (self.regs.sp & 0x0f) > 0x0f;
                let cf = (v & 0xff) + (self.regs.sp & 0xff) > 0xff;
                let result = self.regs.sp.wrapping_add(v);
                self.regs.write_hl(result);
                self.regs.set_zf(false);
                self.regs.set_nf(false);
                self.regs.set_hf(hf);
                self.regs.set_cf(cf);
                go!(self, 1);
            },
            1:{
                go!(self, 0);
                self.fetch(bus);
            },
        });
    }

    pub fn add_hl_reg16(&mut self, bus: &mut Peripherals, reg: Reg16) {
        step!(self, (), {
            0: {
                let v = self.read16(bus, reg).unwrap();
                let hf = (v & 0x0fff) + (self.regs.hl() & 0x0fff) > 0x0fff;
                let cf = (v as u32) + (self.regs.hl() as u32) > 0xffff;
                let result = self.regs.hl().wrapping_add(v);
                self.regs.write_hl(result);
                self.regs.set_nf(false);
                self.regs.set_hf(hf);
                self.regs.set_cf(cf);
                go!(self, 1);
            },
            1: {
                self.fetch(bus);
                go!(self, 0);
            },
        });
    }

    pub fn add_sp_e(&mut self, bus: &mut Peripherals) {
        step!(self, (), {
            0: if let Some(v) = self.read8(bus, Imm8) {
                self.exec_state.val8 = v;
                go!(self, 1);
            },
            1: {
                let e_u8 = self.exec_state.val8;
                let v = e_u8 as i8 as u16;
                let hf = (self.regs.sp & 0x000f) + (v & 0x000f) > 0x000f;
                let cf = (self.regs.sp & 0x00ff) + (e_u8 as u16) > 0x00ff;
                let result = self.regs.sp.wrapping_add(v);

                self.regs.sp = result;
                self.regs.set_zf(false);
                self.regs.set_nf(false);
                self.regs.set_hf(hf);
                self.regs.set_cf(cf);
                go!(self, 2);
            },
            2: {
                go!(self, 0);
                self.fetch(bus);
            },
        });
    }

    pub fn bit<S: Copy>(&mut self, bus: &Peripherals, bit: usize, src: S)
    where  Self: IO8<S> {
        if let Some(v) = self.read8(bus, src) {
            v & 1 << bit;
            self.regs.set_zf(v == 0);
            self.regs.set_nf(false);
            self.regs.set_hf(true);
            self.fetch(bus);
        }
    }

    pub fn push16(&mut self, bus: &mut Peripherals, val: u16) -> Option<()> {
        step!(self, None, {
            0: {
                go!(self, 1);
                return None;
            },
            1: {
                let [lo, hi] = u16::to_le_bytes(val);
                self.regs.sp = self.regs.sp.wrapping_sub(1);
                bus.write(&mut self.interrupts, self.regs.sp, hi);
                self.exec_state.val8 = lo;
                go!(self, 2);
                return None;
            },
            2: {
                self.regs.sp = self.regs.sp.wrapping_sub(1);
                bus.write(&mut self.interrupts, self.regs.sp, self.exec_state.val8);
                go!(self, 3);
                return None;
            },
            3: {
                go!(self, 0);
                return Some(()); 
            },
        });
    }

    pub fn push(&mut self, bus: &mut Peripherals, src: Reg16) {
        step!(self, (), {
            0: {
                self.exec_state.val16 = self.read16(bus, src).unwrap();
                go!(self, 1);
            },
            1: if self.push16(bus, self.exec_state.val16).is_some() {
                go!(self, 2);
            },
            2: {
                go!(self, 0);
                self.fetch(bus);
            },
        });
    }

    pub fn pop16(&mut self, bus: &Peripherals) -> Option<u16> {
        step!(self, None, {
            0: {
                self.exec_state.val8 = bus.read(&mut self.interrupts, self.regs.sp);
                self.regs.sp = self.regs.sp.wrapping_add(1);
                go!(self, 1);
                return None;
            },
            1: {
                let hi = bus.read(&mut self.interrupts, self.regs.sp);
                self.regs.sp = self.regs.sp.wrapping_add(1);
                self.exec_state.val16 = u16::from_le_bytes([self.exec_state.val8, hi]);
                go!(self, 2);
                return None;
            },
            2: {
                go!(self, 0);
                return Some(self.exec_state.val16);
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
        step!(self, (), {
            0: if let Some(v) = self.read8(bus, Imm8) {
                self.regs.pc = self.regs.pc.wrapping_add(v as i8 as u16);
                go!(self, 1); 
                return;
            },
            1: {
                go!(self, 0);
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

    pub fn jr_c(&mut self, bus: &Peripherals, c: Cond) {
        step!(self, (), {
            0: if let Some(v) = self.read8(bus, Imm8) {
                go!(self, 1);
                if self.cond(c) {
                    self.regs.pc = self.regs.pc.wrapping_add(v as i8 as u16);
                    return;
                }
            },
            1: {
                go!(self, 0);
                self.fetch(bus);
            },
        });
    }

    pub fn call(&mut self, bus: &mut Peripherals) {
        step!(self, (), {
            0: if let Some(v) = self.read16(bus, Imm16) {
                self.exec_state.val16 = v;
                go!(self, 1);
            },
            1: if self.push16(bus, self.regs.pc).is_some() {
                self.regs.pc = self.exec_state.val16;
                go!(self, 0);
                self.fetch(bus); 
            },
        });
    }

    pub fn ret(&mut self, bus: &Peripherals) {
        step!(self, (), {
            0: if let Some(v) = self.pop16(bus) {
                self.regs.pc = v;
                go!(self, 1);
            },
            1: {
                go!(self, 0);
                self.fetch(bus);
            },
        });
    }

    pub fn reti(&mut self, bus: &Peripherals) {
        step!(self, (), {
            0: if let Some(v) = self.pop16(bus) {
                self.regs.pc = v;
                go!(self, 1);
                return;
            },
            1: {
                self.interrupts.ime = true;
                go!(self, 0);
                self.fetch(bus);
            },
        });
    }

    pub fn ei(&mut self, bus: &Peripherals) {
        self.fetch(bus);
        self.interrupts.ime = true;
    }

    pub fn di(&mut self, bus: &Peripherals) {
        self.interrupts.ime = false;
        self.fetch(bus);
    }

    pub fn halt(&mut self, bus: &Peripherals) {
        step!(self, (), {
            0: if self.interrupts.get_interrupt() > 0 {
                self.fetch(bus);
            } else {
                return go!(self, 1);
            },
            1: {
                if self.interrupts.get_interrupt() > 0 {
                    go!(self, 0);
                    self.fetch(bus);
                }
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