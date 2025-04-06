pub struct Bootrom {
    rom: vec![u8],//vecマクロ使う必要ないかも
}
impl Bootrom {
    pub fn new(rom: vec![u8]) -> Self {
        Self{rom}
    }
    pub fn read(&self, addr: u16) -> u8 {
        self.rom[addr as usize]
    }
}
