use crate::color::Color;

pub struct Image<T: Color, const D: usize, const W: usize> {
    data: [T; D]
}

impl<T: Color + Clone + Copy, const D: usize, const W: usize> Image<T, D, W> {
    #[inline]
    pub fn new() -> Image<T, D, W> {
        Self { data: [T::empty(); D] }
    }

    #[inline]
    pub fn get(&self, px: (usize, usize)) -> T {
        self.data[px.1 * W + px.0]
    }

    #[inline]
    pub fn set(&mut self, px: (usize, usize), col: T) {
        self.data[px.1 * W + px.0] = col;
    }
}
