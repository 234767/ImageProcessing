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
