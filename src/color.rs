pub trait Color {
    fn empty() -> Self;
}


pub struct Rgb {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Rgb { 
    /// Constructs a new RGB color from red, green, and blue component values.
    #[inline]
    pub fn new(r: f32, g: f32, b: f32) -> Rgb {
        Self { r, g, b }
    }
}

impl From<(f32, f32, f32)> for Rgb {
    #[inline]
    fn from(value: (f32, f32, f32)) -> Rgb {
        Self { r: value.0, g: value.1, b: value.2 }
    }
}

impl From<Rgb> for (f32, f32, f32) {
    #[inline]
    fn from(value: Rgb) -> Self {
        (value.r, value.g, value.b)
    }
}

impl Color for Rgb {
    #[inline]
    fn empty() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}


pub struct Rgba {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Rgba {
    /// Constructs a new RGBA color from red, green, blue, and alpha component values.
    #[inline]
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Rgba {
        Self { r, g, b, a }
    }
}

impl From<(f32, f32, f32, f32)> for Rgba {
    #[inline]
    fn from(value: (f32, f32, f32, f32)) -> Rgba {
        Self { r: value.0, g: value.1, b: value.2, a: value.3 }
    }
}

impl From<Rgba> for (f32, f32, f32, f32) {
    #[inline]
    fn from(value: Rgba) -> Self {
        (value.r, value.g, value.b, value.a)
    }
}

impl Color for Rgba {
    #[inline]
    fn empty() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }
}
