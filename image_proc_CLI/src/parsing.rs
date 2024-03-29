use num::Num;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
pub struct Args {
    pub command: String,
    pub args: HashMap<String, String>,
    pub input_file: String,
}

impl Args {
    /// Tries to get a value for a specified argument, cast to type parameter `T`
    ///
    /// ## Returns
    /// Result<T, String>
    /// * Ok - when value for specified argument is found and successfully parsed as a positive value
    /// * Err - when value was not found, or it was not a positive number
    pub fn try_get_num_arg<T: FromStr + Num + PartialOrd>(
        &self,
        arg_name: &str,
    ) -> Result<T, String> {
        let value_text = self.try_get_arg(arg_name)?;
        match value_text.parse::<T>() {
            Ok(value) if value >= T::zero() => Ok(value),
            _ => Err(format!("Value {} is not a positive number", value_text)),
        }
    }

    pub fn try_get_arg(&self, arg_name: &str) -> Result<String, String> {
        let search_value: String = match arg_name.starts_with("-") {
            true => arg_name.to_string(),
            false => format!("-{}", arg_name),
        };
        let value_text_option = self.args.get(&search_value);
        match value_text_option {
            Some(value) => Ok(value.clone()),
            None => Err(format!("Missing {} argument", arg_name)),
        }
    }
}

pub fn parse_args() -> Option<Args> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let command = match args.get(0) {
        Some(s) => s,
        None => {
            eprintln!("Error:\tNot enough arguments");
            eprintln!("{}", ERROR_MESSAGE);
            return None;
        }
    };

    if command == "--help" || command == "-h" {
        println!("{}", HELP_MESSAGE);
        return None;
    }

    if !command.starts_with("--") {
        eprintln!("Invalid command format. Command must start with --.");
        eprintln!("{}", ERROR_MESSAGE);
        return None;
    }

    let mut map = HashMap::new();
    for arg in args.iter().skip(1).filter(|s| s.starts_with("-")) {
        let equal_sign_index = arg.find('=');
        if let Some(index) = equal_sign_index {
            let (name, value) = arg.split_at(index);
            map.insert(name.to_string(), (&value[1..]).to_string());
        } else {
            map.insert(arg.to_string(), "true".to_string());
        }
    }

    let input_file = args.last().unwrap().clone();
    if input_file.starts_with("-") {
        eprintln!("Error: No input file specified");
        eprintln!("{}", ERROR_MESSAGE);
        return None;
    }

    Some(Args {
        command: command.to_string(),
        args: map,
        input_file,
    })
}

pub const ERROR_MESSAGE: &str = "\
Usage:\ttask1 <COMMAND> [OPTIONS] <FILE>
\tFor more information run task1 --help";

pub const HELP_MESSAGE: &str = "\
Usage:
\t task1 <COMMAND> [OPTIONS] <FILE>

Commands:
\t -h, --help                    \t Display this message

\t --brightness -amount=<AMOUNT> \t Image brightness modification
\t --contrast -amount=<AMOUNT>   \t Image contrast modification
\t --negative                    \t Image negative

\t --hflip                       \t Horizontal flip
\t --vflip                       \t Vertical flip
\t --dflip                       \t Diagonal flip
\t --shrink -factor=<FACTOR>     \t Shrink image
\t --enlarge -factor=<FACTOR>    \t Enlarge image

\t --median -w=<WIDTH> -h=<HEIGHT> Median filter on CPU
                                    \t  WIDTH: Width of sample region in pixels
                                    \t  HEIGHT: Height of sample region in pixels
\t --median-gpu -w=<WIDTH> -h=<HEIGHT> Median filter on GPU
                                    \t  WIDTH: Width of sample region in pixels
                                    \t  HEIGHT: Height of sample region in pixels
\t --gmean -w=<WIDTH> -h=<HEIGHT>  Geometric mean filter on CPU
                                    \t  WIDTH: Width of sample region in pixels
                                    \t  HEIGHT: Height of sample region in pixels
\t --gmean-gpu -w=<WIDTH> -h=<HEIGHT>  Geometric mean filter on GPU
                                    \t  WIDTH: Width of sample region in pixels
                                    \t  HEIGHT: Height of sample region in pixels

\t --max-gpu                        \t Maximum filter
\t --minimum                        \t Minimum filter

\t --histogram -r, -g, -b, -all     \t Create histogram with: Red, Green, Blue or ALL
\t --lowpass                        \t Lowpass filter on CPU
\t --lowpass-gpu                    \t Lowpass filter on GPU
\t --hraleigh -gmin=<GMIN> -gmax=<GMAX> Rayleigh distribution
                                    \t  GMIN: User input
                                    \t  GMAX: User input
\t --uolis                          \t Uolis filter

Options:
\t -o=<FILE>            \t Save the image after transformation to the specified file

\t --mse                \t Display mean square error
\t --pmse               \t Display peak mean square error
\t --snr                \t Display signal to nose ratio
\t --psnr               \t Display peak signal to noise ratio
\t --md                 \t Display max difference

\t --cmean              \t Mean
\t --cvariance          \t Variance
\t --cstdev             \t Standard deviation
\t --cvarcoi            \t Variation coefficient I
\t --casyco             \t Asymmetry coefficient
\t --casyco2            \t Flattening coefficient
\t --cvarcoii           \t Variation coefficient II
\t --centropy           \t Information source entropy
";
