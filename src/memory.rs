const MEM_SIZE: usize = 0x1000;
const FONT_ADDR: usize = 0;
const ROM_ADDR: usize = 0x200;

#[cfg_attr(rustfmt, rustfmt_skip)]
const FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

pub struct Memory {
    raw: [u8; MEM_SIZE],
}

impl Memory {
    pub fn new() -> Memory {
        let mut raw = [0; MEM_SIZE];
        raw[FONT_ADDR..FONT_ADDR + FONTSET.len()].copy_from_slice(&FONTSET);

        Memory { raw }
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        self.raw[ROM_ADDR..ROM_ADDR + rom.len()].copy_from_slice(&rom);
    }

    pub fn read(&self, address: u16) -> u8 {
        self.raw[address as usize]
    }

    pub fn read_word(&self, address: u16) -> u16 {
        (self.raw[address as usize] as u16) << 8 | self.raw[(address + 1) as usize] as u16
    }

    pub fn read_range(&self, start: u16, end: u16) -> &[u8] {
        &self.raw[start as usize..end as usize]
    }

    pub fn write(&mut self, address: u16, val: u8) {
        self.raw[address as usize] = val;
    }

    pub fn get_char_addr(&self, index: u16) -> u16 {
        FONT_ADDR as u16 + (index * 5)
    }

    pub fn print_mem(&self) {
        for (i, val) in self.raw.iter().enumerate() {
            if i % 16 == 0 {
                print!("{:>03X}: ", i);
            }
            print!("{:>02X} ", val);
            if (i + 1) % 16 == 0 {
                println!();
            }
        }
    }
}
