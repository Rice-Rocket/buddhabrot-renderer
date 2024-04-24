use std::slice::{Iter, IterMut};

use crate::color::Color;

pub struct Image<T: Color, const D: usize, const W: usize> {
    data: Vec<T>
}

impl<T: Color + Clone + Copy, const D: usize, const W: usize> Image<T, D, W> {
    /// Creates a new, blank image.
    #[inline]
    pub fn new() -> Image<T, D, W> {
        Self { data: vec![T::empty(); D] }
    }

    /// Gets the value of a pixel at a given `(x, y)` pixel position.
    #[inline]
    pub fn get(&self, px: (usize, usize)) -> T {
        self.data[px.1 * W + px.0]
    }

    /// Sets the value of a pixel at a given `(x, y)` pixel position.
    #[inline]
    #[allow(dead_code)]
    pub fn set(&mut self, px: (usize, usize), col: T) {
        self.data[px.1 * W + px.0] = col;
    }

    /// Swaps two pixels
    #[inline]
    pub fn swap(&mut self, p1: (usize, usize), p2: (usize, usize)) {
        self.data.swap(p1.1 * W + p1.0, p2.1 * W + p2.0);
    }

    /// Adds to the value of a pixel at a given `(x, y)` pixel position.
    #[inline]
    pub fn add(&mut self, px: (usize, usize), col: T) {
        self.data[px.1 * W + px.0].add(col);
    }

    /// Tests if the value of a pixel at a given `(x, y)` pixel position is inside the bounds of
    /// the image.
    #[inline]
    pub fn is_inside(&self, px: (i32, i32)) -> bool {
        (px.0 >= 0) && (px.1 >= 0) && (px.0 < W as i32) && (px.1 < (D / W) as i32)
    }

    /// Get an iterator over every pixel in the image.
    #[inline]
    pub fn pixels(&self) -> Pixels<T, D> {
        Pixels { iter: self.data.iter() }
    }

    /// Get a mutable iterator over every pixel in the image.
    #[inline]
    pub fn pixels_mut(&mut self) -> PixelsMut<T, D> {
        PixelsMut { iter: self.data.iter_mut() }
    }

    #[inline]
    #[allow(dead_code)]
    pub fn enumerate_pixels(&self) -> EnumeratePixels<T, D, W> {
        EnumeratePixels { iter: self.data.iter(), index: 0 }
    }

    #[inline]
    #[allow(dead_code)]
    pub fn enumerate_pixels_mut(&mut self) -> EnumeratePixelsMut<T, D, W> {
        EnumeratePixelsMut { iter: self.data.iter_mut(), index: 0 }
    }
}

impl<T: Color + Clone + Copy, const D: usize, const W: usize> Default for Image<T, D, W> {
    fn default() -> Self {
        Self::new()
    }
}


pub struct Pixels<'a, T: Color, const D: usize> {
    iter: Iter<'a, T>,
}

impl<'a, T: Color, const D: usize> Iterator for Pixels<'a, T, D> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

pub struct PixelsMut<'a, T: Color, const D: usize> {
    iter: IterMut<'a, T>,
}

impl<'a, T: Color, const D: usize> Iterator for PixelsMut<'a, T, D> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}


pub struct EnumeratePixels<'a, T: Color, const D: usize, const W: usize> {
    index: usize,
    iter: Iter<'a, T>,
}

impl<'a, T: Color, const D: usize, const W: usize> Iterator for EnumeratePixels<'a, T, D, W> {
    type Item = (usize, usize, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= D {
            return None;
        }
        self.index += 1;

        Some((self.index % W, self.index / W, self.iter.next()?))
    }
}


pub struct EnumeratePixelsMut<'a, T: Color, const D: usize, const W: usize> {
    index: usize,
    iter: IterMut<'a, T>,
}

impl<'a, T: Color, const D: usize, const W: usize> Iterator for EnumeratePixelsMut<'a, T, D, W> {
    type Item = (usize, usize, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= D {
            return None;
        }
        self.index += 1;

        Some((self.index % W, self.index / W, self.iter.next()?))
    }
}
