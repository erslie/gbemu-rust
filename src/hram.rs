use std::vec;
//High Ram ?スタック用の領域らしい　開始FF00,終了FFFF
pub struct HRam(Vec<u8>);
//128Byte
impl HRam {
    pub fn new() -> Self {
        Self(vec![0; 0x80])
    }
    pub fn read(&self, addr: u16) -> u8 {
        self.0[(addr as usize) & 0x7f]
    }
    pub fn write(&mut self, addr: u16, val: u8) {
        self.0[(addr as usize) & 0x7f] = val;
    }

}

#[cfg(test)]
mod unit_test {
    use super::HRam;

    #[test]
    fn test_readwrite() {
    let mut hram = HRam::new();
    hram.write(0x00, 0x42);
    assert_eq!(0x42, hram.read(0x00));
    println!("{:?}",hram.read(0));
    }

}