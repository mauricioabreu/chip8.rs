const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 64;

pub struct Machine {
    memory: [u8; 4096],
    display: [[bool; 32]; 64],
    pc: u16,
    i: u16,
    v: [u8; u16],
}

impl Machine {
    pub fn new() -> Self {
        let mut machine = Self {
            memory: [0; 4096],
            display: [[false; 32]; 64],
            pc: 0x200,
            i: 0,
            v: [0; 16],
        };

        machine
    }

    pub fn decode(self: &mut Machine) -> OpCode {
        let op_code = OpCode::from_hex(
            self.memory[usize::from(self.pc)],
            self.memory[usize::from(self.pc + 1)],
        );
        self.pc += 2;

        op_code
    }

    pub fn execute(self: &mut Machine, op_code: OpCode) {
        match op_code.op {
            0u8 => {
                self.display = [[false; 32]; 64];
            }
            0x1u8 => {
                self.pc = op_code.nnn;
            }
            0x6u8 => {
                self.v[op_code.x as usize] = op_code.nn;
            }
            0x7u8 => {
                self.v[op_code.x as usize] += op_code.nn;
            }
            0xAu8 => {
                self.i = op_code.nnn;
            }
            0xDu8 => {
                self.draw_on_display(self, op_code);
            }
        }
    }

    fn draw_on_display(self: &mut Machine, op_code: OpCode) {
        /* Draw on display in the register using the vx, vy coordinates.

        If the current pixel in the sprite row is on and the pixel
        at coordinates X,Y on the screen is also on,
        turn off the pixel (XOR).
        */
        let vx = self.v[op_code.x as usize];
        let vy = self.v[op_code.y as usize];

        // handle wrapping
        let start_x = vx % DISPLAY_WIDTH;
        let start_y = vy % DISPLAY_HEIGHT;

        self.v[0xF] = false;

        for row in 0..op_code.n {
            let y_coord = start_y + row;

            // break when reaches the edge
            if y_coord >= 32 {
                break;
            }

            let sprite = self.memory[self.i + row as u16];

            for column in 0..8 {
                let x_coord = start_x + column;
                // break when reaches the edge
                if x_coord >= 64 {
                    break;
                }

                let current_pixel = self.display[x][y];
                let new_pixel = (sprite >> (7 - column)) & 1;
                self.display[x][y] = current_pixel ^ new_pixel;

                if current_pixel != 0 && self.display[x][y] == 0 {
                    self.v[0xF] = true;
                }
            }
        }
    }
}
