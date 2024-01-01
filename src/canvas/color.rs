#[derive(Copy, Clone)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

const fn hex_to_u8(hex: &str) -> u8 {
    u8::from_str_radix(hex, 16).unwrap()
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub const fn hex(hex_str: &str) -> Self {
        let hex = hex_str.trim_start_matches("#");
        match hex.len() {
            8 => Self::new(
                hex_to_u8(&hex[0..2]).unwrap(),
                hex_to_u8(&hex[2..4]).unwrap(),
                hex_to_u8(&hex[4..6]).unwrap(),
                hex_to_u8(&hex[6..8]).unwrap(),
            ),
            6 => Self::new(
                hex_to_u8(&hex[0..2]).unwrap(),
                hex_to_u8(&hex[2..4]).unwrap(),
                hex_to_u8(&hex[4..6]).unwrap(),
                0xFF,
            ),
            _ => panic!("cannot parse color: {}", hex_str),
        }
    }

    pub const fn with_alpha(self, alpha: u8) -> Self {
        Self::new(self.r, self.g, self.b, alpha)
    }

    pub const fn white() -> Color {
        Self::new(0xFF, 0xFF, 0xFF, 0xFF)
    }

    pub const fn black() -> Color {
        Self::new(0x00, 0x00, 0x00, 0xFF)
    }

    pub const fn transparent() -> Color {
        Self::new(0x00, 0x00, 0x00, 0x00)
    }
}
