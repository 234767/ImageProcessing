use crate::parsing::Args;
use std::collections::HashMap;

use image_proc::analysis::*;

pub fn get_analyzers(args: &Args) -> Box<dyn ImageComparer> {
    let _args: &HashMap<String, String> = &args.args;
    let mut composite = CompositeAnalyzer::new();

    macro_rules! add_if_contains {
        ($key:literal,$object:expr) => {
            if (_args.contains_key($key)) {
                composite.analyzers.push(Box::new($object));
            }
        };
    }

    add_if_contains!("--mse", MeanSquareError {});
    add_if_contains!("--pmse", PMSE {});
    add_if_contains!("--snr", SNR {});
    add_if_contains!("--psnr", PSNR {});
    add_if_contains!("--md", MaximumDifference {});

    return Box::new(composite);
}
