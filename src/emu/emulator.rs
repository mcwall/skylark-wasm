use std::fmt;
use std::collections::LinkedList;
use rand::Rng;

// TODO: Maybe usize?
const WIDTH: u32 = 64;
const HEIGHT: u32 = 32;
const PRG_OFFSET: usize = 0x200;
const RAM_SIZE: usize = 0x1000;
const REG_SIZE: usize = 0x10;
const FONT_OFFSET: usize = 0x0;
const FONT_WIDTH: usize = 5;

pub struct Cpu {
    pc: usize,
    i: u16,
    v: Vec<u8>,
    stack: LinkedList<usize>
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

    // TODO: Only re-render when display changes
    // TODO: Ram and display should probably be borrowed by Cpu struct, not just this function
    pub fn tick(&mut self, ram: &mut Vec<u8>, display: &mut DisplayFrame) {
        // Decompose opcode into 4 nibbles
        let opcode: u16 = (ram[self.pc] as u16) << 8 | (ram[self.pc + 1] as u16);
        let a = opcode >> 12;
        let b = opcode >> 8 & 0xf;
        let c = opcode >> 4 & 0xf;
        let d = opcode & 0xf;

        // println!("Executing: 0x{:x?}", opcode);

        match (a, b, c, d) {

            // disp_clear
            (0x0, 0x0, 0xE, 0x0) => {
                display.clear();
            }

            // return
            (0x0, 0x0, 0xE, 0xE) => {
                self.pc = self.stack.pop_front().expect("Stack underflow on return");
            }

            // call RCA (not implemented)
            (0x0, _, _, _) => {
                panic!("Unsupported opcode (RCA 1802): 0x{:x?}", opcode);
            }

            // goto $n
            (0x1, n1, n2, n3) => {
                let n = (n1 << 8 | n2 << 4 | n3) as usize;
                self.pc = n - 2;
            }

            // call $n
            (0x2, n1, n2, n3) => {
                let n = (n1 << 8 | n2 << 4 | n3) as usize;
                self.stack.push_front(self.pc);
                self.pc = n - 2;
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
                let n: u16 = n1 << 4 | n2;
                let res = self.v[x as usize] as u16 + n;
                self.v[x as usize] = (res % 0xFF) as u8;
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

            // i = n
            (0xA, n1, n2, n3) => {
                let n = (n1 << 8 | n2 << 4 | n3) as u16;
                self.i = n;
            }

            // jmp (v[0] + n)
            (0xB, n1, n2, n3) => {
                let n = (n1 << 8 | n2 << 4 | n3) as u16;
                self.pc += (self.v[0] as u16 + n - 2) as usize; // -2 to offset the increment below
            }

            // v[x] = rand() & n
            (0xC, x, n1, n2) => {
                let n = (n1 << 4 | n2) as u8;
                self.v[x as usize] = n & rand::thread_rng().gen_range(0, 0xFF);
            }

            // draw(v[x], v[y], n)
            (0xD, x, y, n) => {
                let sprite_start = self.i as usize;
                let sprite_end = sprite_start + n as usize;
                let sprite = &ram[sprite_start .. sprite_end];
                
                let pixel_flip = display.draw(self.v[x as usize], self.v[y as usize], &sprite);
                self.v[0xF] =  if pixel_flip { 1 } else { 0 };
            }

            // i = sprite[v[x]]
            (0xF, x, 0x2, 0x9) => {
                self.i = (FONT_OFFSET + (FONT_WIDTH * self.v[x as usize] as usize)) as u16;
            }

            _ => {
                println!("Unsupported opcode: 0x{:x?}", opcode)
            }
        };

        self.pc += 2;
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
        ((x % WIDTH) + (y % HEIGHT) * WIDTH) as usize
    }

    pub fn clear(&mut self) {
        for idx in 0..self.pixels.len() {
            self.pixels[idx] = false;
        }
    }

    pub fn draw(&mut self, x: u8, y: u8, sprite: &[u8]) -> bool {
        let mut change = false;
        // println!("Drawing ({}, {}) h{}", x, y, sprite.len());
        let test: Vec<usize> = sprite.iter().map(|s| { *s as usize }).collect();
        for dy in 0..sprite.len() {
            change |= self.draw_sprite(x, y + dy as u8, sprite[dy as usize]);
        }

        change
    }

    // TODO: This can be improved by simply XORing the sprite with existing byte from ram
    fn draw_sprite(&mut self, x: u8, y: u8, sprite: u8) -> bool {
        let mut change = false;
        for i in 0 .. 8 {
            let index = self.get_index(x as u32 + i as u32, y as u32);
            let new_pixel = match sprite >> (7 - i) & 1 {
                0 => false,
                _ => true
            } ^ self.pixels[index];

            change |= self.pixels[index] && !new_pixel;
            self.pixels[index] = new_pixel;
        }

        change
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
        self.ram[addr + 0] = 0xf0; // 1111
        self.ram[addr + 1] = 0x90; // 1001
        self.ram[addr + 2] = 0x90; // 1001
        self.ram[addr + 3] = 0x90; // 1001
        self.ram[addr + 4] = 0xf0; // 1111

        // 1
        addr += 5;
        self.ram[addr + 0] = 0x20; // 0010
        self.ram[addr + 1] = 0x60; // 0110
        self.ram[addr + 2] = 0x20; // 0010
        self.ram[addr + 3] = 0x20; // 0010
        self.ram[addr + 4] = 0x70; // 0111

        // 2
        addr += 5;
        self.ram[addr + 0] = 0xf0; // 1111
        self.ram[addr + 1] = 0x10; // 0001
        self.ram[addr + 2] = 0xf0; // 1111
        self.ram[addr + 3] = 0x80; // 1000
        self.ram[addr + 4] = 0xf0; // 1111

        // 3
        addr += 5;
        self.ram[addr + 0] = 0xf0; // 1111
        self.ram[addr + 1] = 0x10; // 0001
        self.ram[addr + 2] = 0xf0; // 1111
        self.ram[addr + 3] = 0x10; // 0001
        self.ram[addr + 4] = 0xf0; // 1111

        // 4
        addr += 5;
        self.ram[addr + 0] = 0x90; // 1001
        self.ram[addr + 1] = 0x90; // 1001
        self.ram[addr + 2] = 0xf0; // 1111
        self.ram[addr + 3] = 0x10; // 0001
        self.ram[addr + 4] = 0x10; // 0001

        // 5
        addr += 5;
        self.ram[addr + 0] = 0xf0; // 1111
        self.ram[addr + 1] = 0x80; // 1000
        self.ram[addr + 2] = 0xf0; // 1111
        self.ram[addr + 3] = 0x10; // 0001
        self.ram[addr + 4] = 0xf0; // 1111

        // 6
        addr += 5;
        self.ram[addr + 0] = 0xf0; // 1111
        self.ram[addr + 1] = 0x80; // 1000
        self.ram[addr + 2] = 0xf0; // 1111
        self.ram[addr + 3] = 0x90; // 1001
        self.ram[addr + 4] = 0xf0; // 1111

        // 7
        addr += 5;
        self.ram[addr + 0] = 0xf0; // 1111
        self.ram[addr + 1] = 0x10; // 0001
        self.ram[addr + 2] = 0x20; // 0010
        self.ram[addr + 3] = 0x40; // 0100
        self.ram[addr + 4] = 0x40; // 0100

        // 8
        addr += 5;
        self.ram[addr + 0] = 0xf0; // 1111
        self.ram[addr + 1] = 0x90; // 1001
        self.ram[addr + 2] = 0xf0; // 1111
        self.ram[addr + 3] = 0x90; // 1001
        self.ram[addr + 4] = 0xf0; // 1111

        // 9
        addr += 5;
        self.ram[addr + 0] = 0xf0; // 1111
        self.ram[addr + 1] = 0x90; // 1001
        self.ram[addr + 2] = 0xf0; // 1111
        self.ram[addr + 3] = 0x10; // 0001
        self.ram[addr + 4] = 0xf0; // 1111

        // A
        addr += 5;
        self.ram[addr + 0] = 0xf0; // 1111
        self.ram[addr + 1] = 0x90; // 1001
        self.ram[addr + 2] = 0xf0; // 1111
        self.ram[addr + 3] = 0x90; // 1001
        self.ram[addr + 4] = 0x90; // 1001

        // B
        addr += 5;
        self.ram[addr + 0] = 0xe0; // 1110
        self.ram[addr + 1] = 0x90; // 1001
        self.ram[addr + 2] = 0xe0; // 1110
        self.ram[addr + 3] = 0x90; // 1001
        self.ram[addr + 4] = 0xe0; // 1110

        // C
        addr += 5;
        self.ram[addr + 0] = 0xf0; // 1111
        self.ram[addr + 1] = 0x80; // 1000
        self.ram[addr + 2] = 0x80; // 1000
        self.ram[addr + 3] = 0x80; // 1000
        self.ram[addr + 4] = 0xf0; // 1111

        // D
        addr += 5;
        self.ram[addr + 0] = 0xe0; // 1110
        self.ram[addr + 1] = 0x90; // 1001
        self.ram[addr + 2] = 0x90; // 1001
        self.ram[addr + 3] = 0x90; // 1001
        self.ram[addr + 4] = 0xe0; // 1110

        // E
        addr += 5;
        self.ram[addr + 0] = 0xf0; // 1111
        self.ram[addr + 1] = 0x80; // 1000
        self.ram[addr + 2] = 0xf0; // 1111
        self.ram[addr + 3] = 0x80; // 1000
        self.ram[addr + 4] = 0xf0; // 1111

        // F
        addr += 5;
        self.ram[addr + 0] = 0xf0; // 1111
        self.ram[addr + 1] = 0x80; // 1000
        self.ram[addr + 2] = 0xf0; // 1111
        self.ram[addr + 3] = 0x80; // 1000
        self.ram[addr + 4] = 0x80; // 1000
    }
}
