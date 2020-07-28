use image::{Rgb, RgbImage};

mod histogram;
mod classes;
use histogram::*;
use classes::*;

/// 5 dimensional vector.
type Vector5 = [u8; 5];

/// Iteration counter lmit for the kemean algorithm.
const ITERATION_LIMIT: u32      = 30;

/// Colormap
const COLORMAP_SIZE: usize      = 10;
const COLORMAP: [[u8; 3]; COLORMAP_SIZE] =
[
    [0,   0,   255],
    [0,   127, 255],
    [0,   255, 255],
    [0,   255, 127],
    [0,   255,   0],
    [127, 255,   0],
    [255, 255,   0],
    [255, 127,   0],
    [255,   0,   0],
    [127,   0,  55]
];
const COLORMAP_CLOUD_THRESHOLD: usize = 2;

/// Kmeans structure.
pub struct Kmeans
{
    cloud_coverage: f32,
}

impl Kmeans
{
    /// Initialize the Kmeans structure.
    pub fn new() -> Kmeans
    {
        Kmeans
        {
            cloud_coverage: 0.0,
        }
    }

    /// Creates the output image by applying a modified k-means algorithm to the input image.
    pub fn compute_image(&mut self, img_in: &RgbImage, img_out: &mut RgbImage) -> u32
    {
        // init the histogram
        let histo = histogram::Histogram::new(&img_in);
        // init the classes
        let mut classes = classes::Classes::new();
        let mut new_classes = classes.clone();

        let mut stable = false;
        let mut iterations = 0;
        let mut colormap_histo: [u32; COLORMAP_SIZE] = [0; COLORMAP_SIZE];

        // while the system is not stable and the iteration limit has not been reached
        while stable == false && iterations < ITERATION_LIMIT
        {
            iterations += 1;

            // for each class
            for index in 0..classes::CLASS_NUM
            {
                // move the current class according to the kmeans algorithm.
                new_classes[index] = kmeans_mod(&histo, &classes, index);
            }
            // detects if the new class barely changes.
            stable = classes.is_stable(&new_classes);
            // prepare the classes for the next iteration.
            classes = new_classes.clone();
        }

        for y in 0..img_in.height()
        {
            for x in 0..img_in.width()
            {
                // for each pixel, set the output pixel(x,y) as the input pixel(x,y) closest class.
                let colormap_index = set_closest_class(&classes, img_in, img_out, x, y);
                colormap_histo[colormap_index] += 1;
            }
        }
        // compute the cloud coverage.
        //self.cloud_coverage = get_highest_class_coverage(&classes, &img_out);

        let colormap_histo_filter: Vec<&u32> = colormap_histo.iter().filter(|&e| *e != 0).rev().collect();
        let clouds: u32 = colormap_histo_filter[..COLORMAP_CLOUD_THRESHOLD].iter().cloned().sum();
        let others: u32 = colormap_histo_filter[COLORMAP_CLOUD_THRESHOLD..].iter().cloned().sum();
        self.cloud_coverage = (clouds * 100) as f32 / (clouds + others) as f32;

        return iterations;
    }

    /// Cloud coverage getter.
    /// 
    /// The cloud coverage is computed in the [`compute_image`] method.
    pub fn get_cloud_coverage(&self) -> f32
    {
        return self.cloud_coverage;
    }
}

/// Computes the kmeans algorithm for the given index in the class array.
fn kmeans_mod(histogram: &Histogram, classes: &Classes, index: usize) -> u8
{
    let mut mean: u32 = 0;
    let mut sum: u32 = 0;
    
    // get the minimum index.
    let mut index_min: usize = 0;
    if index > 0
    {
        index_min = (classes[index] - (classes[index] - classes[index - 1]) / 2) as usize;
    }
    // get the maximum index.
    let mut index_max: usize = 255;
    if index < CLASS_NUM - 1
    {
        index_max = (classes[index] + (classes[index + 1] - classes[index]) / 2) as usize;
    }

    // compute the sum and the mean between both indexes.
    for pixel_index in index_min..index_max
    {
        sum += histogram[pixel_index as usize] as u32;
        mean += histogram[pixel_index as usize] as u32 * pixel_index as u32;
    }

    if sum == 0
    {
        return index as u8;
    }
    // return the mean index.
    return u8::from((mean / sum) as u8);
}

/// Returns a 5-tuple containing a pixel and its 4 neighbours pixel mean values.println!
/// 
/// [`components`] indexes:
/// ```
///     ... | 0 | ...
/// ... | 1 | 2 | 3 | ...
///     ... | 4 | ...
/// ```
/// If the component is out of bounds, its value is simply 0.
fn get_components(img: &RgbImage, x: u32, y: u32) -> Vector5
{
    let components: Vector5 =
    [
        if y > 0                { get_pix_mean(img.get_pixel(x, y - 1)) } else { 0 },
        if x > 0                { get_pix_mean(img.get_pixel(x - 1, y)) } else { 0 },
                                  get_pix_mean(img.get_pixel(x, y)),
        if y < img.height() - 1 { get_pix_mean(img.get_pixel(x, y + 1)) } else { 0 },
        if x < img.width() - 1  { get_pix_mean(img.get_pixel(x + 1, y)) } else { 0 }
    ];

    return components;
}

/// Returns the vector5 norm.
fn get_components_norm(components: Vector5) -> f32
{
    components.iter().map(|&c| (c as f32).powf(2.0)).sum::<f32>().sqrt()    
}

/// Finds the closest class to the components of the current pixel.
fn set_closest_class(classes: &Classes, img_in: &RgbImage, img_out: &mut RgbImage, x: u32, y: u32) -> usize
{
    let components = get_components(img_in, x, y);

    let norm = get_components_norm(components);

    let mut base_index = 0;
    let mut base_diff = f32::MAX;

    for curr_index in 0..CLASS_NUM
    {
        let curr_norm = f32::sqrt(5.0 * f32::powf(classes[curr_index] as f32, 2.0));
        let curr_diff = f32::abs(curr_norm - norm);

        if curr_diff < base_diff
        {
            base_diff = curr_diff;
            base_index = curr_index;
        }
    }

    // grayscale output
    //img_out.put_pixel(x, y, image::Rgb([classes[base_index]; 3]));

    // colored output with the colormap values
    let colormap_index: usize = base_index * COLORMAP_SIZE / CLASS_NUM;
    img_out.put_pixel(x, y, Rgb(COLORMAP[colormap_index]));
    return colormap_index;
}

/// Recreates the histogram on the output image and computes the ratio clouds/total.
fn _get_highest_class_coverage(classes: &Classes, img_out: &RgbImage) -> f32
{
    // build the histogram with the new pixel values
    let histo = Histogram::new(img_out);

    // get the frontier cloud/everything else
    // [ c0 c1 ... cN-2 <frontier> cN-1 ] => frontier = cN-1 - (cN-1 - cN-2) / 2
    let clouds_min_index: usize = (classes[CLASS_NUM - 1] - (classes[CLASS_NUM - 1] - classes[CLASS_NUM - 2]) / 2) as usize;
    
    // divide the hisogram in 2 chunks and sum them
    let other = histo[..clouds_min_index].iter().cloned().sum::<u32>() as f32;
    let clouds = histo[clouds_min_index..].iter().cloned().sum::<u32>() as f32;

    // return the cloud ratio
    return clouds * 100.0 / (clouds + other);
}