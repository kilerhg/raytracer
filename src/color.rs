#[derive(Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color {
            r: r.min(1.0),
            g: g.min(1.0),
            b: b.min(1.0),
        }
    }
    pub fn add(a: Color, b: Color) -> Self {
        Color::new(a.r + b.r, a.g + b.g, a.b + b.b)
    }
    pub fn from_hex(hex: u32) -> Self {
        Color::from_rgb((hex >> 16) as u8, (hex >> 8) as u8, hex as u8)
    }

    pub fn blend(a: Color, b: Color, weight: f64) -> Self {
        let weight_a = weight.clamp(0.0, 1.0);
        let weight_b = 1.0 - weight_a;

        Color::add_weighted(a, weight_a, b, weight_b)
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Color {
            r: (r / 255) as f64,
            g: (g / 255) as f64,
            b: (b / 255) as f64,
        }
    }

    fn add_weighted(a: Color, weight_a: f64, b: Color, weight_b: f64) -> Self {
        let sum = weight_a + weight_b;
        Color {
            r: ((a.r * weight_a) + (b.r * weight_b)) / sum,
            g: ((a.g * weight_a) + (b.g * weight_b)) / sum,
            b: ((a.b * weight_a) + (b.b * weight_b)) / sum,
        }
    }
}
