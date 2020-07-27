use std::env;
use std::string::String;
use std::path::Path;
use std::vec::Vec;
use std::time::Duration;
use show_image::{ImageInfo, make_window};
mod kmeans;
use kmeans::*;

/// Main function
fn main() -> Result<(), String>
{
    let args: Vec<String> = env::args().collect();

    if args.len() == 2
    {
        let path = Path::new(&args[1]);

        let img_ori = image::open(&path).unwrap().into_rgb();
        let mut img_res = img_ori.clone();

        let (width, height) = img_res.dimensions();
        let rgb_image = ImageInfo::rgb8(width as usize, height as usize);

        let mut kc = Kmeans::init();
        kc.compute_image(&img_ori, &mut img_res);

        let image_out = (img_res.into_raw(), &rgb_image);
        let window_out = make_window("image out")?;
        window_out.set_image(image_out, "image-002")?;

        while let Ok(event) = window_out.wait_key(Duration::new(60, 0))
        {
            if let Some(_) = event
            {
                break;
            }
        }
    }
    else
    {
        println!("Usage: {} image", args[0]);
    }
    Ok(())
}
