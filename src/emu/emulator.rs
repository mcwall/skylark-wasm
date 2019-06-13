use std::fmt;
use std::collections::LinkedList;

// TODO: Maybe usize?
const WIDTH: u32 = 64;
const HEIGHT: u32 = 32;
const PRG_OFFSET: usize = 0x200;
const RAM_SIZE: usize = 0x1000;
const REG_SIZE: usize = 0xf;

pub struct Cpu {
    pc: usize,
    i: u8,
    v: Vec<u8>,
    stack: LinkedList<u8>
}

impl Cpu {
    pub fn new() -> Cpu {
        let pc = PRG_OFFSET;
        let i = 0;
        let v = vec![0; REG_SIZE];
        let stack = LinkedList::new();

        Cpu {
            pc,
            i,
            v,
            stack
        }
    }

    // TODO: Ram and display should probably be borrowed by Cpu struct, not just this function
    pub fn tick(&mut self, ram: &mut Vec<u8>, display: &mut DisplayFrame) {
        // Decompose opcode into 4 nibbles
        let opcode: u16 = (ram[self.pc] as u16) << 8 | (ram[self.pc + 1] as u16);
        let a = opcode >> 12;
        let b = opcode >> 8 & 0xf;
        let c = opcode >> 4 & 0xf;
        let d = opcode & 0xf;

        match (a, b, c, d) {

            // disp_clear
            (0x0, 0x0, 0xE, 0x0) => {
                display.clear();
            }

            // return
            (0x0, 0x0, 0xE, 0xE) => {
                panic!("Unsupported opcode: 0x{:x?}", opcode)
            }

            // call RCA (not implemented)
            (0x0, _, _, _) => {
                panic!("Unsupported opcode (RCA 1802): 0x{:x?}", opcode)
            }

            // goto $xyz
            (0x1, x, y, z) => {
                panic!("Unsupported opcode: 0x{:x?}", opcode)
            }

            // call $n
            (0x2, n1, n2, n3) => {
                panic!("Unsupported opcode: 0x{:x?}", opcode)
            }

            // v[x] == n
            (0x3, x, n1, n2) => {
                let n = (n1 << 4 | n2) as u8;
                if self.v[x as usize] == n {
                    self.pc += 2;
                }
            }

            // v[x] != n
            (0x4, x, n1, n2) => {
                let n = (n1 << 4 | n2) as u8;
                if self.v[x as usize] != n {
                    self.pc += 2;
                }
            }

            // v[x] == v[y]
            (0x5, x, y, 0x0) => {
                if self.v[x as usize] == self.v[y as usize] {
                    self.pc += 2;
                }
            }

            // v[x] = n
            (0x6, x, n1, n2) => {
                let n = (n1 << 4 | n2) as u8;
                self.v[x as usize] = n;
            }

            // v[x] += n
            (0x7, x, n1, n2) => {
                let n = (n1 << 4 | n2) as u8;
                self.v[x as usize] += n;
            }

            // v[x] = v[y]
            (0x8, x, y, 0x0) => {
                self.v[x as usize] = self.v[y as usize];
            }

            // v[x] |= v[y]
            (0x8, x, y, 0x1) => {
                self.v[x as usize] |= self.v[y as usize];
            }

            // v[x] &= v[y]
            (0x8, x, y, 0x2) => {
                self.v[x as usize] &= self.v[y as usize];
            }

            // v[x] ^= v[y]
            (0x8, x, y, 0x3) => {
                self.v[x as usize] ^= self.v[y as usize];
            }

            // v[x] += v[y]
            (0x8, x, y, 0x4) => {
                let res = self.v[x as usize] as u16 + self.v[y as usize] as u16;
                self.v[x as usize] = res as u8;
                self.v[0xf] = if res > 0xff { 1 } else { 0 }; // cary flag
            }

            // v[x] -= v[y]
            (0x8, x, y, 0x5) => {
                let res = self.v[x as usize] as i16 - self.v[y as usize] as i16;
                self.v[x as usize] = res as u8;
                self.v[0xf] = if res >= 0 { 1 } else { 0 }; // inverse borrow flag
            }

            // v[x] >>= 1
            (0x8, x, _, 0x6) => {
                self.v[0xf] = self.v[x as usize] & 1; // store LSB of v[x] in v[0xf]
                self.v[x as usize] >>= 1;
            }

            // v[x] = v[y] - v[x]
            (0x8, x, y, 0x7) => {
                let res = self.v[y as usize] as i16 - self.v[x as usize] as i16;
                self.v[x as usize] = res as u8;
                self.v[0xf] = if res >= 0 { 1 } else { 0 }; // inverse borrow flag
            }

            // v[x] <<= 1
            (0x8, x, _, 0xE) => {
                self.v[0xf] = self.v[x as usize] >> 7 & 1; // store MSB of v[x] in v[0xf]
                self.v[x as usize] <<= 1;
            }

            // v[x] != v[y]
            (0x9, x, y, 0x0) => {
                if self.v[x as usize] != self.v[y as usize] {
                    self.pc += 2;
                }
            }

            _ => {
                println!("Unsupported opcode: 0x{:x?}", opcode)
            }
        };

        self.pc += 2;

        for n in 0..10{
            display.set(n, n, !display.get(n, n));
        }
    }
}

pub struct DisplayFrame {
    pixels: Vec<bool>
}

impl DisplayFrame {
    pub fn new() -> DisplayFrame {
        let pixels = vec![false; (WIDTH * HEIGHT) as usize];

        DisplayFrame {
            pixels
        }
    }

    fn get_index(&self, x: u32, y: u32) -> usize {
        (x * WIDTH + y) as usize
    }

    pub fn get(&self, x: u32, y: u32) -> bool {
        let idx = self.get_index(x, y);
        self.pixels[idx]
    }

    pub fn set(&mut self, x: u32, y: u32, val: bool) {
        let idx = self.get_index(x, y);
        self.pixels[idx] = val;
    }

    pub fn clear(&mut self) {
        for idx in 0..self.pixels.len() {
            self.pixels[idx] = false;
        }
    }
}

impl fmt::Display for DisplayFrame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.pixels.as_slice().chunks(WIDTH as usize) {
            for &pixel in row {
                let symbol = if pixel { '◼' } else { '◻' };
                write!(f, "{}", symbol)?;
            }

            write!(f, "\n")?;
        }

        Ok(())
    }
}

pub struct Emulator {
    ram: Vec<u8>,
    cpu: Cpu,
    display: DisplayFrame
}

impl Emulator {
    pub fn new() -> Emulator {
        let ram = vec![0; RAM_SIZE];
        let cpu = Cpu::new();
        let display = DisplayFrame::new();

        Emulator {
            ram,
            cpu,
            display
        }
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        self.load_fonts();

        for (i, rom_byte) in rom.iter().enumerate() {
            self.ram[PRG_OFFSET + i] = *rom_byte;
        }
    }

    pub fn display_out(&self) -> String {
        self.display.to_string()
    }

    pub fn tick(&mut self) {
        self.cpu.tick(&mut self.ram, &mut self.display);
    }

    fn load_fonts(&mut self) {
        // TODO: move this data to a proper data file

        // 0
        let mut addr = 0;
        self.ram[addr + 0] = 0xf; // 1111
        self.ram[addr + 1] = 0x9; // 1001
        self.ram[addr + 2] = 0x9; // 1001
        self.ram[addr + 3] = 0x9; // 1001
        self.ram[addr + 4] = 0xf; // 1111

        // 1
        addr += 5;
        self.ram[addr + 0] = 0x2; // 0010
        self.ram[addr + 1] = 0x6; // 0110
        self.ram[addr + 2] = 0x2; // 0010
        self.ram[addr + 3] = 0x2; // 0010
        self.ram[addr + 4] = 0x7; // 0111

        // 2
        addr += 5;
        self.ram[addr + 0] = 0xf; // 1111
        self.ram[addr + 1] = 0x1; // 0001
        self.ram[addr + 2] = 0xf; // 1111
        self.ram[addr + 3] = 0x8; // 1000
        self.ram[addr + 4] = 0xf; // 1111

        // 3
        addr += 5;
        self.ram[addr + 0] = 0xf; // 1111
        self.ram[addr + 1] = 0x1; // 0001
        self.ram[addr + 2] = 0xf; // 1111
        self.ram[addr + 3] = 0x1; // 0001
        self.ram[addr + 4] = 0xf; // 1111

        // 4
        addr += 5;
        self.ram[addr + 0] = 0x9; // 1001
        self.ram[addr + 1] = 0x9; // 1001
        self.ram[addr + 2] = 0xf; // 1111
        self.ram[addr + 3] = 0x1; // 0001
        self.ram[addr + 4] = 0x1; // 0001

        // 5
        addr += 5;
        self.ram[addr + 0] = 0xf; // 1111
        self.ram[addr + 1] = 0x8; // 1000
        self.ram[addr + 2] = 0xf; // 1111
        self.ram[addr + 3] = 0x1; // 0001
        self.ram[addr + 4] = 0xf; // 1111

        // 6
        addr += 5;
        self.ram[addr + 0] = 0xf; // 1111
        self.ram[addr + 1] = 0x8; // 1000
        self.ram[addr + 2] = 0xf; // 1111
        self.ram[addr + 3] = 0x9; // 1001
        self.ram[addr + 4] = 0xf; // 1111

        // 7
        addr += 5;
        self.ram[addr + 0] = 0xf; // 1111
        self.ram[addr + 1] = 0x1; // 0001
        self.ram[addr + 2] = 0x2; // 0010
        self.ram[addr + 3] = 0x4; // 0100
        self.ram[addr + 4] = 0x4; // 0100

        // 8
        addr += 5;
        self.ram[addr + 0] = 0xf; // 1111
        self.ram[addr + 1] = 0x9; // 1001
        self.ram[addr + 2] = 0xf; // 1111
        self.ram[addr + 3] = 0x9; // 1001
        self.ram[addr + 4] = 0xf; // 1111

        // 9
        addr += 5;
        self.ram[addr + 0] = 0xf; // 1111
        self.ram[addr + 1] = 0x9; // 1001
        self.ram[addr + 2] = 0xf; // 1111
        self.ram[addr + 3] = 0x1; // 0001
        self.ram[addr + 4] = 0xf; // 1111

        // A
        addr += 5;
        self.ram[addr + 0] = 0xf; // 1111
        self.ram[addr + 1] = 0x9; // 1001
        self.ram[addr + 2] = 0xf; // 1111
        self.ram[addr + 3] = 0x9; // 1001
        self.ram[addr + 4] = 0x9; // 1001

        // B
        addr += 5;
        self.ram[addr + 0] = 0xe; // 1110
        self.ram[addr + 1] = 0x9; // 1001
        self.ram[addr + 2] = 0xe; // 1110
        self.ram[addr + 3] = 0x9; // 1001
        self.ram[addr + 4] = 0xe; // 1110

        // C
        addr += 5;
        self.ram[addr + 0] = 0xf; // 1111
        self.ram[addr + 1] = 0x8; // 1000
        self.ram[addr + 2] = 0x8; // 1000
        self.ram[addr + 3] = 0x8; // 1000
        self.ram[addr + 4] = 0xf; // 1111

        // D
        addr += 5;
        self.ram[addr + 0] = 0xe; // 1110
        self.ram[addr + 1] = 0x9; // 1001
        self.ram[addr + 2] = 0x9; // 1001
        self.ram[addr + 3] = 0x9; // 1001
        self.ram[addr + 4] = 0xe; // 1110

        // E
        addr += 5;
        self.ram[addr + 0] = 0xf; // 1111
        self.ram[addr + 1] = 0x8; // 1000
        self.ram[addr + 2] = 0xf; // 1111
        self.ram[addr + 3] = 0x8; // 1000
        self.ram[addr + 4] = 0xf; // 1111

        // F
        addr += 5;
        self.ram[addr + 0] = 0xf; // 1111
        self.ram[addr + 1] = 0x8; // 1000
        self.ram[addr + 2] = 0xf; // 1111
        self.ram[addr + 3] = 0x8; // 1000
        self.ram[addr + 4] = 0x8; // 1000
    }
}
