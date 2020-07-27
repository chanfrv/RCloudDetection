use image::RgbImage;

mod histogram;
mod classes;
use histogram::*;
use classes::*;

/// 5 dimensional vector.
type Vector5 = (u8, u8, u8, u8, u8);

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

/// Kmeans structure.
pub struct Kmeans
{
    cloud_coverage: f32
}

impl Kmeans
{
    /// Initialize the Kmeans structure.
    pub fn init() -> Kmeans
    {
        Kmeans
        {
            cloud_coverage: 0.0
        }
    }

    /// Creates the output image by applying a modified k-means algorithm to the input image.
    pub fn compute_image(&mut self, img_in: &RgbImage, img_out: &mut RgbImage) -> u32
    {
        // init the histogram
        let histo = histogram::Histogram::init(&img_in);
        // init the classes
        let mut classes = classes::Classes::init();
        let mut new_classes = classes.clone();

        let mut stable = false;
        let mut iterations = 0;

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
            stable = is_stable(&classes, &new_classes);
            // prepare the classes for the next iteration.
            classes = new_classes.clone();
        }

        for y in 0..img_in.height()
        {
            for x in 0..img_in.width()
            {
                // for each pixel, set the output pixel(x,y) as the input pixel(x,y) closest class.
                set_closest_class(&classes, img_in, img_out, x, y);
            }
        }
        // compute the cloud coverage.
        self.cloud_coverage = get_highest_class_coverage(&classes, &img_out);

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
    let mut index_min: u8 = 0;
    if index > 0
    {
        index_min = classes[index] - ((classes[index] - classes[index - 1]) / 2);
    }
    // get the maximum index.
    let mut index_max: u8 = 255;
    if index < CLASS_NUM - 1
    {
        index_max = classes[index] + ((classes[index + 1] - classes[index]) / 2);
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

/// Return true if lhs and rhs are close enough.
fn is_stable(lhs: &Classes, rhs: &Classes) -> bool
{
    for index in 0..CLASS_NUM
    {
        let diff = lhs[index] as i32 - rhs[index] as i32;
        if diff.abs() as usize >= CLASS_THRESHOLD
        {
            return false;
        }
    }
    return true;
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
    let mut components: Vector5 = (0, 0, 0, 0, 0);

    components.2 = get_pix_mean(img.get_pixel(x, y));
    if y > 0                { components.0 = get_pix_mean(img.get_pixel(x, y - 1)); }
    if x > 0                { components.1 = get_pix_mean(img.get_pixel(x - 1, y)); }
    if y < img.height() - 1 { components.3 = get_pix_mean(img.get_pixel(x, y + 1)); }
    if x < img.width() - 1  { components.4 = get_pix_mean(img.get_pixel(x + 1, y)); }

    return components;
}

/// Returns the vector5 norm.
fn get_components_norm(components: Vector5) -> f32
{
    f32::sqrt(
        f32::powf(components.0 as f32, 2.0) +
        f32::powf(components.1 as f32, 2.0) +
        f32::powf(components.2 as f32, 2.0) +
        f32::powf(components.3 as f32, 2.0) +
        f32::powf(components.4 as f32, 2.0))
}

/// Finds the closest class to the components of the current pixel.
fn set_closest_class(classes: &Classes, img_in: &RgbImage, img_out: &mut RgbImage, x: u32, y: u32)
{
    let components = get_components(img_in, x, y);

    let norm = get_components_norm(components);

    let mut base_index = 0;
    let base_norm = f32::sqrt(5.0 * f32::powf(classes[base_index] as f32, 2.0));
    let base_diff = f32::abs(base_norm - norm);

    for curr_index in 1..CLASS_NUM
    {
        let curr_norm = f32::sqrt(5.0 * f32::powf(classes[curr_index] as f32, 2.0));
        let curr_diff = f32::abs(curr_norm - norm);

        if curr_diff < base_diff
        {
            base_index = curr_index;
        }
    }

    //img_out.put_pixel(x, y, image::Rgb([classes[base_index]; 3]));

    let colormap_index: usize = base_index * COLORMAP_SIZE / CLASS_NUM;
    img_out.put_pixel(x, y, image::Rgb(COLORMAP[colormap_index]));
}

/// Recreates the histogram on the output image and computes the ratio clouds/total.
fn get_highest_class_coverage(classes: &Classes, img_out: &RgbImage) -> f32
{
    let mut other: f32 = 0.0;
    let mut clouds: f32 = 0.0;
    let mut index: usize = 0;

    let histo = Histogram::init(img_out);
    let clouds_min_index = classes[CLASS_NUM - 1] - (classes[CLASS_NUM - 1] - classes[CLASS_NUM - 2]) / 2;
    
    while index < clouds_min_index as usize
    {
        other += histo[index] as f32;
        index += 1;
    }

    while index < 256
    {
        clouds += histo[index] as f32;
        index += 1;
    }

    return clouds * 100.0 / (clouds + other);
}