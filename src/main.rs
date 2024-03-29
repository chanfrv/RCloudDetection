use show_image::{ImageView, ImageInfo, WindowOptions, event, create_window};
use std::path::{Path, PathBuf};
use std::string::String;
use std::time::*;
use std::vec::Vec;
use std::{env, fs, io};

mod kmeans;
use kmeans::kmeans::*;

const PATH_DEFAULT: &str = ".";
const CLASS_NUM_DEFAULT: usize = 5;
const KMEANS_COLORS_DEFAULT: KmeansColor = KmeansColor::Rgb;

fn parse_args(args: Vec<String>) -> (String, usize, KmeansColor) {

    let parse_class = |c: &str| {
        match c.parse::<usize>() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("Expected class num on argument 2");
                CLASS_NUM_DEFAULT
            }
        }
    };
    let parse_colors = |c: &str| {
        match &c[..] {
            "Rgb" => KmeansColor::Rgb,
            "Grayscale" => KmeansColor::Grayscale,
            _ => {
                eprintln!("Expected Rgb or Grayscale on argument 3");
                KMEANS_COLORS_DEFAULT
            }
        }
    };

    match args.len() {
        1 => (String::from(PATH_DEFAULT), CLASS_NUM_DEFAULT, KMEANS_COLORS_DEFAULT),
        2 => (String::from(&args[1]), CLASS_NUM_DEFAULT, KMEANS_COLORS_DEFAULT),
        3 => (String::from(&args[1]), parse_class(&args[2]), KMEANS_COLORS_DEFAULT),
        _ => (String::from(&args[1]), parse_class(&args[2]), parse_colors(&args[3])),
    }
}

/// Main function
#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let (path_str, classes, colors) = parse_args(args);

    let path = Path::new(&path_str);

    let mut entries = match fs::read_dir(path) {
        Ok(m) => match m
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()
        {
            Ok(m) => m,
            Err(_) => {
                eprintln!("Error while reading file '{}'.", path.display());
                return Ok(());
            }
        },
        Err(_) => vec![PathBuf::from(path)],
    };

    entries.sort();

    let window_opt = WindowOptions::new()
        .set_size([1000, 1600])
        .set_resizable(true)
        .set_preserve_aspect_ratio(true);
    let show_window = create_window("image", window_opt)?;

    // for each file
    for entry in entries {
        // ignoring dirs
        if entry.is_dir() == false {
            // get the image object
            let img_ori = image::open(&entry).unwrap().into_rgb8();
            let mut img_res = img_ori.clone();

            let imageinfo =
                ImageInfo::rgb8(img_ori.width(), img_ori.height()* 2);

            // start chrono
            let start = Instant::now();

            // process the image
            let mut kc = Kmeans::new(colors.clone());
            let (iterations, cloud_coverage) = kc.compute_image(&img_ori, &mut img_res, classes);

            // stop chrono
            let end = Instant::now();

            // statistics
            println!(
                "{:?} | Cloud coverage: {:>6.2}% | Iterations: {:>2} | Elapsed time: {:?}",
                entry.file_name().unwrap(),
                cloud_coverage,
                iterations,
                end.duration_since(start)
            );

            // show image
            let (slice_ori, slice_res) = (&img_ori.into_raw(), &img_res.into_raw());
            let chain = slice_ori.iter().chain(slice_res).cloned().collect::<Vec<_>>();
            let show_img = ImageView::new(
                imageinfo,
                &chain,
            );
            show_window.set_image(entry.display().to_string(), show_img)?;

            for event in show_window.event_channel()? {
                if let event::WindowEvent::KeyboardInput(event) = event {
                    if event.input.key_code == Some(event::VirtualKeyCode::Return) && event.input.state.is_pressed() {
                        break;
                    }
                }
            }
        }
    }
    Ok(())
}
