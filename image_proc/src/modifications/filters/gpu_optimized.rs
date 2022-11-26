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
    impl_try_new!();
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

pub struct LowPassFilterGPU {
    config: GPUConfig,
    mask: [f64; 9],
    mask_scale: f64,
}

impl LowPassFilterGPU {
    pub fn try_new(mask: [f64; 9], mask_scale: Option<f64>) -> Result<Self, String> {
        if let Some(config) = GPUConfig::new() {
            Ok(Self {
                config,
                mask,
                mask_scale: mask_scale.unwrap_or(1.0),
            })
        } else {
            Err(String::from(
                "Vulkan required for running GPU optimized version",
            ))
        }
    }
}

impl Transformation for LowPassFilterGPU {
    fn apply(&self, image: &mut RgbImage) {
        mod cs {
            vulkano_shaders::shader! {
                ty: "compute",
                path: "src/shaders/lowpass.glsl",
                types_meta: {
                    use bytemuck::{Pod,Zeroable};

                    #[derive(Clone, Copy, Zeroable, Pod)]
                }
            }
        }

        let mut mask:[f64;9] = self.mask;
        for x in &mut mask {
            *x /= 9.0;
        }

        println!("{:?}", mask);

        let push_constants = cs::ty::PushConstantData { mask };

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
