use image::{DynamicImage, ImageBuffer, Rgb, RgbImage, Rgba, RgbaImage};
use std::sync::Arc;
use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
use vulkano::command_buffer::{
    AutoCommandBufferBuilder, CommandBufferUsage, CopyBufferToImageInfo, CopyImageToBufferInfo,
};
use vulkano::descriptor_set::{PersistentDescriptorSet, WriteDescriptorSet};

use vulkano::device::{Device, DeviceCreateInfo, DeviceCreationError, Queue, QueueCreateInfo};
use vulkano::instance::{Instance, InstanceCreateInfo};
use vulkano::pipeline::{ComputePipeline, Pipeline, PipelineBindPoint};
use vulkano::shader::ShaderModule;

use vulkano::device::physical::PhysicalDevice;
use vulkano::format::Format;
use vulkano::image::{view::ImageView, ImageDimensions, StorageImage};
use vulkano::sync::GpuFuture;
use vulkano::{sync, VulkanError, VulkanLibrary};

pub struct GPUComputeRunner {
    device: Arc<Device>,
    queue: Arc<Queue>,
}

impl GPUComputeRunner {
    fn new() -> Option<Self> {
        let library = VulkanLibrary::new().expect("No local vulkan library");
        let instance = match Instance::new(
            library,
            InstanceCreateInfo {
                enumerate_portability: true,
                ..Default::default()
            },
        ) {
            Ok(i) => i,
            Err(e) => return None,
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

pub fn RgbImageFromBuffer( width: u32, height: u32, buffer: Arc<CpuAccessibleBuffer<[u8]>>) -> RgbImage {
    let buffer_content = Vec::from(&buffer.read().unwrap()[..]);
    DynamicImage::from(
        ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, buffer_content)
            .unwrap(),
    )
    .to_rgb8()
}
