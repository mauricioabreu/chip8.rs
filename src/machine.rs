use crate::op_code::OpCode;
use rand::Rng;

const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;

static FONTS: [u8; 80] = [
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
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Machine {
    memory: [u8; 4096],
    display: [[bool; 32]; 64],
    pc: u16,
    i: u16,
    v: [u8; 16],
    delay_timer: u8,
    sound_timer: u8,
    keypad: [bool; 16],
    stack: Vec<u16>,
}

impl Machine {
    pub fn new() -> Self {
        let mut machine = Self {
            memory: [0; 4096],
            display: [[false; 32]; 64],
            pc: 0x200,
            i: 0,
            v: [0; 16],
            delay_timer: 0,
            sound_timer: 0,
            keypad: [false; 16],
            stack: Vec::new(),
        };

        for (n, font) in FONTS.iter().enumerate() {
            machine.memory[0x50 + n] = *font;
        }

        machine
    }

    pub fn load_rom(self: &mut Machine, data: Vec<u8>) {
        for (i, b) in data.iter().enumerate() {
            self.memory[0x200 + i] = *b;
        }
    }

    pub fn decode_op(self: &mut Machine) -> OpCode {
        let op_code = OpCode::from_bytes(
            self.memory[usize::from(self.pc)],
            self.memory[usize::from(self.pc + 1)],
        );
        self.pc += 2;

        op_code
    }

    pub fn execute_op(self: &mut Machine, op_code: OpCode) {
        let vx = self.fetch_vx(&op_code);
        let vy = self.fetch_vy(&op_code);

        match op_code.op {
            0u8 => {
                match op_code.n {
                    0u8 => {
                        println!("00E0: clear screen");
                        self.display = [[false; 32]; 64];
                    }
                    0xEu8 => {
                        self.pc = self.stack.pop().expect("Stack is empty");
                        println!("00EE: return from subroutine {}", self.pc)
                    }
                    _ => panic!("OpCode {:#04x}{} not implemented!", op_code.op, op_code.n),
                }
            }
            0x1u8 => {
                println!("00EE: jump to {}", op_code.nnn);
                self.pc = op_code.nnn;
            }
            0x2u8 => {
                println!("2NNN: set PC to NNN {}", op_code.nnn);
                self.stack.push(self.pc);
                self.pc = op_code.nnn;
            }
            0x3u8 => {
                println!("3XNN: skip when Vx{} == NN{}", vx, op_code.nn);
                if vx == op_code.nn {
                    self.pc += 2;
                }
            }
            0x4u8 => {
                println!("4XNN: skip when Vx{} != NN{}", vx, op_code.nn);
                if vx != op_code.nn {
                    self.pc += 2;
                }
            }
            0x5u8 => {
                println!("5XY0: skip when Vx{} == Vy{}", vx, vy);
                if vx == vy {
                    self.pc += 2;
                }
            }
            0x9u8 => {
                println!("9XY0: skip when Vx{} != Vy{}", vx, vy);
                if vx != vy {
                    self.pc += 2;
                }
            }
            0x6u8 => {
                println!("6XNN: set value {} to Vx{}", op_code.nn, op_code.x);
                self.register_vx(&op_code, op_code.nn);
            }
            0x7u8 => {
                println!(
                    "7XNN: add the value {} to Vx({}){}",
                    op_code.nn, vx, op_code.x
                );
                self.register_vx(&op_code, vx.wrapping_add(op_code.nn));
            }
            0x8u8 => match op_code.n {
                0x0u8 => {
                    self.register_vx(&op_code, vy);
                }
                0x1u8 => {
                    self.register_vx(&op_code, vx | vy);
                }
                0x2u8 => {
                    self.register_vx(&op_code, vx & vy);
                }
                0x3u8 => {
                    self.register_vx(&op_code, vx ^ vy);
                }
                0x4u8 => {
                    self.register_vx(&op_code, vx.wrapping_add(vy));
                    self.v[0xF] = usize::from(self.fetch_vx(&op_code) < vx) as u8;
                }
                0x5u8 => {
                    self.register_vx(&op_code, vx.wrapping_sub(vy));
                    self.v[0xF] = usize::from(vx > vy) as u8;
                }
                0x7u8 => {
                    self.register_vx(&op_code, vy.wrapping_sub(vx));
                    self.v[0xF] = usize::from(vy > vx) as u8;
                }
                0x6u8 => {
                    self.register_vx(&op_code, vx >> 1); // shift 1 bit right
                    self.v[0xF] = usize::from(vx & 0b00000001u8 != 0) as u8;
                }
                0xEu8 => {
                    self.register_vx(&op_code, vx << 1); // shift 1 bit left
                    self.v[0xF] = usize::from(vx & 0b10000000u8 != 0) as u8;
                }
                _ => panic!("OpCode {:#04x}{} not implemented!", op_code.op, op_code.n),
            },
            0xAu8 => {
                println!("ANNN: set index register I {}", op_code.nnn);
                self.i = op_code.nnn;
            }
            0xBu8 => {
                self.pc = op_code.nnn + u16::from(self.v[0]);
            }
            0xCu8 => {
                println!("CXNN: set random value in VX");
                let rand_number: u8 = rand::thread_rng().gen();
                self.register_vx(&op_code, op_code.nn & rand_number);
            }
            0xDu8 => {
                self.draw_on_display(op_code);
            }
            0xEu8 => match op_code.nn {
                0x9Eu8 => {
                    if self.keypad[usize::from(vx)] && vx < 16 {
                        self.pc += 2;
                    }
                }
                0xA1u8 => {
                    if !self.keypad[usize::from(vx)] && vx < 16 {
                        self.pc += 2;
                    }
                }
                _ => panic!("OpCode {:#04x}{} not implemented!", op_code.op, op_code.nn),
            },
            0xFu8 => match op_code.nn {
                0x07u8 => {
                    self.register_vx(&op_code, self.delay_timer);
                }
                0x15u8 => {
                    self.delay_timer = vx;
                }
                0x18u8 => {
                    self.sound_timer = vx;
                }
                0x1Eu8 => {
                    self.i += u16::from(vx);
                }
                0x0Au8 => {
                    // Block instruction and wait for key input.
                    // If a key is not pressed, the PC should be decremented
                    // because we increment PC every instruction we fetch.
                    let mut key_pressed = false;
                    for (i, k) in self.keypad.iter().enumerate() {
                        if *k {
                            self.register_vx(&op_code, i as u8); // set index of first pressed key
                            key_pressed = true;
                            break;
                        }
                    }

                    if !key_pressed {
                        self.pc -= 2;
                    }
                }
                0x29u8 => {
                    self.i = vx as u16 * 5;
                }
                0x33u8 => {
                    let digits = self.digits_from_number(vx);
                    self.memory[usize::from(self.i)] = digits[0];
                    self.memory[usize::from(self.i + 1)] = digits[1];
                    self.memory[usize::from(self.i + 2)] = digits[2];
                }
                0x55u8 => {
                    for i in 0..usize::from(op_code.x + 1) {
                        self.memory[usize::from(self.i) + i] = self.v[i];
                    }
                }
                0x65u8 => {
                    for i in 0..usize::from(op_code.x + 1) {
                        self.v[i] = self.memory[usize::from(self.i) + i];
                    }
                }
                _ => panic!("OpCode {:#04x}{} not implemented!", op_code.op, op_code.nn),
            },
            _ => panic!("OpCode {:#04x} not implemented!", op_code.op),
        }
    }

    fn register_vx(self: &mut Machine, op_code: &OpCode, value: u8) {
        self.v[usize::from(op_code.x)] = value;
    }

    fn fetch_vx(self: &mut Machine, op_code: &OpCode) -> u8 {
        self.v[usize::from(op_code.x)]
    }

    fn fetch_vy(self: &mut Machine, op_code: &OpCode) -> u8 {
        self.v[usize::from(op_code.y)]
    }

    pub fn read_display(&self) -> &[[bool; 32]; 64] {
        &self.display
    }

    fn draw_on_display(self: &mut Machine, op_code: OpCode) {
        /* Draw on display in the register using the vx, vy coordinates.

        If the current pixel in the sprite row is on and the pixel
        at coordinates X,Y on the screen is also on,
        turn off the pixel (XOR).
        */
        let vx = self.fetch_vx(&op_code);
        let vy = self.fetch_vy(&op_code);
        println!("DXYN: draw {} on screen at Vx{} Vy{}", op_code.n, vx, vy);

        // handle wrapping
        let start_x = usize::from(vx) % DISPLAY_WIDTH;
        let start_y = usize::from(vy) % DISPLAY_HEIGHT;

        let mut flagged = false;
        self.v[0xF] = usize::from(flagged) as u8;

        for row in 0..op_code.n {
            let y_coord = start_y + row as usize;

            // break when reaches the edge
            if y_coord >= 32 {
                break;
            }

            let sprite = self.memory[usize::from(self.i + row as u16)];

            for column in 0..8 {
                let x_coord = start_x + column;
                // break when reaches the edge
                if x_coord >= 64 {
                    break;
                }

                let current_pixel = self.display[x_coord][y_coord];
                let new_pixel = (sprite >> (7 - column)) & 1 != 0;
                self.display[x_coord][y_coord] = current_pixel ^ new_pixel;
                flagged = flagged || (current_pixel && new_pixel);
            }
        }
        self.v[0xF] = usize::from(flagged) as u8;
        self.debug_draw();
    }

    fn debug_draw(self: &mut Machine) {
        for y in 0..32 {
            for x in 0..64 {
                if !self.display[x][y] {
                    print!("_");
                } else {
                    print!("#");
                }
            }
            println!();
        }
    }

    pub fn tick(self: &mut Machine) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
    }

    pub fn keydown(self: &mut Machine, key: usize) {
        self.keypad[key] = true;
    }

    pub fn keyup(self: &mut Machine, key: usize) {
        self.keypad[key] = false;
    }

    fn digits_from_number(self: &mut Machine, n: u8) -> Vec<u8> {
        let third = n % 10;
        let second = (n % 100) / 10;
        let first = n / 100;

        vec![first, second, third]
    }
}
