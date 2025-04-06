use std::vec;
//WRam メインメモリ
pub struct WRam(Vec<u8>);
//8KiB
impl WRam {
    pub fn new() -> Self {
        Self(vec![0; 0x8000])
    }
    pub fn read(&self, addr: u16) -> u8 {
        self.0[(addr as usize) & 0x1FFF]
    }
    pub fn write(&mut self, addr: u16, val: u8) {
        self.0[(addr as usize) & 0x1FFF] = val;
    }
}

#[cfg(test)]
mod unit_test {
    use super::WRam;
    
    #[test]
    fn test_readwrite() {
    let mut wram = WRam::new();
    wram.write(0x00, 0x42);
    assert_eq!(0x42, wram.read(0x00));
    println!("{:?}",wram.read(0));
    }

}