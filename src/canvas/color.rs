#[derive(Copy, Clone)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { color }
    }

    pub fn hex(raw_hex: &str) -> Self {
        Self::new(femtovg::Color::hex(raw_hex))
    }

    pub fn femtovg(&self) -> femtovg::Color {
        self.color
    }

    pub const fn with_alpha(self, alpha: f32) -> Self {
        Self::new(femtovg::Color {
            r: self.color.r,
            g: self.color.g,
            b: self.color.b,
            a: alpha,
        })
    }

    pub fn white() -> Color {
        Self::new(femtovg::Color::white())
    }

    pub fn black() -> Color {
        Self::new(femtovg::Color::black())
    }

    pub fn transparent() -> Color {
        Self::new(femtovg::Color::rgbaf(0., 0., 0., 0.))
    }
}
