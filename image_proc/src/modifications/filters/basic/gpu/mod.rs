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


mod median_filter;
mod gmean_filter;
mod max_filter;

pub use median_filter::MedianFilterGPU;
pub use gmean_filter::GMeanFilterGPU;
pub use max_filter::MaxFilterGPU;