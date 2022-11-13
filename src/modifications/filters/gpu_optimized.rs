use crate::gpu::{GPUConfig, InOutImageTransformationPipeline};
use crate::modifications::Transformation;
use crate::parsing::Args;
use image::RgbImage;
use num::Integer;

pub struct MedianFilterGPU {
    x_radius: u32,
    y_radius: u32,
    config: GPUConfig,
}

impl MedianFilterGPU {
    pub fn try_new(args: &Args) -> Result<Self, String> {
        let mut width: u32 = args.try_get_arg("-w")?;
        if width.is_odd() {
            width -= 1
        }
        let mut height: u32 = args.try_get_arg("-h")?;
        if height.is_odd() {
            height -= 1
        }
        if height * width > 400 {
            return Err(format!(
                "Values of height and width too large. Maximum sampling area is 400, got {}.",
                height * width
            ));
        }
        if let Some(config) = GPUConfig::new() {
            Ok(Self {
                x_radius: width / 2,
                y_radius: height / 2,
                config,
            })
        } else {
            Err(String::from(
                "Vulkan required for running GPU optimized version",
            ))
        }
    }
}

impl Transformation for MedianFilterGPU {
    fn apply(&self, image: &mut RgbImage) {
        mod cs {
            vulkano_shaders::shader! {
                ty: "compute",
                path: "src/shaders/median_filter.glsl",
                types_meta: {
                    use bytemuck::{Pod,Zeroable};

                    #[derive(Clone, Copy, Zeroable, Pod)]
                }
            }
        }

        let push_constants = cs::ty::PushConstantData {
            x_radius: self.x_radius,
            y_radius: self.y_radius,
        };

        let pipeline = InOutImageTransformationPipeline::new(
            self.config.clone(),
            image,
            |device| cs::load(device).expect("Failed to create shader module"),
            [image.width() / 16, image.height() / 16, 1],
            Some(push_constants),
        );

        let result_image = pipeline.dispatch();

        *image = result_image;
    }
}

pub struct GMeanFilterGPU {
    x_radius: u32,
    y_radius: u32,
    config: GPUConfig,
}

impl GMeanFilterGPU {
    pub fn try_new(args: &Args) -> Result<Self, String> {
        let mut width: u32 = args.try_get_arg("-w")?;
        if width.is_odd() {
            width -= 1
        }
        let mut height: u32 = args.try_get_arg("-h")?;
        if height.is_odd() {
            height -= 1
        }
        if let Some(config) = GPUConfig::new() {
            Ok(Self {
                x_radius: width / 2,
                y_radius: height / 2,
                config,
            })
        } else {
            Err(String::from(
                "Vulkan required for running GPU optimized version",
            ))
        }
    }
}

impl Transformation for GMeanFilterGPU {
    fn apply(&self, image: &mut RgbImage) {
        mod cs {
            vulkano_shaders::shader! {
                ty: "compute",
                path: "src/shaders/gmean_filter.glsl",
                types_meta: {
                    use bytemuck::{Pod,Zeroable};

                    #[derive(Clone, Copy, Zeroable, Pod)]
                }
            }
        }

        let push_constants = cs::ty::PushConstantData {
            x_radius: self.x_radius,
            y_radius: self.y_radius,
        };

        let pipeline = InOutImageTransformationPipeline::new(
            self.config.clone(),
            image,
            |device| cs::load(device).expect("Failed to create shader module"),
            [image.width() / 16, image.height() / 16, 1],
            Some(push_constants),
        );

        let result_image = pipeline.dispatch();
        *image = result_image;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use image::Rgb;

    macro_rules! gpu_tests {
        ($($name:ident $values:expr,)*) => {
            $(
            #[test]
            fn $name() {
                let (width, height, channel) = $values;

                let mut image = RgbImage::new(16,16);
                // did not work with images smaller than the group size


                let gpu_config = GPUConfig::new().unwrap();
                let filter = MedianFilterGPU {
                    x_radius: width / 2,
                    y_radius: height / 2,
                    config: gpu_config,
                };

                let values: Vec<u8> = (0..width * height).map(|i| (i * i) as u8).collect();
                assert_eq!(width * height, values.len() as u32);
                let median = values[values.len() / 2];

                let mut iter = values.iter();
                for xi in 1..(1 + width) {
                    for yi in 1..(1 + height) {
                        let Rgb(pixel) = image.get_pixel_mut(xi, yi);
                        let luminance = *iter.next().unwrap();
                        pixel[channel] = luminance;
                    }
                }

                filter.apply(&mut image);
                let (target_x, target_y) = ((1+width) / 2, (1+height)/2);
                let Rgb(target_pixel) = image.get_pixel(target_x, target_y);
                assert_eq!(median, target_pixel[channel]);
            }
            )*
        }
    }

    // todo: add the check for minimal image size to try_new function
    // todo: fix error when image size is not multiple of 16

    gpu_tests! {
        median_3x3_gpu_red (3,3,0),
        median_3x3_gpu_green (3,3,1),
        median_3x3_gpu_blue (3,3,2),
        median_5x5_gpu_red (5,5,0),
        median_5x5_gpu_green (5,5,1),
        median_5x5_gpu_blue (5,5,2),
        median_7x7_gpu_red (7,7,0),
        median_7x7_gpu_green (7,7,1),
        median_7x7_gpu_blue (7,7,2),
        median_9x9_gpu_red (9,9,0),
        median_9x9_gpu_green (9,9,1),
        median_9x9_gpu_blue (9,9,2),
    }
}
