use image::{DynamicImage, ImageBuffer, RgbImage, Rgba};
use std::sync::Arc;
use vulkano::buffer::{BufferContents, BufferUsage, CpuAccessibleBuffer};
use vulkano::command_buffer::{
    AutoCommandBufferBuilder, CommandBufferUsage, CopyBufferToImageInfo, CopyImageToBufferInfo,
    PrimaryAutoCommandBuffer,
};
use vulkano::descriptor_set::{PersistentDescriptorSet, WriteDescriptorSet};

use vulkano::device::{Device, DeviceCreateInfo, Queue, QueueCreateInfo};
use vulkano::instance::{Instance, InstanceCreateInfo};

use vulkano::format::Format;
use vulkano::image::view::ImageView;
use vulkano::image::{ImageCreationError, ImageDimensions, StorageImage};
use vulkano::pipeline::{ComputePipeline, Pipeline, PipelineBindPoint};
use vulkano::shader::ShaderModule;
use vulkano::sync::GpuFuture;
use vulkano::{sync, VulkanLibrary};

#[derive(Clone)]
pub struct GPUConfig {
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,
}

impl GPUConfig {
    pub fn new() -> Option<Self> {
        let library = VulkanLibrary::new().expect("No local vulkan library");
        let instance = match Instance::new(
            library,
            InstanceCreateInfo {
                enumerate_portability: true,
                ..Default::default()
            },
        ) {
            Ok(i) => i,
            Err(_) => return None,
        };

        let physical = match instance.enumerate_physical_devices() {
            Ok(mut devices) => devices.next()?,
            Err(_) => return None,
        };

        let queue_family_index: u32 = physical
            .queue_family_properties()
            .iter()
            .enumerate()
            .position(|(_, f)| f.queue_flags.compute)? as u32;

        let (device, mut queues) = match Device::new(
            physical.clone(),
            DeviceCreateInfo {
                queue_create_infos: vec![QueueCreateInfo {
                    queue_family_index,
                    ..Default::default()
                }],
                ..Default::default()
            },
        ) {
            Ok(result) => result,
            Err(_) => return None,
        };

        let queue = queues.next()?;
        Some(Self { device, queue })
    }
}

pub struct InOutImageTransformationPipeline {
    device: Arc<Device>,
    queue: Arc<Queue>,
    image_width: u32,
    image_height: u32,
    out_buffer: Arc<CpuAccessibleBuffer<[u8]>>,
    command_buffer: PrimaryAutoCommandBuffer,
}

impl InOutImageTransformationPipeline {
    pub fn new<Sb, Pc>(
        config: GPUConfig,
        src_image: &RgbImage,
        shader_builder: Sb,
        group_counts: [u32; 3],
        push_constants: Option<Pc>,
    ) -> InOutImageTransformationPipeline
    where
        Sb: Fn(Arc<Device>) -> Arc<ShaderModule>,
        Pc: BufferContents,
    {
        let out_image = create_image(
            config.device.clone(),
            &*config.queue,
            src_image.width(),
            src_image.height(),
        )
        .unwrap();

        let out_image_view = ImageView::new_default(out_image.clone()).unwrap();

        let in_image = create_image(
            config.device.clone(),
            &*config.queue,
            src_image.width(),
            src_image.height(),
        )
        .unwrap();

        let in_image_view = ImageView::new_default(in_image.clone()).unwrap();

        let shader = shader_builder(config.device.clone());

        let compute_pipeline = ComputePipeline::new(
            config.device.clone(),
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

        let in_buffer: Arc<CpuAccessibleBuffer<[u8]>> = {
            let input_rgba: Vec<u8> = DynamicImage::from(src_image.clone())
                .to_rgba8()
                .as_raw()
                .clone();

            CpuAccessibleBuffer::from_iter(
                config.device.clone(),
                BufferUsage {
                    transfer_src: true,
                    ..Default::default()
                },
                false,
                input_rgba,
            )
            .unwrap()
        };

        let out_buffer = CpuAccessibleBuffer::from_iter(
            config.device.clone(),
            BufferUsage {
                transfer_dst: true,
                ..Default::default()
            },
            false,
            (0..src_image.width() * src_image.height() * 4).map(|_| 0u8),
        )
        .unwrap();

        let mut builder = AutoCommandBufferBuilder::primary(
            config.device.clone(),
            config.queue.queue_family_index(),
            CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap();

        builder
            .copy_buffer_to_image(CopyBufferToImageInfo::buffer_image(
                in_buffer.clone(),
                in_image.clone(),
            ))
            .unwrap()
            .bind_pipeline_compute(compute_pipeline.clone())
            .bind_descriptor_sets(
                PipelineBindPoint::Compute,
                compute_pipeline.layout().clone(),
                0,
                set,
            );

        if let Some(push_constants) = push_constants {
            builder.push_constants(compute_pipeline.layout().clone(), 0, push_constants);
        }

        builder
            .dispatch(group_counts)
            .unwrap()
            .copy_image_to_buffer(CopyImageToBufferInfo::image_buffer(
                out_image.clone(),
                out_buffer.clone(),
            ))
            .unwrap();

        let command_buffer = builder.build().unwrap();

        Self {
            device: config.device,
            queue: config.queue,
            image_width: src_image.width(),
            image_height: src_image.height(),
            out_buffer,
            command_buffer,
        }
    }

    pub(crate) fn dispatch(self) -> RgbImage {
        let future = sync::now(self.device.clone())
            .then_execute(self.queue.clone(), self.command_buffer)
            .unwrap()
            .then_signal_fence_and_flush()
            .unwrap();

        future.wait(None).unwrap();

        rgb_image_from_buffer(self.image_width, self.image_height, self.out_buffer)
    }
}

pub fn rgb_image_from_buffer(
    width: u32,
    height: u32,
    buffer: Arc<CpuAccessibleBuffer<[u8]>>,
) -> RgbImage {
    let buffer_content = Vec::from(&buffer.read().unwrap()[..]);
    DynamicImage::from(ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, buffer_content).unwrap())
        .to_rgb8()
}

pub fn create_image(
    device: Arc<Device>,
    queue: &Queue,
    width: u32,
    height: u32,
) -> Result<Arc<StorageImage>, ImageCreationError> {
    StorageImage::new(
        device.clone(),
        ImageDimensions::Dim2d {
            width,
            height,
            array_layers: 1,
        },
        Format::R8G8B8A8_UNORM,
        Some(queue.queue_family_index()),
    )
}
