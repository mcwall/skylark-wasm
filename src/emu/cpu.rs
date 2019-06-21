use std::collections::LinkedList;
use rand::Rng;
use super::{ display, emulator, keyboard, timer };

extern crate web_sys;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

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
    pub fn tick(&mut self, ram: &mut Vec<u8>, keyboard: &keyboard::Keyboard, display: &mut display::DisplayFrame, timer: &mut timer::Timer, elapsed_millis: u32) {
        // Decompose opcode into 4 nibbles
        let opcode: u16 = (ram[self.pc] as u16) << 8 | (ram[self.pc + 1] as u16);
        // let opcode: usize = ((ram[self.pc] as u16) << 8 | (ram[self.pc + 1] as u16)) as usize;
        let a = opcode >> 12;
        let b = opcode >> 8 & 0xf;
        let c = opcode >> 4 & 0xf;
        let d = opcode & 0xf;

        println!("Executing: 0x{:x?}", opcode);

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

            // goto addr
            (0x1, n1, n2, n3) => {
                let n = (n1 << 8 | n2 << 4 | n3) as usize;
                self.pc = n - 2;
            }

            // call addr
            (0x2, n1, n2, n3) => {
                let n = (n1 << 8 | n2 << 4 | n3) as usize;
                self.stack.push_front(self.pc);
                self.pc = n - 2;
            }

            // Vx == N
            (0x3, x, n1, n2) => {
                let n = (n1 << 4 | n2) as u8;
                if self.v[x as usize] == n {
                    self.pc += 2;
                }
            }

            // Vx != N
            (0x4, x, n1, n2) => {
                let n = (n1 << 4 | n2) as u8;
                if self.v[x as usize] != n {
                    self.pc += 2;
                }
            }

            // Vx == Vy
            (0x5, x, y, 0x0) => {
                if self.v[x as usize] == self.v[y as usize] {
                    self.pc += 2;
                }
            }

            // Vx = N
            (0x6, x, n1, n2) => {
                let n = (n1 << 4 | n2) as u8;
                self.v[x as usize] = n;
            }

            // Vx += N
            (0x7, x, n1, n2) => {
                let n: u16 = n1 << 4 | n2;
                let res = self.v[x as usize] as u16 + n;
                self.v[x as usize] = (res % 0xFF) as u8;
            }

            // Vx = Vy
            (0x8, x, y, 0x0) => {
                self.v[x as usize] = self.v[y as usize];
            }

            // Vx |= Vy
            (0x8, x, y, 0x1) => {
                self.v[x as usize] |= self.v[y as usize];
            }

            // Vx &= Vy
            (0x8, x, y, 0x2) => {
                self.v[x as usize] &= self.v[y as usize];
            }

            // Vx ^= Vy
            (0x8, x, y, 0x3) => {
                self.v[x as usize] ^= self.v[y as usize];
            }

            // Vx += Vy
            (0x8, x, y, 0x4) => {
                let res = self.v[x as usize] as u16 + self.v[y as usize] as u16;
                self.v[x as usize] = res as u8;
                self.v[0xf] = if res > 0xff { 1 } else { 0 }; // cary flag
            }

            // Vx -= Vy
            (0x8, x, y, 0x5) => {
                let res = self.v[x as usize] as i16 - self.v[y as usize] as i16;
                self.v[x as usize] = res as u8;
                self.v[0xf] = if res >= 0 { 1 } else { 0 }; // inverse borrow flag
            }

            // Vx >>= 1
            (0x8, x, _, 0x6) => {
                self.v[0xf] = self.v[x as usize] & 1; // store LSB of Vx in v[0xf]
                self.v[x as usize] >>= 1;
            }

            // Vx = Vy - Vx
            (0x8, x, y, 0x7) => {
                let res = self.v[y as usize] as i16 - self.v[x as usize] as i16;
                self.v[x as usize] = res as u8;
                self.v[0xf] = if res >= 0 { 1 } else { 0 }; // inverse borrow flag
            }

            // Vx <<= 1
            (0x8, x, _, 0xE) => {
                self.v[0xf] = self.v[x as usize] >> 7 & 1; // store MSB of Vx in v[0xf]
                self.v[x as usize] <<= 1;
            }

            // Vx != Vy
            (0x9, x, y, 0x0) => {
                if self.v[x as usize] != self.v[y as usize] {
                    self.pc += 2;
                }
            }

            // I = N
            (0xA, n1, n2, n3) => {
                let n = (n1 << 8 | n2 << 4 | n3) as u16;
                self.i = n;
            }

            // Jmp V0 + N
            (0xB, n1, n2, n3) => {
                let n = (n1 << 8 | n2 << 4 | n3) as u16;
                self.pc += (self.v[0] as u16 + n - 2) as usize; // -2 to offset the increment below
            }

            // Vx = Rand() & N
            (0xC, x, n1, n2) => {
                let n = (n1 << 4 | n2) as u8;
                self.v[x as usize] = n & rand::thread_rng().gen_range(0, 0xFF);
            }

            // Drw Vx, Vy, N
            (0xD, x, y, n) => {
                let sprite_start = self.i as usize;
                let sprite_end = sprite_start + n as usize;
                let sprite = &ram[sprite_start .. sprite_end];
                
                let pixel_flip = display.draw(self.v[x as usize], self.v[y as usize], &sprite);
                self.v[0xF] =  if pixel_flip { 1 } else { 0 };
            }

            // Key == Vx
            (0xE, x, 0x9, 0xE) => {
                if keyboard.is_pressed(self.v[x as usize] as usize) {
                    self.pc += 2;
                }
            }

            // Key != Vx
            (0xE, x, 0xA, 0x1) => {
                if !keyboard.is_pressed(self.v[x as usize] as usize) {
                    self.pc += 2;
                }
            }

            // Vx = Timer
            (0xF, x, 0x0, 0x7) => {
                self.v[x as usize] = timer.get(elapsed_millis);
            }

            // Vx = Key
            (0xF, x, 0x0, 0xA) => {
                match keyboard.current_key() {
                    Some(k) => self.v[x as usize] = k as u8,
                    None => self.pc -= 2 // block on this instruction until key is pressed
                }
            }

            // Timer = Vx
            (0xF, x, 0x1, 0x5) => {
                timer.set(x as u8, elapsed_millis);
            }

            // Sound = Vx
            (0xF, x, 0x1, 0x8) => {
                //log!("Sound not supported: 0x{:x?}", opcode)
            }

            // I += Vx
            (0xF, x, 0x1, 0xE) => {
                self.i += self.v[x as usize] as u16;
            }

            // I = sprite[Vx]
            (0xF, x, 0x2, 0x9) => {
                self.i = (emulator::FONT_OFFSET + (emulator::FONT_WIDTH * self.v[x as usize] as usize)) as u16;
            }

            // I = BCD(Vx)
            (0xF, x, 0x3, 0x3) => {
                // memory->write(i, v[x] / 100);
                // memory->write(i + 1, (v[x] % 100) / 10);
                // memory->write(i  +2, (v[x] % 10));

                let vx = self.v[x as usize];
                ram[self.i as usize] = vx / 100;
                ram[self.i as usize + 1] = (vx % 100) / 10;
                ram[self.i as usize + 2] = vx % 10;
            }

            // Load [I], Vx (reg_dump)
            (0xF, x, 0x5, 0x5) => {
                for k in 0..x + 1 {
                    ram[self.i as usize + k as usize] = self.v[k as usize];
                }
            }

            // Load Vx, [I] (reg_load)
            (0xF, x, 0x6, 0x5) => {
                for k in 0..x + 1 {
                    self.v[k as usize] = ram[self.i as usize + k as usize];
                }
            }

            _ => {
                panic!("Unsupported opcode: 0x{:x?}", opcode)
            }
        };

        self.pc += 2;
    }
}
