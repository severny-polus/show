#[derive(Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

const fn hex_to_u8(hex: u8) -> u8 {
    match hex {
        b'0'..=b'9' => hex - b'0',
        b'a'..=b'f' => hex - b'a' + 10,
        b'A'..=b'F' => hex - b'A' + 10,
        _ => panic!("incorrect hex value"),
    }
}

const fn hex2_to_u8(a: u8, b: u8) -> u8 {
    16 * hex_to_u8(a) + hex_to_u8(b)
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub const fn hex(hex_str: &str) -> Self {
        let h = hex_str.as_bytes();
        let i: usize = if h[0] == b'#' { 1 } else { 0 };
        match h.len() - i {
            8 => Self::new(
                hex2_to_u8(h[i + 0], h[i + 1]),
                hex2_to_u8(h[i + 2], h[i + 3]),
                hex2_to_u8(h[i + 4], h[i + 5]),
                hex2_to_u8(h[i + 6], h[i + 7]),
            ),
            6 => Self::new(
                hex2_to_u8(h[i + 0], h[i + 1]),
                hex2_to_u8(h[i + 2], h[i + 3]),
                hex2_to_u8(h[i + 4], h[i + 5]),
                0xFF,
            ),
            _ => panic!("incorrect hex string length"),
        }
    }

    pub const fn from_array(v: [u8; 4]) -> Self {
        Self::new(v[0], v[1], v[2], v[3])
    }

    pub fn to_array(self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }

    pub fn blend(self, other: Self) -> Self {
        let a = other.a as u16;
        Self::new(
            ((a * other.r as u16 + (255 - a) * self.r as u16) / 255) as u8,
            ((a * other.g as u16 + (255 - a) * self.g as u16) / 255) as u8,
            ((a * other.b as u16 + (255 - a) * self.b as u16) / 255) as u8,
            self.a,
        )
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

    pub fn to_vec3(self) -> [f32; 3] {
        [self.r, self.g, self.b].map(|b| b as f32 / 255.)
    }

    pub fn to_vec4(self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a].map(|b| b as f32 / 255.)
    }
}

impl From<&str> for Color {
    fn from(value: &str) -> Self {
        Color::hex(value)
    }
}
