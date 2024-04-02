use ash::{ext, khr, vk};

use super::debug::Debug;
use crate::error::RendererError;

pub struct FoxyInstance {
  _debug: Debug,
  instance: ash::Instance,
  _entry: ash::Entry,
}

impl Drop for FoxyInstance {
  fn drop(&mut self) {
    self._debug.delete();
    unsafe { self.instance.destroy_instance(None) };
  }
}

impl FoxyInstance {
  pub fn new() -> Result<Self, RendererError> {
    let entry = ash::Entry::linked();

    let app_name = c"Foxy";

    let layer_names = [c"VK_LAYER_KHRONOS_validation"];
    let layers_names_raw: Vec<*const core::ffi::c_char> =
      layer_names.iter().map(|raw_name| raw_name.as_ptr()).collect();

    let mut extension_names = [khr::surface::NAME.as_ptr(), khr::win32_surface::NAME.as_ptr()].to_vec();
    extension_names.push(ext::debug_utils::NAME.as_ptr());

    let supported_version = match unsafe { entry.try_enumerate_instance_version() }? {
      // Vulkan 1.1+
      Some(version) => version,
      // Vulkan 1.0
      None => vk::make_api_version(0, 1, 0, 0),
    };

    let major = vk::api_version_major(supported_version);
    let minor = vk::api_version_minor(supported_version);
    let patch = vk::api_version_patch(supported_version);
    let variant = vk::api_version_variant(supported_version);
    tracing::info!("This system can support Vulkan: {major}.{minor}.{patch}.{variant}");

    let selected_version = vk::make_api_version(0, major, minor, 0);

    let major = vk::api_version_major(selected_version);
    let minor = vk::api_version_minor(selected_version);
    let patch = vk::api_version_patch(selected_version);
    let variant = vk::api_version_variant(selected_version);
    tracing::info!("Requesting minimum Vulkan: {major}.{minor}.{patch}.{variant}");

    let appinfo = vk::ApplicationInfo::default()
      .application_name(app_name)
      .application_version(0)
      .engine_name(app_name)
      .engine_version(0)
      .api_version(selected_version);

    let create_info = vk::InstanceCreateInfo::default()
      .application_info(&appinfo)
      .enabled_layer_names(&layers_names_raw)
      .enabled_extension_names(&extension_names);

    let instance: ash::Instance = unsafe { entry.create_instance(&create_info, None)? };

    let _debug = Debug::new(&entry, &instance)?;

    Ok(Self {
      _debug,
      instance,
      _entry: entry,
    })
  }

  pub fn entry(&self) -> &ash::Entry {
    &self._entry
  }

  pub fn instance(&self) -> &ash::Instance {
    &self.instance
  }
}
