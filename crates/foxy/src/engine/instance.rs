use std::sync::Arc;

use ash::vk;
use ash_bootstrap as vkb;
use winit::{
  raw_window_handle::{HasDisplayHandle, HasWindowHandle},
  window::Window,
};

#[allow(unused)]
pub struct ApiVersion {
  pub variant: u8,
  pub major: u8,
  pub minor: u8,
  pub patch: u8,
}

#[allow(unused)]
impl ApiVersion {
  pub fn new(raw: u32) -> Self {
    let variant = vk::api_version_variant(raw) as u8;
    let major = vk::api_version_major(raw) as u8;
    let minor = vk::api_version_minor(raw) as u8;
    let patch = vk::api_version_patch(raw) as u8;
    Self {
      variant,
      major,
      minor,
      patch,
    }
  }

  pub fn as_string_short(&self) -> String {
    format!("{}.{}", self.major, self.minor)
  }

  pub fn as_string(&self) -> String {
    format!("{}.{}.{}", self.major, self.minor, self.patch)
  }

  pub fn as_string_full(&self) -> String {
    format!("{}.{}.{}.{}", self.variant, self.major, self.minor, self.patch)
  }
}

pub struct Vulkan {
  pub instance: Arc<vkb::Instance>,
  pub device: Arc<vkb::Device>,
  name: String,
  version: ApiVersion,
}

impl Vulkan {
  pub fn new(window: Arc<Window>) -> Self {
    let builder = vkb::InstanceBuilder::new(Some((window.window_handle().unwrap(), window.display_handle().unwrap())));

    let instance = builder
      .app_name("Vulkan App")
      .request_validation_layers(std::env::var("FOXY_USE_VK_VALIDATION_LAYERS").is_ok())
      .use_default_tracing_messenger()
      .require_api_version(vk::make_api_version(0, 1, 4, 0))
      .build()
      .unwrap();

    let features13 = vk::PhysicalDeviceVulkan13Features {
      dynamic_rendering: 1,
      synchronization2: 1,
      ..Default::default()
    };

    let features12 = vk::PhysicalDeviceVulkan12Features {
      buffer_device_address: 1,
      descriptor_indexing: 1,
      ..Default::default()
    };

    let selector = vkb::PhysicalDeviceSelector::new(instance.clone());

    let gpu = selector
      .preferred_device_type(vkb::PreferredDeviceType::Discrete)
      .add_required_extension_feature(features13)
      .add_required_extension_feature(features12)
      .select()
      .unwrap();

    let builder = vkb::DeviceBuilder::new(gpu, instance.clone());

    let device = Arc::new(builder.build().unwrap());

    let properties = unsafe {
      instance
        .as_ref()
        .as_ref()
        .get_physical_device_properties(*device.physical_device().as_ref())
    };

    let name = properties
      .clone()
      .device_name_as_c_str()
      .unwrap_or(c"Invalid Device Name")
      .to_string_lossy()
      .to_string();

    let version = ApiVersion::new(properties.api_version);

    Self {
      instance,
      device,
      name,
      version,
    }
  }

  pub fn device_name(&self) -> &str {
    &self.name
  }

  pub fn api_version(&self) -> &ApiVersion {
    &self.version
  }

  pub fn destroy(&self) {
    self.device.destroy();
    self.instance.destroy();
  }
}
