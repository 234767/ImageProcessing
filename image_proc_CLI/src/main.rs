extern crate core;

use std::process::exit;
use image::{RgbImage};
use image_proc::modifications::Transformation;

mod analysis;
mod parsing;
mod transformations;

use crate::analysis::get_characteristic;
use analysis::get_comparers;
use transformations::get_transformation;

fn try_get_image(path: &String) -> Option<RgbImage> {
    match image::io::Reader::open(path) {
        Ok(reader) => match reader.decode() {
            Ok(image) => Some(image.to_rgb8()),
            Err(_) => None,
        },
        _ => None,
    }
}

fn main() {
    let args = match parsing::parse_args() {
        Some(args) => args,
        None => {
            eprintln!("Error: no arguments supplied");
            exit(1);
        }
    };

    let img = match try_get_image(&args.input_file) {
        Some(image) => image,
        None => {
            eprintln!("Error: could not find file {}", &args.input_file);
            exit(1);
        }
    };

    let transformation: Box<dyn Transformation> = match get_transformation(&args) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(1);
        }
    };

    let altered_image = {
        let mut image_copy: RgbImage = img.clone();
        transformation.apply(&mut image_copy);
        image_copy
    };

    if args.command != "--histogram" {
        let comparer = get_comparers(&args);
        let comparison_baseline: Option<RgbImage> = match args.args.get("-baseline") {
            Some(path) => try_get_image(path),
            None => None,
        };
        let comparison_result = comparer.compare(
            match &comparison_baseline {
                Some(image) => image,
                None => &img,
            },
            &altered_image,
        );
        match comparison_result {
            Ok(result) => print!("{}", result),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    if args.command != "--histogram" {
        let characteristics = get_characteristic(&args);
        let comparison_result = characteristics.analyze(&altered_image);
        match comparison_result {
            Ok(result) => print!("{}", result),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    if let Some(output_path) = args.args.get("-o") {
        let save_result = altered_image.save(output_path);
        match save_result {
            Ok(_) => println!("Saved modified image to {}", output_path),
            Err(error) => eprintln!("Error while saving image: {}", error),
        }
    }
}
