use image::RgbImage;
use modifications::{get_transformation, Transformation};

mod analysis;
mod modifications;
mod parsing;

fn main() {
    let args = parsing::parse_args();

    if args.is_none() {
        return ();
    }

    let args = args.unwrap();

    println!("{:#?}", args);

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
    let comparison_result = comparer.compare(&img, &altered_image);
    match comparison_result {
        Ok(result) => println!("{}", result),
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
