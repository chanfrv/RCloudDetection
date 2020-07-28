use std::{env, fs, io};
use std::string::String;
use std::path::{Path, PathBuf};
use std::vec::Vec;
use std::time::*;
use show_image::*;

mod kmeans;
use kmeans::*;

/// Main function
fn main() -> Result<(), String>
{
    let mut path = Path::new(".");
    let args: Vec<String> = env::args().collect();

    if args.len() == 2
    {
        path = Path::new(&args[1]);
    }
    else
    {
        println!("Using default directory '{}'.", path.display());
    }

    let mut entries = match fs::read_dir(path)
    {
        Ok(m) => match m
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()
            {
                Ok(m) => m,
                Err(_) =>
                {
                    println!("Error while reading file '{}'.", path.display());
                    return Ok(())
                }
            },
        Err(_) => vec![PathBuf::from(path)]
    };

    entries.sort();

    let window_opt = WindowOptions
    {
        name: "Image".to_string(),
        size: [800, 900],
        resizable: true,
        preserve_aspect_ratio: true
    };
    let show_window = make_window_full(window_opt)?;

    // for each file
    for entry in entries
    {
        // ignoring dirs
        if entry.is_dir() == false
        {
            // get the image object
            let img_ori = image::open(&entry).unwrap().into_rgb();
            let mut img_res = img_ori.clone();

            let imageinfo = ImageInfo::rgb8(
                img_ori.width() as usize,
                img_ori.height() as usize * 2);

            // start chrono
            let start = Instant::now();

            // process the image
            let mut kc = Kmeans::new();
            let iterations = kc.compute_image(&img_ori, &mut img_res);
            let cloud_coverage = kc.get_cloud_coverage();
            
            // stop chrono
            let end = Instant::now();

            // statistics
            println!("{:?} | Cloud coverage: {:>6.2}% | Iterations: {:>2} | Elapsed time: {:?}",
                    entry.file_name().unwrap(), cloud_coverage, iterations, end.duration_since(start));

            // show image
            let (slice_ori, slice_res) = (&img_ori.into_raw(), &img_res.into_raw());
            let show_img: (Vec<u8>, &ImageInfo) = (slice_ori.iter().chain(slice_res).cloned().collect(), &imageinfo);
            show_window.set_image(&show_img, "image")?;

            while let Ok(event) = show_window.wait_key(Duration::new(60, 0))
            {
                if let Some(event) = event
                {
                    if event.key == KeyCode::Escape
                    {
                        return Ok(());
                    }
                    break;
                }
            }
        }
    }
    Ok(())
}
