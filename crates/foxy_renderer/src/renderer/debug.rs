use ash::{
  ext,
  vk::{self, DebugUtilsMessageSeverityFlagsEXT},
};

use crate::error::RendererError;

pub struct Debug {
  _debug_utils: Option<ext::debug_utils::Instance>,
  _debug_messenger: Option<vk::DebugUtilsMessengerEXT>,
}

impl Debug {
  pub const ENABLE_VALIDATION_LAYERS: bool = cfg!(debug_assertions);

  pub fn delete(&mut self) {
    if let Some(d) = self._debug_utils.take() {
      unsafe { d.destroy_debug_utils_messenger(self._debug_messenger.take().unwrap(), None) };
    }
  }

  pub fn new(entry: &ash::Entry, instance: &ash::Instance) -> Result<Self, RendererError> {
    if Self::ENABLE_VALIDATION_LAYERS {
      let create_info = vk::DebugUtilsMessengerCreateInfoEXT::default()
        .message_severity(
          DebugUtilsMessageSeverityFlagsEXT::ERROR
            | DebugUtilsMessageSeverityFlagsEXT::WARNING
            | DebugUtilsMessageSeverityFlagsEXT::VERBOSE
            | DebugUtilsMessageSeverityFlagsEXT::INFO,
        )
        .message_type(
          vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
            | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE
            | vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
            | vk::DebugUtilsMessageTypeFlagsEXT::DEVICE_ADDRESS_BINDING,
        )
        .pfn_user_callback(Some(debug_callback));
      let debug_utils = ext::debug_utils::Instance::new(entry, instance);

      Ok(
        unsafe { debug_utils.create_debug_utils_messenger(&create_info, None) }.map(|debug_messenger| Self {
          _debug_utils: Some(debug_utils),
          _debug_messenger: Some(debug_messenger),
        })?,
      )
    } else {
      Ok(Self {
        _debug_utils: None,
        _debug_messenger: None,
      })
    }
  }
}

unsafe extern "system" fn debug_callback(
  message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
  message_type: vk::DebugUtilsMessageTypeFlagsEXT,
  callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT<'_>,
  _user_data: *mut core::ffi::c_void,
) -> vk::Bool32 {
  let callback_data = unsafe { callback_data.as_ref() }.unwrap();

  let ty = if message_type.intersects(vk::DebugUtilsMessageTypeFlagsEXT::GENERAL) {
    "General"
  } else if message_type.intersects(vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION) {
    "Validation"
  } else if message_type.intersects(vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE) {
    "Performance"
  } else {
    "Device Address Binding"
  };

  if message_severity.intersects(vk::DebugUtilsMessageSeverityFlagsEXT::ERROR) {
    if let Some(message) = unsafe { callback_data.message_as_c_str() } {
      tracing::error!("Vulkan {ty}: {:?}", message)
    }
  } else if message_severity.intersects(vk::DebugUtilsMessageSeverityFlagsEXT::WARNING) {
    if let Some(message) = unsafe { callback_data.message_as_c_str() } {
      tracing::warn!("Vulkan {ty}: {:?}", message)
    }
  };

  false.into()
}
