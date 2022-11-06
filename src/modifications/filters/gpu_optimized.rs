use crate::gpu::{create_image, rgb_image_from_buffer, GPUComputeRunner};
use crate::modifications::Transformation;
use crate::parsing::Args;
use image::{DynamicImage, RgbImage};
use num::Integer;
use std::sync::Arc;
use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
use vulkano::command_buffer::{
    AutoCommandBufferBuilder, CommandBufferUsage, CopyBufferToImageInfo, CopyImageToBufferInfo,
};
use vulkano::descriptor_set::{PersistentDescriptorSet, WriteDescriptorSet};
use vulkano::device::{Device, Queue};
use vulkano::image::view::ImageView;
use vulkano::pipeline::{ComputePipeline, Pipeline, PipelineBindPoint};
use vulkano::shader::ShaderModule;
use vulkano::sync;
use vulkano::sync::GpuFuture;

pub struct MedianFilterGPU {
    x_radius: u32,
    y_radius: u32,
    device: Arc<Device>,
    queue: Arc<Queue>,
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
        if height * width > 500 {
            return Err(format!("Values of height and width too large. Maximum sampling area is 500, got {}.", height*width));
        }
        if let Some(GPUComputeRunner { device, queue }) = GPUComputeRunner::new() {
            Ok(Self {
                x_radius: width / 2,
                y_radius: height / 2,
                device,
                queue,
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
        let out_image = create_image(
            self.device.clone(),
            &*self.queue,
            image.width(),
            image.height(),
        )
        .unwrap();

        let out_image_view = ImageView::new_default(out_image.clone()).unwrap();

        let in_image = create_image(
            self.device.clone(),
            &*self.queue,
            image.width(),
            image.height(),
        )
        .unwrap();

        let in_image_view = ImageView::new_default(in_image.clone()).unwrap();

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

        let shader: Arc<ShaderModule> =
            cs::load(self.device.clone()).expect("Failed to create shader module");

        let compute_pipeline = ComputePipeline::new(
            self.device.clone(),
            shader.entry_point("main").unwrap(),
            &(),
            None,
            |_| {},
        )
        .expect("Failed to create compute pipeline");

        let layout = compute_pipeline.layout().set_layouts().get(0).unwrap();
        let set = PersistentDescriptorSet::new(
            layout.clone(),
            [
                WriteDescriptorSet::image_view(0, in_image_view.clone()),
                WriteDescriptorSet::image_view(1, out_image_view.clone()),
            ],
        )
        .unwrap();

        let out_buf = CpuAccessibleBuffer::from_iter(
            self.device.clone(),
            BufferUsage {
                transfer_dst: true,
                ..Default::default()
            },
            false,
            (0..image.width() * image.height() * 4).map(|_| 0u8),
        )
        .unwrap();

        let input_rgba: Vec<u8> = DynamicImage::from(image.clone())
            .to_rgba8()
            .as_raw()
            .clone();

        let in_buf = CpuAccessibleBuffer::from_iter(
            self.device.clone(),
            BufferUsage {
                transfer_src: true,
                ..Default::default()
            },
            false,
            input_rgba,
        )
        .unwrap();

        let mut builder = AutoCommandBufferBuilder::primary(
            self.device.clone(),
            self.queue.queue_family_index(),
            CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap();

        builder
            .copy_buffer_to_image(CopyBufferToImageInfo::buffer_image(
                in_buf.clone(),
                in_image.clone(),
            ))
            .unwrap()
            .bind_pipeline_compute(compute_pipeline.clone())
            .bind_descriptor_sets(
                PipelineBindPoint::Compute,
                compute_pipeline.layout().clone(),
                0,
                set,
            )
            .push_constants(compute_pipeline.layout().clone(), 0, push_constants)
            .dispatch([image.width() / 16, image.height() / 16, 1])
            .unwrap()
            .copy_image_to_buffer(CopyImageToBufferInfo::image_buffer(
                out_image.clone(),
                out_buf.clone(),
            ))
            .unwrap();

        let command_buffer = builder.build().unwrap();

        let future = sync::now(self.device.clone())
            .then_execute(self.queue.clone(), command_buffer)
            .unwrap()
            .then_signal_fence_and_flush()
            .unwrap();

        future.wait(None).unwrap();

        let result_image = rgb_image_from_buffer(image.width(), image.height(), out_buf);

        *image = result_image;
    }
}

pub struct GMeanFilterGPU {
    x_radius: u32,
    y_radius: u32,
    device: Arc<Device>,
    queue: Arc<Queue>,
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
        if let Some(GPUComputeRunner { device, queue }) = GPUComputeRunner::new() {
            Ok(Self {
                x_radius: width / 2,
                y_radius: height / 2,
                device,
                queue,
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
        let out_image = create_image(
            self.device.clone(),
            &*self.queue,
            image.width(),
            image.height(),
        )
            .unwrap();

        let out_image_view = ImageView::new_default(out_image.clone()).unwrap();

        let in_image = create_image(
            self.device.clone(),
            &*self.queue,
            image.width(),
            image.height(),
        )
            .unwrap();

        let in_image_view = ImageView::new_default(in_image.clone()).unwrap();

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

        let shader: Arc<ShaderModule> =
            cs::load(self.device.clone()).expect("Failed to create shader module");

        let compute_pipeline = ComputePipeline::new(
            self.device.clone(),
            shader.entry_point("main").unwrap(),
            &(),
            None,
            |_| {},
        )
            .expect("Failed to create compute pipeline");

        let layout = compute_pipeline.layout().set_layouts().get(0).unwrap();
        let set = PersistentDescriptorSet::new(
            layout.clone(),
            [
                WriteDescriptorSet::image_view(0, in_image_view.clone()),
                WriteDescriptorSet::image_view(1, out_image_view.clone()),
            ],
        )
            .unwrap();

        let out_buf = CpuAccessibleBuffer::from_iter(
            self.device.clone(),
            BufferUsage {
                transfer_dst: true,
                ..Default::default()
            },
            false,
            (0..image.width() * image.height() * 4).map(|_| 0u8),
        )
            .unwrap();

        let input_rgba: Vec<u8> = DynamicImage::from(image.clone())
            .to_rgba8()
            .as_raw()
            .clone();

        let in_buf = CpuAccessibleBuffer::from_iter(
            self.device.clone(),
            BufferUsage {
                transfer_src: true,
                ..Default::default()
            },
            false,
            input_rgba,
        )
            .unwrap();

        let mut builder = AutoCommandBufferBuilder::primary(
            self.device.clone(),
            self.queue.queue_family_index(),
            CommandBufferUsage::OneTimeSubmit,
        )
            .unwrap();

        builder
            .copy_buffer_to_image(CopyBufferToImageInfo::buffer_image(
                in_buf.clone(),
                in_image.clone(),
            ))
            .unwrap()
            .bind_pipeline_compute(compute_pipeline.clone())
            .bind_descriptor_sets(
                PipelineBindPoint::Compute,
                compute_pipeline.layout().clone(),
                0,
                set,
            )
            .push_constants(compute_pipeline.layout().clone(), 0, push_constants)
            .dispatch([image.width() / 16, image.height() / 16, 1])
            .unwrap()
            .copy_image_to_buffer(CopyImageToBufferInfo::image_buffer(
                out_image.clone(),
                out_buf.clone(),
            ))
            .unwrap();

        let command_buffer = builder.build().unwrap();

        let future = sync::now(self.device.clone())
            .then_execute(self.queue.clone(), command_buffer)
            .unwrap()
            .then_signal_fence_and_flush()
            .unwrap();

        future.wait(None).unwrap();

        let result_image = rgb_image_from_buffer(image.width(), image.height(), out_buf);

        *image = result_image;
    }
}
