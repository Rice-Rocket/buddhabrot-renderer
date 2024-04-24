type Float = f32;


pub trait Color {
    fn empty() -> Self;
    fn add(&mut self, rhs: Self);
    fn max(self, rhs: Self) -> Self;
    fn map(self, f: impl Fn(Float) -> Float) -> Self;
}


#[derive(Clone, Copy, Debug)]
pub struct Rgb {
    pub r: Float,
    pub g: Float,
    pub b: Float,
}

impl Rgb { 
    /// Constructs a new RGB color from red, green, and blue component values.
    #[inline]
    pub fn new(r: Float, g: Float, b: Float) -> Rgb {
        Self { r, g, b }
    }
}

impl From<(Float, Float, Float)> for Rgb {
    #[inline]
    fn from(value: (Float, Float, Float)) -> Rgb {
        Self { r: value.0, g: value.1, b: value.2 }
    }
}

impl From<Rgb> for (Float, Float, Float) {
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

    #[inline]
    fn add(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }

    #[inline]
    fn max(self, rhs: Self) -> Self {
        Self {
            r: self.r.max(rhs.r),
            g: self.g.max(rhs.g),
            b: self.b.max(rhs.b),
        }
    }

    #[inline]
    fn map(self, f: impl Fn(Float) -> Float) -> Self {
        Self {
            r: f(self.r),
            g: f(self.g),
            b: f(self.b),
        }
    }
}


#[derive(Clone, Copy, Debug)]
pub struct Rgba {
    pub r: Float,
    pub g: Float,
    pub b: Float,
    pub a: Float,
}

impl Rgba {
    /// Constructs a new RGBA color from red, green, blue, and alpha component values.
    #[inline]
    pub fn new(r: Float, g: Float, b: Float, a: Float) -> Rgba {
        Self { r, g, b, a }
    }
}

impl From<(Float, Float, Float, Float)> for Rgba {
    #[inline]
    fn from(value: (Float, Float, Float, Float)) -> Rgba {
        Self { r: value.0, g: value.1, b: value.2, a: value.3 }
    }
}

impl From<Rgba> for (Float, Float, Float, Float) {
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

    #[inline]
    fn add(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
        self.a += rhs.a;
    }

    #[inline]
    fn max(self, rhs: Self) -> Self {
        Self {
            r: self.r.max(rhs.r),
            g: self.g.max(rhs.g),
            b: self.b.max(rhs.b),
            a: self.a.max(rhs.a),
        }
    }

    #[inline]
    fn map(self, f: impl Fn(Float) -> Float) -> Self {
        Self {
            r: f(self.r),
            g: f(self.g),
            b: f(self.b),
            a: f(self.a),
        }
    }
}
