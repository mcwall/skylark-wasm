use std::collections::LinkedList;
use rand::Rng;
use super::emulator;
use super::display;

pub struct Cpu {
    pc: usize,
    i: u16,
    v: Vec<u8>,
    stack: LinkedList<usize>
}

impl Cpu {
    pub fn new() -> Cpu {
        let pc = emulator::PRG_OFFSET;
        let i = 0;
        let v = vec![0; emulator::REG_SIZE];
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
    pub fn tick(&mut self, ram: &mut Vec<u8>, display: &mut display::DisplayFrame) {
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
                self.i = (emulator::FONT_OFFSET + (emulator::FONT_WIDTH * self.v[x as usize] as usize)) as u16;
            }

            _ => {
                println!("Unsupported opcode: 0x{:x?}", opcode)
            }
        };

        self.pc += 2;
    }
}
