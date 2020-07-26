use image::{Rgb, RgbImage};
use std::ops::{Index, IndexMut};

/// Histogram structure.
pub struct Histogram
{
    array: [u32; 256]
}

impl Histogram
{
    /// Initialize an histogram and fills it with the input image pixels.
    pub fn init(img: &RgbImage) -> Histogram
    {
        let mut histo = Histogram{array: [0; 256]};

        for y in 0..img.height()
        {
            for x in 0..img.width()
            {
                // add one occurence of the pixel (x, y) in the histogram.
                let curr_pixel = get_pix_mean(&img[(x, y)]) as usize;
                histo[curr_pixel] += 1;
            }
        }
        return histo;
    }
}

/// Immutable array operator `[]`.
impl Index<usize> for Histogram
{
    type Output = u32;
    fn index(&self, index: usize) -> &Self::Output
    {
        &self.array[index]
    }
}

/// Mutable array operator `[]`.
impl IndexMut<usize> for Histogram
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output
    {
        &mut self.array[index]
    }
}

/// Returns the mean of a pixel `(r + g + b) / 3`.
pub fn get_pix_mean(pixel: &Rgb<u8>) -> u8
{
    let mut sum: u32 = 0;
    for i in 0..3 { sum += pixel[i] as u32;}
    return (sum / 3) as u8;
}