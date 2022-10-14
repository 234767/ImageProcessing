use modifications::{Transformation,get_transformation};

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

    let mut img = image::io::Reader::open(&args.input_file)
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
        },
    };

    transformation.apply(&mut img);

    if let Some(output_path) = args.args.get("-o") {
        _ = img.save(output_path);
    }
}
