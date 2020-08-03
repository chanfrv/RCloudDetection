use image::{Rgb, RgbImage};
use std::ops::{Index, IndexMut, Range, RangeFrom, RangeTo};

/// Histogram structure.
pub struct Histogram {
    array: [u32; 256],
}

impl Histogram {
    /// Initialize an histogram and fills it with the input image pixels.
    pub fn new(img: &RgbImage) -> Self {
        let mut histo = Self { array: [0; 256] };

        for y in 0..img.height() {
            for x in 0..img.width() {
                // add one occurence of the pixel (x, y) in the histogram.
                let curr_pixel = get_pix_mean(&img[(x, y)]) as usize;
                histo[curr_pixel] += 1;
            }
        }
        return histo;
    }
}

/// Immutable array operator `[]`.
impl Index<usize> for Histogram {
    type Output = u32;
    fn index(&self, index: usize) -> &Self::Output {
        &self.array[index]
    }
}

impl Index<Range<usize>> for Histogram {
    type Output = [u32];
    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.array[index]
    }
}

impl Index<RangeFrom<usize>> for Histogram {
    type Output = [u32];
    fn index(&self, index: RangeFrom<usize>) -> &Self::Output {
        &self.array[index]
    }
}

impl Index<RangeTo<usize>> for Histogram {
    type Output = [u32];
    fn index(&self, index: RangeTo<usize>) -> &Self::Output {
        &self.array[index]
    }
}

/// Mutable array operator `[]`.
impl IndexMut<usize> for Histogram {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.array[index]
    }
}

/// Returns the mean of a pixel `(r + g + b) / 3`.
pub fn get_pix_mean(pixel: &Rgb<u8>) -> u8 {
    (pixel.0.iter().map(|&x| x as u32).sum::<u32>() / 3) as u8
}
