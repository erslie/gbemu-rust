

#[derive(Clone, Copy, Default)]
pub struct Registres { 
    pub pc: u16,
    pub sp: u16,
    pub a:  u8,
    pub b:  u8,
    pub c:  u8,
    pub d:  u8,
    pub e:  u8,
    pub f:  u8,
    pub h:  u8,
    pub l:  u8,
}
impl Registres {
    pub fn af(&self) -> u16 {
        ((self.a as u16) << 8 ) | (self.f as u16)
    }

    pub fn bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    pub fn de(&self) -> u16 {
        ((self.d as u16) << 8)  | (self.e as u16)
    }

    pub fn hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    pub fn write_af(&mut self, val: u16) {
        self.a = (val >> 8) as u8;//下位マスクしたら明示的だけど参考文献では省いているので
        self.f = (val & 0xF0) as u8;//フラグレジスタは下位8bitは使用しない
    }

    pub fn write_bc(&mut self, val: u16) {
        self.b = (val >> 8) as u8;
        self.c = val as u8;
    }

    pub fn write_de(&mut self, val: u16) {
        self.d = (val >> 8) as u8;
        self.e = val as u8;
    }

    pub fn write_hl(&mut self, val: u16) {
        self.h = (val >> 8) as u8;
        self.l = val as u8;
    }

    pub fn zf(&self) -> bool {
        //Z80もそうだがフラグレジスタの最上位7bit目がゼロフラグのため
        //ANDでゼロフラグのみ判別
        (self.f & 0b_1000_0000) > 0
    }

    pub fn nf(&self) -> bool {
        //NはDAAに使用するフラグらしい
        (self.f & 0b_0100_0000) > 0
    }

    pub fn hf(&self) -> bool {
        //half carry (多分下位4bitで使うキャリーフラグ)
        (self.f & 0b_0010_0000) > 0
    }

    pub fn cf(&self) -> bool {
        //carry flag
        (self.f & 0b_0001_0000) > 0
    }

    pub fn set_zf(&mut self, zf: bool) {
        if zf {
            self.f |= 0b_1000_0000;
        } else {
            self.f &= 0b_0111_1111;
        }
    }

    pub fn set_nf(&mut self, nf: bool) {
        if nf {
            self.f |= 0b_0100_0000;
        } else {
            self.f &= 0b_1011_1111;
        }
    }

    pub fn set_hf(&mut self, hf: bool) {
        if hf {
            self.f |= 0b_0010_0000;
        } else {
            self.f &= 0b_1101_1111;
        }
    }

    pub fn set_cf(&mut self, cf: bool) {
        if cf {
            self.f |= 0b_0001_0000;
        } else {
            self.f &= 0b_1110_1111;
        }
    }


}


#[cfg(test)]
mod unit_test {

    //インライン展開でレジスタアクセスどれぐらい軽くなるかテストしたい

    use super::Registres;
    #[test]
    fn test_readwrite_af() {
        let mut reg = Registres::default();
        reg.write_af(0xFF00);
        assert_eq!(0xFF00,reg.af());
    }

    #[test]
    fn test_readwrite_bc() {
        let mut reg = Registres::default();
        reg.write_bc(0xAFFA);
        assert_eq!(0xAFFA,reg.bc()); 
    }

    #[test]
    fn test_readwrite_zf() {
        let mut reg = Registres::default();
        reg.set_zf(true);
        assert!(reg.zf());
    }
}