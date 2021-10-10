use crate::op_code::OpCode;

const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;

pub struct Machine {
    memory: [u8; 4096],
    display: [[bool; 32]; 64],
    pc: u16,
    i: u16,
    v: [u8; 16],
}

impl Machine {
    pub fn new() -> Self {
        let machine = Self {
            memory: [0; 4096],
            display: [[false; 32]; 64],
            pc: 0x200,
            i: 0,
            v: [0; 16],
        };

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

        match op_code.op {
            0u8 => {
                println!("00E0: clear screen");
                self.display = [[false; 32]; 64];
            }
            0x1u8 => {
                println!("00EE: jump");
                self.pc = op_code.nnn;
            }
            0x6u8 => {
                println!("6XNN: set value {} to Vx{}", op_code.nn, op_code.x);
                self.register_vx(&op_code, op_code.nn);
            }
            0x7u8 => {
                println!("7XNN: add the value {} to Vx{}", op_code.nn, op_code.x);
                self.register_vx(&op_code, vx.wrapping_add(op_code.nn));
            }
            0xAu8 => {
                println!("ANNN: set index register I {}", op_code.nnn);
                self.i = op_code.nnn;
            }
            0xDu8 => {
                self.draw_on_display(op_code);
            }
            _ => panic!("OpCode not implemented!"),
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
        println!("DXYN: draw on screen at Vx{} Vy{}", vx, vy);

        // handle wrapping
        let start_x = usize::from(vx) % DISPLAY_WIDTH;
        let start_y = usize::from(vy) % DISPLAY_HEIGHT;

        self.v[0xF] = usize::from(false) as u8;

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

                if current_pixel != true && self.display[x_coord][y_coord] == false {
                    self.v[0xF] = usize::from(true) as u8;
                }
            }
        }
    }
}
