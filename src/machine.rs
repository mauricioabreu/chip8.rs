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
        }

        machine
    }

    pub fn decode(self: &mut Machine) -> OpCode {
        let op_code = OpCode::from_hex(
            self.ram[usize::from(self.pc)],
            self.ram[usize::from(self.pc + 1)]
        )
        self.pc += 2;

        op_code
    }

    pub fn execute(self: &mut Machine, op_code: OpCode) {
        match op_code.op {
            0u8 => {
                self.display = [[false; 32]; 64];
            },
            0x1u8 => {
                self.pc = op_code.nnn;
            },
            0x6u8 => {
                self.v[op_code.x] = op_code.nn;
            },
            0x7u8 => {
                self.v[op_code.x] += op_code.nn;
            },
            0xAu8 => {
                self.i = op_code.nnn;
            },
            0xDu8 => {
                
            }
        }
    }
}