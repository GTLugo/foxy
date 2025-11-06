use ash::vk;

//> init_cmd
pub fn command_pool_create_info<'a>(
  queue_family_index: u32,
  flags: vk::CommandPoolCreateFlags,
) -> vk::CommandPoolCreateInfo<'a> {
  vk::CommandPoolCreateInfo::default()
    .queue_family_index(queue_family_index)
    .flags(flags)
}

pub fn command_buffer_allocate_info<'a>(pool: vk::CommandPool, count: u32) -> vk::CommandBufferAllocateInfo<'a> {
  vk::CommandBufferAllocateInfo::default()
    .command_pool(pool)
    .command_buffer_count(count)
    .level(vk::CommandBufferLevel::PRIMARY)
}
//< init_cmd
//
//> init_cmd_draw
pub fn command_buffer_begin_info<'a>(flags: vk::CommandBufferUsageFlags) -> vk::CommandBufferBeginInfo<'a> {
  vk::CommandBufferBeginInfo::default().flags(flags)
}
//< init_cmd_draw

//> init_sync
pub fn fence_create_info<'a>(flags: vk::FenceCreateFlags) -> vk::FenceCreateInfo<'a> {
  vk::FenceCreateInfo::default().flags(flags)
}

pub fn semaphore_create_info<'a>(flags: vk::SemaphoreCreateFlags) -> vk::SemaphoreCreateInfo<'a> {
  vk::SemaphoreCreateInfo::default().flags(flags)
}
//< init_sync

//> init_submit
pub fn semaphore_submit_info<'a>(
  stage_mask: vk::PipelineStageFlags2,
  semaphore: vk::Semaphore,
) -> vk::SemaphoreSubmitInfo<'a> {
  vk::SemaphoreSubmitInfo::default()
    .semaphore(semaphore)
    .stage_mask(stage_mask)
    .device_index(0)
    .value(1)
}

pub fn command_buffer_submit_info<'a>(cmd: vk::CommandBuffer) -> vk::CommandBufferSubmitInfo<'a> {
  vk::CommandBufferSubmitInfo::default()
    .command_buffer(cmd)
    .device_mask(0)
}

pub fn submit_info<'a>(
  cmd: &'a vk::CommandBufferSubmitInfo,
  signal_semaphore_info: Option<&'a vk::SemaphoreSubmitInfo>,
  wait_semaphore_info: Option<&'a vk::SemaphoreSubmitInfo>,
) -> vk::SubmitInfo2<'a> {
  let mut default = vk::SubmitInfo2::default().command_buffer_infos(std::slice::from_ref(cmd));

  if let Some(wait_info) = wait_semaphore_info {
    default = default.wait_semaphore_infos(std::slice::from_ref(wait_info));
  }

  if let Some(signal_info) = signal_semaphore_info {
    default = default.signal_semaphore_infos(std::slice::from_ref(signal_info));
  }

  default
}
//< init_submit

pub fn present_info() -> vk::PresentInfoKHR<'static> {
  vk::PresentInfoKHR::default()
}

//> color_info
pub fn attachment_info<'a>(
  view: vk::ImageView,
  clear: Option<&'a vk::ClearValue>,
  layout: vk::ImageLayout,
) -> vk::RenderingAttachmentInfo<'a> {
  let load_op = if clear.is_some() {
    vk::AttachmentLoadOp::CLEAR
  } else {
    vk::AttachmentLoadOp::LOAD
  };

  let mut default = vk::RenderingAttachmentInfo::default()
    .image_view(view)
    .image_layout(layout)
    .load_op(load_op)
    .store_op(vk::AttachmentStoreOp::STORE);

  if let Some(clear_value) = clear {
    default = default.clear_value(*clear_value);
  }

  default
}
//< color_info
//> depth_info
pub fn depth_attachment_info<'a>(view: vk::ImageView, layout: vk::ImageLayout) -> vk::RenderingAttachmentInfo<'a> {
  vk::RenderingAttachmentInfo::default()
    .image_view(view)
    .image_layout(layout)
    .load_op(vk::AttachmentLoadOp::CLEAR)
    .store_op(vk::AttachmentStoreOp::STORE)
    .clear_value(vk::ClearValue {
      depth_stencil: vk::ClearDepthStencilValue { depth: 0.0, stencil: 0 },
    })
}
//< depth_info
//> render_info
pub fn rendering_info<'a>(
  render_extent: vk::Extent2D,
  color_attachment: &'a vk::RenderingAttachmentInfo,
  depth_attachment: Option<&'a vk::RenderingAttachmentInfo>,
) -> vk::RenderingInfo<'a> {
  let mut default = vk::RenderingInfo::default()
    .render_area(vk::Rect2D {
      offset: vk::Offset2D { x: 0, y: 0 },
      extent: render_extent,
    })
    .layer_count(1)
    .color_attachments(std::slice::from_ref(color_attachment));

  if let Some(depth) = depth_attachment {
    default = default.depth_attachment(depth);
  }

  default
}
//< render_info
//> subresource
pub fn image_subresource_range(aspect_mask: vk::ImageAspectFlags) -> vk::ImageSubresourceRange {
  vk::ImageSubresourceRange::default()
    .aspect_mask(aspect_mask)
    .base_mip_level(0)
    .level_count(vk::REMAINING_MIP_LEVELS)
    .base_array_layer(0)
    .layer_count(vk::REMAINING_ARRAY_LAYERS)
}
//< subresource

pub fn descriptorset_layout_binding<'a>(
  ty: vk::DescriptorType,
  stage_flags: vk::ShaderStageFlags,
  binding: u32,
) -> vk::DescriptorSetLayoutBinding<'a> {
  vk::DescriptorSetLayoutBinding::default()
    .binding(binding)
    .descriptor_count(1)
    .descriptor_type(ty)
    .stage_flags(stage_flags)
}

pub fn descriptorset_layout_create_info<'a>(
  bindings: &'a [vk::DescriptorSetLayoutBinding],
) -> vk::DescriptorSetLayoutCreateInfo<'a> {
  vk::DescriptorSetLayoutCreateInfo::default().bindings(bindings)
}

pub fn write_descriptor_image<'a>(
  ty: vk::DescriptorType,
  dst_set: vk::DescriptorSet,
  image_info: &'a vk::DescriptorImageInfo,
  binding: u32,
) -> vk::WriteDescriptorSet<'a> {
  vk::WriteDescriptorSet::default()
    .dst_binding(binding)
    .dst_set(dst_set)
    .descriptor_count(1)
    .descriptor_type(ty)
    .image_info(std::slice::from_ref(image_info))
}

pub fn write_descriptor_buffer<'a>(
  ty: vk::DescriptorType,
  dst_set: vk::DescriptorSet,
  buffer_info: &'a vk::DescriptorBufferInfo,
  binding: u32,
) -> vk::WriteDescriptorSet<'a> {
  vk::WriteDescriptorSet::default()
    .dst_binding(binding)
    .dst_set(dst_set)
    .descriptor_count(1)
    .descriptor_type(ty)
    .buffer_info(std::slice::from_ref(buffer_info))
}

pub fn buffer_info(buffer: vk::Buffer, offset: vk::DeviceSize, range: vk::DeviceSize) -> vk::DescriptorBufferInfo {
  vk::DescriptorBufferInfo::default()
    .buffer(buffer)
    .offset(offset)
    .range(range)
}

//> image_set
pub fn image_create_info<'a>(
  format: vk::Format,
  usage_flags: vk::ImageUsageFlags,
  extent: vk::Extent3D,
) -> vk::ImageCreateInfo<'a> {
  vk::ImageCreateInfo::default()
		.image_type(vk::ImageType::TYPE_2D)
		.format(format)
		.extent(extent)
		.mip_levels(1)
		.array_layers(1)
		//for MSAA. we will not be using it by default, so default it to 1 sample per pixel.
		.samples(vk::SampleCountFlags::TYPE_1)
		//optimal tiling, which means the image is stored on the best gpu format
		.tiling(vk::ImageTiling::OPTIMAL)
		.usage(usage_flags)
}

pub fn imageview_create_info<'a>(
  format: vk::Format,
  image: vk::Image,
  aspect_flags: vk::ImageAspectFlags,
) -> vk::ImageViewCreateInfo<'a> {
  // build a image-view for the depth image to use for rendering
  vk::ImageViewCreateInfo::default()
    .view_type(vk::ImageViewType::TYPE_2D)
    .image(image)
    .format(format)
    .subresource_range(
      vk::ImageSubresourceRange::default()
        .base_mip_level(0)
        .level_count(1)
        .base_array_layer(0)
        .layer_count(1)
        .aspect_mask(aspect_flags),
    )
}
//< image_set
pub fn pipeline_layout_create_info<'a>() -> vk::PipelineLayoutCreateInfo<'a> {
  // empty defaults
  vk::PipelineLayoutCreateInfo::default()
}

pub fn pipeline_shader_stage_create_info<'a>(
  stage: vk::ShaderStageFlags,
  shader_module: vk::ShaderModule,
  entry: &'a std::ffi::CStr,
) -> vk::PipelineShaderStageCreateInfo<'a> {
  vk::PipelineShaderStageCreateInfo::default()
    .stage(stage) // shader stage
		.module(shader_module) // module containing the code for this shader stage
		.name(entry) // the entry point of the shader
}
