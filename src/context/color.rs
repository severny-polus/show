#[derive(Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

const fn hex_to_u8(hex: u8) -> u8 {
    match hex {
        b'0'..=b'9' => hex - b'0',
        b'a'..=b'f' => hex - b'a' + 10,
        b'A'..=b'F' => hex - b'A' + 10,
        _ => panic!("incorrect hex value"),
    }
}

fn hex2_to_f32(a: u8, b: u8) -> f32 {
    (16 * hex_to_u8(a) + hex_to_u8(b)) as f32 / 255.
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn hex(hex_str: &str) -> Self {
        let h = hex_str.as_bytes();
        let i: usize = if h[0] == b'#' { 1 } else { 0 };
        match h.len() - i {
            8 => Self::new(
                hex2_to_f32(h[i + 0], h[i + 1]),
                hex2_to_f32(h[i + 2], h[i + 3]),
                hex2_to_f32(h[i + 4], h[i + 5]),
                hex2_to_f32(h[i + 6], h[i + 7]),
            ),
            6 => Self::new(
                hex2_to_f32(h[i + 0], h[i + 1]),
                hex2_to_f32(h[i + 2], h[i + 3]),
                hex2_to_f32(h[i + 4], h[i + 5]),
                1.,
            ),
            _ => panic!("incorrect hex string length"),
        }
    }

    pub fn from_array(v: [f32; 4]) -> Self {
        Self::new(v[0], v[1], v[2], v[3])
    }

    pub fn to_array(self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }

    pub fn with_alpha(self, alpha: f32) -> Self {
        Self::new(self.r, self.g, self.b, alpha)
    }

    pub fn white() -> Color {
        Self::new(1., 1., 1., 1.)
    }

    pub fn black() -> Color {
        Self::new(0., 0., 0., 1.)
    }

    pub fn transparent() -> Color {
        Self::new(0., 0., 0., 0.)
    }

    pub fn to_vec3(self) -> [f32; 3] {
        [self.r, self.g, self.b].map(|b| b as f32 / 255.)
    }

    pub fn to_vec4(self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a].map(|b| b as f32 / 255.)
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::transparent()
    }
}

impl From<&str> for Color {
    fn from(value: &str) -> Self {
        Color::hex(value)
    }
}
