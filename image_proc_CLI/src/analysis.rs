use crate::parsing::Args;
use std::collections::HashMap;

use image_proc::analysis::*;
use image_proc::analysis::characteristics::{Characteristic, CompositeCharacteristic};
use image_proc::analysis::characteristics::{Mean, StandardDeviation, Variance, VarianceCoefficient1, VarianceCoefficient2, AsymmetryCoefficient, FlatteningCoefficient, InformationSourceEntropy};

pub fn get_comparers(args: &Args) -> Box<dyn ImageComparer> {
    let _args: &HashMap<String, String> = &args.args;
    let mut composite = CompositeComparer::new();

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

pub fn get_characteristics(args: &Args) -> Box<dyn Characteristic> {
    let _args: &HashMap<String, String> = &args.args;
    let mut characteristic = CompositeCharacteristic::new();

    macro_rules! add_if_contains {
        ($key:literal,$object:expr) => {
            if (_args.contains_key($key)) {
                characteristic.push(Box::new($object));
            }
        };
    }
    add_if_contains!("--cmean", Mean {});
    add_if_contains!("--cvariance", Variance {});
    add_if_contains!("--cstdev", StandardDeviation {});
    add_if_contains!("--cvarcoi", VarianceCoefficient1 {});
    add_if_contains!("--casyco", AsymmetryCoefficient {});
    add_if_contains!("--casyco2", FlatteningCoefficient {});
    add_if_contains!("--cvarcoii", VarianceCoefficient2 {});
    add_if_contains!("--centropy ", InformationSourceEntropy {});

    return Box::new(characteristic);
}
