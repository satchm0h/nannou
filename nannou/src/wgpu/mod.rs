//! Items related to wgpu and its integration in nannou!
//!
//! **WebGPU** is the portable graphics specification that nannou targets allowing us to write code
//! that is both fast and allows us to target a wide range of platforms. **wgpu** is the name of
//! the crate we use that implements this specification.
//!
//! This module re-exports the entire `wgpu` crate along with all of its documentation while also
//! adding some additional items that makes `wgpu` easier to use alongside nannou.
//!
//! Useful links:
//!
//! - An awesome [guide for wgpu-rs](https://sotrh.github.io/learn-wgpu/#what-is-wgpu). Highly
//!   recommended reading if you would like to work more closely with the GPU in nannou!
//! - The [wgpu-rs repository](https://github.com/gfx-rs/wgpu-rs).
//! - The [WebGPU specification](https://gpuweb.github.io/gpuweb/).
//! - WebGPU [on wikipedia](https://en.wikipedia.org/wiki/WebGPU).

mod bind_group_builder;
pub mod blend;
mod device_map;
mod render_pass;
mod render_pipeline_builder;
mod sampler_builder;
mod texture;

// Re-export all of `wgpu` along with its documentation.
//
// We do this manually rather than a glob-re-export in order to rename `Texture` to `TextureHandle`
// and have it show up in the documentation properly.
pub use self::bind_group_builder::{
    Builder as BindGroupBuilder, LayoutBuilder as BindGroupLayoutBuilder,
};
pub use self::device_map::{
    ActiveAdapter, AdapterMap, AdapterMapKey, DeviceMap, DeviceMapKey, DeviceQueuePair,
};
pub use self::render_pass::{
    Builder as RenderPassBuilder,
    ColorAttachmentDescriptorBuilder as RenderPassColorAttachmentDescriptorBuilder,
};
pub use self::render_pipeline_builder::RenderPipelineBuilder;
pub use self::sampler_builder::SamplerBuilder;
pub use self::texture::capturer::{
    AwaitWorkerTimeout as TextureCapturerAwaitWorkerTimeout, Capturer as TextureCapturer,
    Snapshot as Textue5cfe74reSnapshot,
};
pub use self::texture::image::format_from_image_color_type as texture_format_from_image_color_type;
pub use self::texture::reshaper::Reshaper as TextureReshaper;
pub use self::texture::row_padded_buffer::{ImageHolder, ImageReadMapping, RowPaddedBuffer};
pub use self::texture::{
    descriptor_eq as texture_descriptor_eq, extent_3d_eq,
    format_size_bytes as texture_format_size_bytes, Builder as TextureBuilder, Texture, TextureId,
    TextureView, TextureViewId, ToTextureView,
};
#[doc(inline)]
pub use wgpu_upstream::{
    util::{self, BufferInitDescriptor},
    vertex_attr_array, Adapter, AdapterInfo, AddressMode, Backend, BackendBit, BindGroup,
    BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, BindingType, BlendFactor, BlendOperation, BlendState,
    Buffer, BufferAddress, BufferAsyncError, BufferBindingType, BufferCopyView, BufferCopyViewBase,
    BufferDescriptor, BufferSize, BufferSlice, BufferUsage, BufferView, BufferViewMut, Color,
    ColorTargetState, ColorWrite, CommandBuffer, CommandBufferDescriptor, CommandEncoder,
    CommandEncoderDescriptor, CompareFunction, ComputePass, ComputePassDescriptor, ComputePipeline,
    ComputePipelineDescriptor, CullMode, DepthBiasState, DepthStencilState, Device,
    DeviceDescriptor, DeviceType, DynamicOffset, Error, Extent3d, Features, FilterMode,
    FragmentState, FrontFace, IndexFormat, InputStepMode, Instance, Label, Limits, LoadOp,
    Maintain, MapMode, MultisampleState, Operations, Origin3d, PipelineLayout,
    PipelineLayoutDescriptor, PipelineStatisticsTypes, PolygonMode, PowerPreference, PresentMode,
    PrimitiveState, PrimitiveTopology, PushConstantRange, QuerySet, QuerySetDescriptor, QueryType,
    Queue, RenderBundle, RenderBundleDescriptor, RenderBundleEncoder,
    RenderBundleEncoderDescriptor, RenderPass, RenderPassColorAttachmentDescriptor,
    RenderPassDepthStencilAttachmentDescriptor, RenderPassDescriptor, RenderPipeline,
    RenderPipelineDescriptor, RequestAdapterOptions, RequestAdapterOptionsBase, RequestDeviceError,
    Sampler, SamplerBorderColor, SamplerDescriptor, ShaderFlags, ShaderLocation, ShaderModule,
    ShaderModuleDescriptor, ShaderSource, ShaderStage, StencilFaceState, StencilOperation,
    StencilState, StorageTextureAccess, Surface, SwapChain, SwapChainDescriptor, SwapChainError,
    SwapChainFrame, SwapChainStatus, SwapChainTexture, Texture as TextureHandle, TextureAspect,
    TextureCopyView, TextureCopyViewBase, TextureDataLayout, TextureDescriptor, TextureDimension,
    TextureFormat, TextureSampleType, TextureUsage, TextureView as TextureViewHandle,
    TextureViewDescriptor, TextureViewDimension, UncapturedErrorHandler, VertexAttribute,
    VertexBufferLayout, VertexFormat, VertexState, BIND_BUFFER_ALIGNMENT, COPY_BUFFER_ALIGNMENT,
    COPY_BYTES_PER_ROW_ALIGNMENT, PUSH_CONSTANT_ALIGNMENT,
};

pub fn shader_from_spirv_bytes(
    device: &wgpu_upstream::Device,
    bytes: &[u8],
) -> wgpu_upstream::ShaderModule {
    let source = util::make_spirv(bytes);
    let desc = ShaderModuleDescriptor {
        label: Some("nannou_shader_module"),
        source,
        flags: ShaderFlags::VALIDATION,
    };
    device.create_shader_module(&desc)
}

/// The default power preference used for requesting the WGPU adapter.
pub const DEFAULT_POWER_PREFERENCE: PowerPreference = PowerPreference::HighPerformance;

/// Nannou's default WGPU backend preferences.
pub const DEFAULT_BACKENDS: BackendBit = BackendBit::PRIMARY;

/// Adds a simple render pass command to the given encoder that simply clears the given texture
/// with the given colour.
///
/// The given `texture` must have `TextureUsage::OUTPUT_ATTACHMENT` enabled.
pub fn clear_texture(
    texture: &TextureViewHandle,
    clear_color: Color,
    encoder: &mut CommandEncoder,
) {
    RenderPassBuilder::new()
        .color_attachment(texture, |builder| {
            builder.load_op(LoadOp::Clear(clear_color))
        })
        .begin(encoder);
}

/// The default device descriptor used to instantiate a logical device when creating windows.
pub fn default_device_descriptor() -> DeviceDescriptor<'static> {
    let features = Features::default();
    let limits = Limits::default();
    DeviceDescriptor {
        label: Some("nannou_device"),
        features,
        limits,
    }
}

/// Adds a simple render pass command to the given encoder that resolves the given multisampled
/// `src_texture` to the given non-multisampled `dst_texture`.
///
/// Both the `src_texture` and `dst_texture` must have:
///
/// - `TextureUsage::OUTPUT_ATTACHMENT` enabled.
/// - The same dimensions.
/// - The same `TextureFormat`.
pub fn resolve_texture(
    src_texture: &TextureViewHandle,
    dst_texture: &TextureViewHandle,
    encoder: &mut CommandEncoder,
) {
    RenderPassBuilder::new()
        .color_attachment(src_texture, |color| {
            color
                .load_op(LoadOp::Load)
                .resolve_target_handle(Some(dst_texture))
        })
        .begin(encoder);
}

/// Shorthand for creating the pipeline layout from a slice of bind group layouts.
pub fn create_pipeline_layout<'p>(
    device: &wgpu_upstream::Device,
    label: Option<&'p str>,
    bind_group_layouts: &[&wgpu_upstream::BindGroupLayout],
    push_constant_ranges: &'p [wgpu_upstream::PushConstantRange],
) -> wgpu_upstream::PipelineLayout {
    let descriptor = wgpu_upstream::PipelineLayoutDescriptor {
        label,
        bind_group_layouts,
        push_constant_ranges,
    };
    device.create_pipeline_layout(&descriptor)
}

/// Whether or not the sampler descriptor describes a sampler that might perform linear filtering.
///
/// This is used to determine the `filtering` field for the sampler binding type variant which
/// assists wgpu with validation.
pub fn sampler_filtering(desc: &SamplerDescriptor) -> bool {
    match (desc.mag_filter, desc.min_filter, desc.mipmap_filter) {
        (FilterMode::Nearest, FilterMode::Nearest, FilterMode::Nearest) => false,
        _ => true,
    }
}

/// The functions within this module use unsafe in order to retrieve their input as a slice of
/// bytes. This is necessary in order to upload data to the GPU via the wgpu
/// `DeviceExt::create_buffer_init` buffer constructor. This method is unsafe as the type `T` may contain
/// padding which is considered to be uninitialised memory in Rust and may potentially lead to
/// undefined behaviour.
///
/// These should be replaced in the future with something similar to `zerocopy`. Unfortunately, we
/// don't gain much benefit from using `zerocopy` in our case as `zerocopy` provides no way to
/// implement the `AsBytes` trait for generic types (e.g. `Vector*`), even with their type
/// parameters filled (e.g. `Vector2<f32>`). This means we can't derive `AsBytes` for the majority
/// of the types where we need to as `derive(AsBytes)` requires that all fields implement
/// `AsBytes`, and neither our `Vector` types or the palette color types can implement it.
///
/// There is a relatively new crate `bytemuck` which provides traits for this, however these traits
/// are `unsafe` and so we don't gain much benefit in terms of safety, especially for our simple
/// use-case. There is a `zeroable` crate that attempts to derive the `Zeroable` trait from
/// `bytemuck`, however:
/// 1. there not yet any other publicly dependent crates or public discussion around the safety of
///    the provided derives and
/// 2. we would still require implementing `Pod` unsafely.
pub mod bytes {
    pub unsafe fn from_slice<T>(slice: &[T]) -> &[u8]
    where
        T: Copy + Sized,
    {
        let len = slice.len() * std::mem::size_of::<T>();
        let ptr = slice.as_ptr() as *const u8;
        std::slice::from_raw_parts(ptr, len)
    }

    pub unsafe fn from<T>(t: &T) -> &[u8]
    where
        T: Copy + Sized,
    {
        let len = std::mem::size_of::<T>();
        let ptr = t as *const T as *const u8;
        std::slice::from_raw_parts(ptr, len)
    }

    /// This is really an astonishingly unsafe function.
    /// Please don't use it.
    pub unsafe fn to_slice<T>(slice: &[u8]) -> &[T]
    where
        T: Copy + Sized,
    {
        let size = std::mem::size_of::<T>();
        let align = std::mem::align_of::<T>();
        assert_eq!(slice.len() % size, 0, "incorrect buffer size");
        assert_eq!(
            slice.as_ptr() as usize % align,
            0,
            "incorrect buffer alignment"
        );
        let len = slice.len() / size;
        let ptr = slice.as_ptr() as *const T;
        std::slice::from_raw_parts(ptr, len)
    }
}
