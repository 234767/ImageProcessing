use super::super::Transformation;
use crate::gpu::{GPUConfig, InOutImageTransformationPipeline};
use image::RgbImage;

macro_rules! impl_try_new {
    () => {
        pub fn try_new(width: u32, height: u32) -> Result<Self, String> {
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
    };
}

pub struct MedianFilterGPU {
    x_radius: u32,
    y_radius: u32,
    config: GPUConfig,
}

impl MedianFilterGPU {
    pub fn try_new(width: u32, height: u32) -> Result<Self, String> {
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
            [image.width() / 16 + 1, image.height() / 16 + 1, 1],
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
    impl_try_new!();
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
            [image.width() / 16 + 1, image.height() / 16 + 1, 1],
            Some(push_constants),
        );

        let result_image = pipeline.dispatch();
        *image = result_image;
    }
}

pub struct MaxFilterGPU {
    x_radius: u32,
    y_radius: u32,
    config: GPUConfig,
}

impl MaxFilterGPU {
    impl_try_new!();
}

impl Transformation for MaxFilterGPU {
    fn apply(&self, image: &mut RgbImage) {
        mod cs {
            vulkano_shaders::shader! {
                ty: "compute",
                path: "src/shaders/max_filter.glsl",
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
            [image.width() / 16 + 1, image.height() / 16 + 1, 1],
            Some(push_constants),
        );

        let result_image = pipeline.dispatch();
        *image = result_image;
    }
}
