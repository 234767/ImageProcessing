use image::RgbImage;
use modifications::{get_transformation, Transformation};

mod analysis;
mod modifications;
mod parsing;
mod gpu;

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
    let args = parsing::parse_args();

    if args.is_none() {
        return ();
    }

    let args = args.unwrap();
    println!("{:#?}",args);

    let img = image::io::Reader::open(&args.input_file)
        .unwrap()
        .decode()
        .unwrap()
        .to_rgb8();

    let transformation: Box<dyn Transformation>;

    transformation = match get_transformation(&args) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error: {}", e);
            return ();
        }
    };

    let mut altered_image: RgbImage = img.clone();
    transformation.apply(&mut altered_image);

    let comparer = analysis::get_analyzers(&args);
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

    if let Some(output_path) = args.args.get("-o") {
        let save_result = altered_image.save(output_path);
        match save_result {
            Ok(_) => println!("Saved modified image to {}", output_path),
            Err(error) => eprintln!("Error while saving image: {}", error),
        }
    }
}
