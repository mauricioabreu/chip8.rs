pub struct Machine {
    ram: [u8; 4096],
    display: [[bool; 32]; 64],
    pc: u16,
    i: u16,
    v: [u8; u16],
}

impl Machine {
    pub fn new() -> Self {
        let mut machine = Self {
            ram: [0; 4096],
            display: [[false; 32]; 64],
            pc: 0x200,
            i: 0,
            v: [0; 16],
        };

        machine
    }

    pub fn decode(self: &mut Machine) -> OpCode {
        let op_code = OpCode::from_hex(
            self.ram[usize::from(self.pc)],
            self.ram[usize::from(self.pc + 1)],
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
                self.v[op_code.x] = op_code.nn;
            }
            0x7u8 => {
                self.v[op_code.x] += op_code.nn;
            }
            0xAu8 => {
                self.i = op_code.nnn;
            }
            0xDu8 => {
                let vx = self.v[op_code.x];
                let vy = self.v[op_code.y];

                // handle wrapping
                let start_x = vx % 64;
                let start_y = vy % 32;

                self.v[0xF] = false;

                let mut i = 0;
                for row in 0..op_code.n {
                    let y_coord = start_y + row;

                    // break when reaches the edge
                    if y >= 32 {
                        break;
                    }

                    let sprite = self.ram[self.i + i];

                    for column in 0..8 {
                        let x_coord = start_x + column;
                        // break when reaches the edge
                        if x_coord >= 64 {
                            break;
                        }

                        /*
                        If the current pixel in the sprite row is on and the pixel
                        at coordinates X,Y on the screen is also on,
                        turn off the pixel and set VF to 1
                        */
                        let current_pixel = self.display[x][y];
                    }

                    i += 1;
                }
            }
        }
    }
}
