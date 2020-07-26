use std::{env, fs, io};
use std::string::String;
use std::path::{Path, PathBuf};
use std::vec::Vec;
use std::time::*;

mod kmeans;
use kmeans::*;

/// Main function
fn main()
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
                Err(_) => { println!("Error while reading file '{}'.", path.display()); return }
            },
        Err(_) => vec![PathBuf::from(path)]
    };

    entries.sort();

    for entry in entries
    {
        let ref path = &entry.display();

        let img_ori = image::open(&entry).unwrap().into_rgb();
        let mut img_res = img_ori.clone();

        let start = Instant::now();

        let mut kc = Kmeans::init();
        let iterations = kc.compute_image(&img_ori, &mut img_res);
        let cloud_coverage = kc.get_cloud_coverage();

        let end = Instant::now();

        println!("{} | Cloud coverage: {:>6.2}% | Iterations: {:>2} | Elapsed time: {:?}",
                path, cloud_coverage, iterations, end.duration_since(start));
    }
}
