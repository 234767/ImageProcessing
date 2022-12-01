use image::RgbImage;
use crate::gpu::{GPUConfig, InOutImageTransformationPipeline};
use crate::modifications::Transformation;

pub struct LinearFilterGPU {
    config: GPUConfig,
    mask: [f64; 9],
    mask_scale: f64,
}

impl LinearFilterGPU {
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

impl Transformation for LinearFilterGPU {
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

        let mask = self.mask.map(|f| f as f32 * self.mask_scale as f32);

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
