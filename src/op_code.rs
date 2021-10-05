pub struct OpCode {
    pub op: u8,
    pub x: u8.
    pub y: u8,
    pub n: u8,
    pub nn: u8,
    pub nnnn: u16,
}

impl OpCode {
    pub fn from_hex(fbyte: u8, sbyte: u8) -> Self {
        Self {
            op: (fbyte >> 4) & 0xF,
            x: fbyte & 0xF,
            y: (sbyte >> 4) & 0xF,
            n: sbyte & 0xF,
            nn: sbyte,
            nnn: (u16::from(fbyte & 0xF) << 8) | u16::from(sbyte),
        }
    }
}