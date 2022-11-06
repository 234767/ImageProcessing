use image::{DynamicImage, ImageBuffer, RgbImage, Rgba};
use std::sync::Arc;
use vulkano::buffer::CpuAccessibleBuffer;

use vulkano::device::{Device, DeviceCreateInfo, Queue, QueueCreateInfo};
use vulkano::instance::{Instance, InstanceCreateInfo};

use vulkano::format::Format;
use vulkano::image::{ImageDimensions, StorageImage, ImageCreationError};
use vulkano::{VulkanLibrary};

pub struct GPUComputeRunner {
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,
}

impl GPUComputeRunner {
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

pub fn rgb_image_from_buffer(width: u32, height: u32, buffer: Arc<CpuAccessibleBuffer<[u8]>>) -> RgbImage {
    let buffer_content = Vec::from(&buffer.read().unwrap()[..]);
    DynamicImage::from(
        ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, buffer_content)
            .unwrap(),
    )
    .to_rgb8()
}

pub fn create_image(device: Arc<Device>, queue: &Queue, width: u32, height: u32) -> Result<Arc<StorageImage>, ImageCreationError>{
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
