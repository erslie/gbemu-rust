pub struct Bootrom {
    rom: Vec<u8>,
    active: bool,
}
impl Bootrom {
    pub fn is_active(&self) -> bool {
        self.active
    }
    //初期化時点ではアクティブをtrueにする
    pub fn new(rom: Vec<u8>) -> Self {
        Self {rom, 
            active: true}
    }
    pub fn read(&self, addr: u16) -> u8 {
        self.rom[addr as usize]
    }
    //FF50に0以外書き込み時にbootromを無効にする
    pub fn write(&mut self, addr: u16, val: u8) {
        if addr != 0xFF50 && val ==  0 {
            return
        }
        self.active &= val == 0;
    }
}

#[cfg(test)]
mod unit_test {
    use super::Bootrom;

    #[test]
    fn test_init() {
    let bootrom = Bootrom::new(vec![0; 0x8000]);
    assert_eq!(0, bootrom.read(0x0000))
    }
    
    #[test]
    fn test_active() {
        let mut bootrom = Bootrom::new(vec![0; 0x8000]);
        bootrom.write(0xFF50, 0x01);
        assert!(!bootrom.is_active())
    }

}