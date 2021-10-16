pub struct OpCode {
    pub op: u8,
    pub x: u8,
    pub y: u8,
    pub n: u8,
    pub nn: u8,
    pub nnn: u16,
}

impl OpCode {
    pub fn from_bytes(f_byte: u8, s_byte: u8) -> Self {
        Self {
            op: (f_byte >> 4) & 0xF,
            x: f_byte & 0xF,
            y: (s_byte >> 4) & 0xF,
            n: s_byte & 0xF,
            nn: s_byte,
            nnn: (u16::from(f_byte & 0xF) << 8) | u16::from(s_byte),
        }
    }
}
