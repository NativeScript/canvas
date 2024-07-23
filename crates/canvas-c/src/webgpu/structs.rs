use wgpu_types::{CompositeAlphaMode, PresentMode, SurfaceCapabilities};

use crate::buffers::StringBuffer;

use super::{
    enums::{CanvasGPUTextureFormat, CanvasTextureAspect, CanvasVertexFormat},
    gpu_texture_view::CanvasGPUTextureView,
};

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct CanvasOrigin3d {
    /// X position of the origin
    pub x: u32,
    /// Y position of the origin
    pub y: u32,
    /// Z position of the origin
    pub z: u32,
}

impl From<wgpu_types::Origin3d> for CanvasOrigin3d {
    fn from(value: wgpu_types::Origin3d) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

impl Into<wgpu_types::Origin3d> for CanvasOrigin3d {
    fn into(self) -> wgpu_types::Origin3d {
        wgpu_types::Origin3d {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CanvasOrigin2d {
    ///
    pub x: u32,
    ///
    pub y: u32,
}

impl From<wgpu_types::Origin2d> for CanvasOrigin2d {
    fn from(value: wgpu_types::Origin2d) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl Into<wgpu_types::Origin2d> for CanvasOrigin2d {
    fn into(self) -> wgpu_types::Origin2d {
        wgpu_types::Origin2d {
            x: self.x,
            y: self.y,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct CanvasExtent3d {
    /// Width of the extent
    pub width: u32,
    /// Height of the extent
    pub height: u32,
    /// The depth of the extent or the number of array layers
    pub depth_or_array_layers: u32,
}

impl Into<wgpu_types::Extent3d> for CanvasExtent3d {
    fn into(self) -> wgpu_types::Extent3d {
        wgpu_types::Extent3d {
            width: self.width,
            height: self.height,
            depth_or_array_layers: self.depth_or_array_layers,
        }
    }
}

impl From<wgpu_types::Extent3d> for CanvasExtent3d {
    fn from(value: wgpu_types::Extent3d) -> Self {
        Self {
            width: value.width,
            height: value.height,
            depth_or_array_layers: value.depth_or_array_layers,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CanvasColor {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

impl CanvasColor {
    pub const TRANSPARENT: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    };
    pub const BLACK: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const WHITE: Self = Self {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
    pub const RED: Self = Self {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const GREEN: Self = Self {
        r: 0.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };
    pub const BLUE: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };
}

impl Into<wgpu_types::Color> for CanvasColor {
    fn into(self) -> wgpu_types::Color {
        wgpu_types::Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
        }
    }
}

impl From<wgpu_types::Color> for CanvasColor {
    fn from(value: wgpu_types::Color) -> Self {
        Self {
            r: value.r,
            g: value.g,
            b: value.b,
            a: value.a,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CanvasImageDataLayout {
    offset: u64,
    bytes_per_row: i32,
    rows_per_image: i32,
}

impl From<wgpu_types::ImageDataLayout> for CanvasImageDataLayout {
    fn from(value: wgpu_types::ImageDataLayout) -> Self {
        Self {
            offset: value.offset,
            bytes_per_row: value.bytes_per_row.unwrap_or_default() as i32,
            rows_per_image: value.rows_per_image.unwrap_or_default() as i32,
        }
    }
}

impl Into<wgpu_types::ImageDataLayout> for CanvasImageDataLayout {
    fn into(self) -> wgpu_types::ImageDataLayout {
        let bytes_per_row: Option<u32> = self.bytes_per_row.try_into().ok();
        let rows_per_image: Option<u32> = self.rows_per_image.try_into().ok();

        wgpu_types::ImageDataLayout {
            offset: self.offset,
            bytes_per_row,
            rows_per_image,
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CanvasImageCopyExternalImage {
    /// The texture to be copied from. The copy source data is captured at the moment
    /// the copy is issued.
    pub source: *const u8,
    pub source_size: usize,
    /// The base texel used for copying from the external image. Together
    /// with the `copy_size` argument to copy functions, defines the
    /// sub-region of the image to copy.
    ///
    /// Relative to the top left of the image.
    ///
    /// Must be [`Origin2d::ZERO`] if [`DownlevelFlags::UNRESTRICTED_EXTERNAL_TEXTURE_COPIES`] is not supported.
    pub origin: CanvasOrigin2d,
    /// If the Y coordinate of the image should be flipped. Even if this is
    /// true, `origin` is still relative to the top left.
    pub flip_y: bool,

    pub width: u32,

    pub height: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CanvasImageSubresourceRange {
    pub aspect: CanvasTextureAspect,
    pub base_mip_level: u32,
    pub mip_level_count: i32,
    pub base_array_layer: u32,
    pub array_layer_count: i32,
}

impl From<wgpu_types::ImageSubresourceRange> for CanvasImageSubresourceRange {
    fn from(value: wgpu_types::ImageSubresourceRange) -> Self {
        Self {
            aspect: value.aspect.into(),
            base_mip_level: value.base_mip_level,
            mip_level_count: value.mip_level_count.map(|v| v as i32).unwrap_or(-1),
            base_array_layer: value.base_array_layer,
            array_layer_count: value.array_layer_count.map(|v| v as i32).unwrap_or(-1),
        }
    }
}

impl Into<wgpu_types::ImageSubresourceRange> for CanvasImageSubresourceRange {
    fn into(self) -> wgpu_types::ImageSubresourceRange {
        wgpu_types::ImageSubresourceRange {
            aspect: self.aspect.into(),
            base_mip_level: self.base_mip_level,
            mip_level_count: self.mip_level_count.try_into().ok(),
            base_array_layer: self.base_array_layer,
            array_layer_count: self.array_layer_count.try_into().ok(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CanvasColorTargetState {
    pub format: CanvasGPUTextureFormat,
    pub blend: CanvasOptionalBlendState,
    pub write_mask: u32,
}

impl From<wgpu_types::ColorTargetState> for CanvasColorTargetState {
    fn from(value: wgpu_types::ColorTargetState) -> Self {
        Self {
            format: value.format.into(),
            blend: value.blend.into(),
            write_mask: value.write_mask.bits(),
        }
    }
}

impl Into<wgpu_types::ColorTargetState> for CanvasColorTargetState {
    fn into(self) -> wgpu_types::ColorTargetState {
        wgpu_types::ColorTargetState {
            format: self.format.into(),
            blend: self.blend.into(),
            write_mask: wgpu_types::ColorWrites::from_bits_truncate(self.write_mask),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CanvasOptionalBlendState {
    None,
    Some(CanvasBlendState),
}

impl From<wgpu_types::TextureFormat> for CanvasColorTargetState {
    fn from(format: wgpu_types::TextureFormat) -> Self {
        Self {
            format: format.into(),
            blend: CanvasOptionalBlendState::None,
            write_mask: wgpu_types::ColorWrites::ALL.bits(),
        }
    }
}

impl From<wgpu_types::BlendState> for CanvasOptionalBlendState {
    fn from(value: wgpu_types::BlendState) -> Self {
        Self::Some(CanvasBlendState {
            color: value.color.into(),
            alpha: value.alpha.into(),
        })
    }
}

impl From<Option<wgpu_types::BlendState>> for CanvasOptionalBlendState {
    fn from(value: Option<wgpu_types::BlendState>) -> Self {
        match value {
            Some(value) => Self::Some(value.into()),
            None => Self::None,
        }
    }
}

impl Into<Option<wgpu_types::BlendState>> for CanvasOptionalBlendState {
    fn into(self) -> Option<wgpu_types::BlendState> {
        match self {
            CanvasOptionalBlendState::None => None,
            CanvasOptionalBlendState::Some(value) => Some(wgpu_types::BlendState {
                color: value.color.into(),
                alpha: value.alpha.into(),
            }),
        }
    }
}

impl From<CanvasGPUTextureFormat> for CanvasColorTargetState {
    fn from(format: CanvasGPUTextureFormat) -> Self {
        Self {
            format,
            blend: CanvasOptionalBlendState::None,
            write_mask: wgpu_types::ColorWrites::ALL.bits(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CanvasBlendState {
    /// Color equation.
    pub color: CanvasBlendComponent,
    /// Alpha equation.
    pub alpha: CanvasBlendComponent,
}

impl CanvasBlendState {
    /// Blend mode that does no color blending, just overwrites the output with the contents of the shader.
    pub const REPLACE: Self = Self {
        color: CanvasBlendComponent::REPLACE,
        alpha: CanvasBlendComponent::REPLACE,
    };

    /// Blend mode that does standard alpha blending with non-premultiplied alpha.
    pub const ALPHA_BLENDING: Self = Self {
        color: CanvasBlendComponent {
            src_factor: CanvasBlendFactor::SrcAlpha,
            dst_factor: CanvasBlendFactor::OneMinusSrcAlpha,
            operation: CanvasBlendOperation::Add,
        },
        alpha: CanvasBlendComponent::OVER,
    };

    /// Blend mode that does standard alpha blending with premultiplied alpha.
    pub const PREMULTIPLIED_ALPHA_BLENDING: Self = Self {
        color: CanvasBlendComponent::OVER,
        alpha: CanvasBlendComponent::OVER,
    };
}

impl From<wgpu_types::BlendState> for CanvasBlendState {
    fn from(value: wgpu_types::BlendState) -> Self {
        Self {
            color: value.color.into(),
            alpha: value.alpha.into(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CanvasBlendComponent {
    pub src_factor: CanvasBlendFactor,
    pub dst_factor: CanvasBlendFactor,
    pub operation: CanvasBlendOperation,
}

impl CanvasBlendComponent {
    /// Default blending state that replaces destination with the source.
    pub const REPLACE: Self = Self {
        src_factor: CanvasBlendFactor::One,
        dst_factor: CanvasBlendFactor::Zero,
        operation: CanvasBlendOperation::Add,
    };

    /// Blend state of (1 * src) + ((1 - src_alpha) * dst)
    pub const OVER: Self = Self {
        src_factor: CanvasBlendFactor::One,
        dst_factor: CanvasBlendFactor::OneMinusSrcAlpha,
        operation: CanvasBlendOperation::Add,
    };

    /// Returns true if the state relies on the constant color, which is
    /// set independently on a render command encoder.
    pub fn uses_constant(&self) -> bool {
        match (self.src_factor, self.dst_factor) {
            (CanvasBlendFactor::Constant, _)
            | (CanvasBlendFactor::OneMinusConstant, _)
            | (_, CanvasBlendFactor::Constant)
            | (_, CanvasBlendFactor::OneMinusConstant) => true,
            (_, _) => false,
        }
    }
}

impl Default for CanvasBlendComponent {
    fn default() -> Self {
        Self::REPLACE
    }
}

impl From<wgpu_types::BlendComponent> for CanvasBlendComponent {
    fn from(value: wgpu_types::BlendComponent) -> Self {
        Self {
            src_factor: value.src_factor.into(),
            dst_factor: value.dst_factor.into(),
            operation: value.operation.into(),
        }
    }
}

impl Into<wgpu_types::BlendComponent> for CanvasBlendComponent {
    fn into(self) -> wgpu_types::BlendComponent {
        wgpu_types::BlendComponent {
            src_factor: self.src_factor.into(),
            dst_factor: self.dst_factor.into(),
            operation: self.operation.into(),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum CanvasBlendFactor {
    /// 0.0
    Zero = 0,
    /// 1.0
    One = 1,
    /// S.component
    Src = 2,
    /// 1.0 - S.component
    OneMinusSrc = 3,
    /// S.alpha
    SrcAlpha = 4,
    /// 1.0 - S.alpha
    OneMinusSrcAlpha = 5,
    /// D.component
    Dst = 6,
    /// 1.0 - D.component
    OneMinusDst = 7,
    /// D.alpha
    DstAlpha = 8,
    /// 1.0 - D.alpha
    OneMinusDstAlpha = 9,
    /// min(S.alpha, 1.0 - D.alpha)
    SrcAlphaSaturated = 10,
    /// Constant
    Constant = 11,
    /// 1.0 - Constant
    OneMinusConstant = 12,
    /// S1.component
    Src1 = 13,
    /// 1.0 - S1.component
    OneMinusSrc1 = 14,
    /// S1.alpha
    Src1Alpha = 15,
    /// 1.0 - S1.alpha
    OneMinusSrc1Alpha = 16,
}

impl CanvasBlendFactor {
    /// Returns `true` if the blend factor references the second blend source.
    ///
    /// Note that the usage of those blend factors require [`Features::DUAL_SOURCE_BLENDING`].
    pub fn ref_second_blend_source(&self) -> bool {
        match self {
            CanvasBlendFactor::Src1
            | CanvasBlendFactor::OneMinusSrc1
            | CanvasBlendFactor::Src1Alpha
            | CanvasBlendFactor::OneMinusSrc1Alpha => true,
            _ => false,
        }
    }
}

impl From<wgpu_types::BlendFactor> for CanvasBlendFactor {
    fn from(value: wgpu_types::BlendFactor) -> Self {
        match value {
            wgpu_types::BlendFactor::Zero => Self::Zero,
            wgpu_types::BlendFactor::One => Self::One,
            wgpu_types::BlendFactor::Src => Self::Src,
            wgpu_types::BlendFactor::OneMinusSrc => Self::OneMinusSrc,
            wgpu_types::BlendFactor::SrcAlpha => Self::SrcAlpha,
            wgpu_types::BlendFactor::OneMinusSrcAlpha => Self::OneMinusSrcAlpha,
            wgpu_types::BlendFactor::Dst => Self::Dst,
            wgpu_types::BlendFactor::OneMinusDst => Self::OneMinusDst,
            wgpu_types::BlendFactor::DstAlpha => Self::DstAlpha,
            wgpu_types::BlendFactor::OneMinusDstAlpha => Self::OneMinusDstAlpha,
            wgpu_types::BlendFactor::SrcAlphaSaturated => Self::SrcAlphaSaturated,
            wgpu_types::BlendFactor::Constant => Self::Constant,
            wgpu_types::BlendFactor::OneMinusConstant => Self::OneMinusConstant,
            wgpu_types::BlendFactor::Src1 => Self::Src1,
            wgpu_types::BlendFactor::OneMinusSrc1 => Self::OneMinusSrc1,
            wgpu_types::BlendFactor::Src1Alpha => Self::Src1Alpha,
            wgpu_types::BlendFactor::OneMinusSrc1Alpha => Self::OneMinusSrc1Alpha,
        }
    }
}

impl Into<wgpu_types::BlendFactor> for CanvasBlendFactor {
    fn into(self) -> wgpu_types::BlendFactor {
        match self {
            Self::Zero => wgpu_types::BlendFactor::Zero,
            Self::One => wgpu_types::BlendFactor::One,
            Self::Src => wgpu_types::BlendFactor::Src,
            Self::OneMinusSrc => wgpu_types::BlendFactor::OneMinusSrc,
            Self::SrcAlpha => wgpu_types::BlendFactor::SrcAlpha,
            Self::OneMinusSrcAlpha => wgpu_types::BlendFactor::OneMinusSrcAlpha,
            Self::Dst => wgpu_types::BlendFactor::Dst,
            Self::OneMinusDst => wgpu_types::BlendFactor::OneMinusDst,
            Self::DstAlpha => wgpu_types::BlendFactor::DstAlpha,
            Self::OneMinusDstAlpha => wgpu_types::BlendFactor::OneMinusDstAlpha,
            Self::SrcAlphaSaturated => wgpu_types::BlendFactor::SrcAlphaSaturated,
            Self::Constant => wgpu_types::BlendFactor::Constant,
            Self::OneMinusConstant => wgpu_types::BlendFactor::OneMinusConstant,
            Self::Src1 => wgpu_types::BlendFactor::Src1,
            Self::OneMinusSrc1 => wgpu_types::BlendFactor::OneMinusSrc1,
            Self::Src1Alpha => wgpu_types::BlendFactor::Src1Alpha,
            Self::OneMinusSrc1Alpha => wgpu_types::BlendFactor::OneMinusSrc1Alpha,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum CanvasBlendOperation {
    /// Src + Dst
    Add = 0,
    /// Src - Dst
    Subtract = 1,
    /// Dst - Src
    ReverseSubtract = 2,
    /// min(Src, Dst)
    Min = 3,
    /// max(Src, Dst)
    Max = 4,
}

impl From<wgpu_types::BlendOperation> for CanvasBlendOperation {
    fn from(value: wgpu_types::BlendOperation) -> Self {
        match value {
            wgpu_types::BlendOperation::Add => Self::Add,
            wgpu_types::BlendOperation::Subtract => Self::Subtract,
            wgpu_types::BlendOperation::ReverseSubtract => Self::ReverseSubtract,
            wgpu_types::BlendOperation::Min => Self::Min,
            wgpu_types::BlendOperation::Max => Self::Max,
        }
    }
}

impl Into<wgpu_types::BlendOperation> for CanvasBlendOperation {
    fn into(self) -> wgpu_types::BlendOperation {
        match self {
            CanvasBlendOperation::Add => wgpu_types::BlendOperation::Add,
            CanvasBlendOperation::Subtract => wgpu_types::BlendOperation::Subtract,
            CanvasBlendOperation::ReverseSubtract => wgpu_types::BlendOperation::ReverseSubtract,
            CanvasBlendOperation::Min => wgpu_types::BlendOperation::Min,
            CanvasBlendOperation::Max => wgpu_types::BlendOperation::Max,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CanvasVertexAttribute {
    pub format: CanvasVertexFormat,
    pub offset: u64,
    pub shader_location: u32,
}

impl From<wgpu_types::VertexAttribute> for CanvasVertexAttribute {
    fn from(value: wgpu_types::VertexAttribute) -> Self {
        Self {
            format: value.format.into(),
            offset: value.offset,
            shader_location: value.shader_location,
        }
    }
}

impl Into<wgpu_types::VertexAttribute> for CanvasVertexAttribute {
    fn into(self) -> wgpu_types::VertexAttribute {
        wgpu_types::VertexAttribute {
            format: self.format.into(),
            offset: self.offset,
            shader_location: self.shader_location,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CanvasMultisampleState {
    pub count: u32,
    pub mask: u64,
    pub alpha_to_coverage_enabled: bool,
}

impl Default for CanvasMultisampleState {
    fn default() -> Self {
        CanvasMultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        }
    }
}

impl From<wgpu_types::MultisampleState> for CanvasMultisampleState {
    fn from(value: wgpu_types::MultisampleState) -> Self {
        Self {
            count: value.count,
            mask: value.mask,
            alpha_to_coverage_enabled: value.alpha_to_coverage_enabled,
        }
    }
}

impl Into<wgpu_types::MultisampleState> for CanvasMultisampleState {
    fn into(self) -> wgpu_types::MultisampleState {
        wgpu_types::MultisampleState {
            count: self.count,
            mask: self.mask,
            alpha_to_coverage_enabled: self.alpha_to_coverage_enabled,
        }
    }
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct CanvasRenderPassColorAttachment {
    pub view: *const CanvasGPUTextureView,
    pub resolve_target: *const CanvasGPUTextureView,
    pub channel: CanvasPassChannelColor,
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct CanvasPassChannelColor {
    pub load_op: CanvasLoadOp,
    pub store_op: CanvasStoreOp,
    pub clear_value: CanvasColor,
    pub read_only: bool,
}

impl From<wgpu_core::command::PassChannel<wgpu_types::Color>> for CanvasPassChannelColor {
    fn from(value: wgpu_core::command::PassChannel<wgpu_types::Color>) -> Self {
        Self {
            load_op: value.load_op.into(),
            store_op: value.store_op.into(),
            clear_value: value.clear_value.into(),
            read_only: value.read_only,
        }
    }
}

impl Into<wgpu_core::command::PassChannel<wgpu_types::Color>> for CanvasPassChannelColor {
    fn into(self) -> wgpu_core::command::PassChannel<wgpu_types::Color> {
        wgpu_core::command::PassChannel {
            load_op: self.load_op.into(),
            store_op: self.store_op.into(),
            clear_value: self.clear_value.into(),
            read_only: self.read_only,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum CanvasLoadOp {
    Clear = 0,
    Load = 1,
}

impl From<wgpu_core::command::LoadOp> for CanvasLoadOp {
    fn from(value: wgpu_core::command::LoadOp) -> Self {
        match value {
            wgpu_core::command::LoadOp::Clear => Self::Clear,
            wgpu_core::command::LoadOp::Load => Self::Load,
        }
    }
}

impl Into<wgpu_core::command::LoadOp> for CanvasLoadOp {
    fn into(self) -> wgpu_core::command::LoadOp {
        match self {
            CanvasLoadOp::Clear => wgpu_core::command::LoadOp::Clear,
            CanvasLoadOp::Load => wgpu_core::command::LoadOp::Load,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum CanvasStoreOp {
    Discard = 0,
    Store = 1,
}

impl From<wgpu_core::command::StoreOp> for CanvasStoreOp {
    fn from(value: wgpu_core::command::StoreOp) -> Self {
        match value {
            wgpu_core::command::StoreOp::Discard => Self::Discard,
            wgpu_core::command::StoreOp::Store => Self::Store,
        }
    }
}

impl Into<wgpu_core::command::StoreOp> for CanvasStoreOp {
    fn into(self) -> wgpu_core::command::StoreOp {
        match self {
            CanvasStoreOp::Discard => wgpu_core::command::StoreOp::Discard,
            CanvasStoreOp::Store => wgpu_core::command::StoreOp::Store,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CanvasOptionalLoadOp {
    None,
    Some(CanvasLoadOp),
}

impl From<Option<CanvasLoadOp>> for CanvasOptionalLoadOp {
    fn from(value: Option<CanvasLoadOp>) -> Self {
        match value {
            Some(value) => Self::Some(value),
            None => Self::None,
        }
    }
}

impl Into<Option<CanvasLoadOp>> for CanvasOptionalLoadOp {
    fn into(self) -> Option<CanvasLoadOp> {
        match self {
            CanvasOptionalLoadOp::None => None,
            CanvasOptionalLoadOp::Some(value) => Some(value),
        }
    }
}

impl From<Option<wgpu_core::command::LoadOp>> for CanvasOptionalLoadOp {
    fn from(value: Option<wgpu_core::command::LoadOp>) -> Self {
        match value {
            Some(value) => Self::Some(value.into()),
            None => Self::None,
        }
    }
}

impl Into<Option<wgpu_core::command::LoadOp>> for CanvasOptionalLoadOp {
    fn into(self) -> Option<wgpu_core::command::LoadOp> {
        match self {
            CanvasOptionalLoadOp::None => None,
            CanvasOptionalLoadOp::Some(value) => Some(value.into()),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CanvasOptionalStoreOp {
    None,
    Some(CanvasStoreOp),
}

impl From<Option<CanvasStoreOp>> for CanvasOptionalStoreOp {
    fn from(value: Option<CanvasStoreOp>) -> Self {
        match value {
            Some(value) => Self::Some(value),
            None => Self::None,
        }
    }
}

impl Into<Option<CanvasStoreOp>> for CanvasOptionalStoreOp {
    fn into(self) -> Option<CanvasStoreOp> {
        match self {
            CanvasOptionalStoreOp::None => None,
            CanvasOptionalStoreOp::Some(value) => Some(value),
        }
    }
}

impl From<Option<wgpu_core::command::StoreOp>> for CanvasOptionalStoreOp {
    fn from(value: Option<wgpu_core::command::StoreOp>) -> Self {
        match value {
            Some(value) => Self::Some(value.into()),
            None => Self::None,
        }
    }
}

impl Into<Option<wgpu_core::command::StoreOp>> for CanvasOptionalStoreOp {
    fn into(self) -> Option<wgpu_core::command::StoreOp> {
        match self {
            CanvasOptionalStoreOp::None => None,
            CanvasOptionalStoreOp::Some(value) => Some(value.into()),
        }
    }
}

#[repr(C)]
pub struct CanvasRenderPassDepthStencilAttachment {
    pub view: *const CanvasGPUTextureView,
    pub depth_clear_value: f32,
    pub depth_load_op: CanvasOptionalLoadOp,
    pub depth_store_op: CanvasOptionalStoreOp,
    pub depth_read_only: bool,
    pub stencil_clear_value: u32,
    pub stencil_load_op: CanvasOptionalLoadOp,
    pub stencil_store_op: CanvasOptionalStoreOp,
    pub stencil_read_only: bool,
}

#[repr(C)]
pub struct CanvasSurfaceCapabilities {
    pub formats: *const StringBuffer,
    pub present_modes: *const StringBuffer,
    pub alpha_modes: *const StringBuffer,
    pub usages: u32,
}

impl From<SurfaceCapabilities> for CanvasSurfaceCapabilities {
    fn from(value: SurfaceCapabilities) -> Self {
        let formats = Box::into_raw(Box::new(
            StringBuffer::from(value.formats.into_iter().map(|v| {
                let format: CanvasGPUTextureFormat = v.into();
                format.into()
            }).collect::<Vec<String>>())
        ));

        let present_modes = Box::into_raw(Box::new(
            StringBuffer::from(value.present_modes.into_iter().map(|v| {
                match v {
                    PresentMode::AutoVsync => "autoVsync".to_string(),
                    PresentMode::AutoNoVsync => "autoNoVsync".to_string(),
                    PresentMode::Fifo => "fifo".to_string(),
                    PresentMode::FifoRelaxed => "fifoRelaxed".to_string(),
                    PresentMode::Immediate => "immediate".to_string(),
                    PresentMode::Mailbox => "mailbox".to_string(),
                }
            }).collect::<Vec<_>>())
        ));


        let alpha_modes = Box::into_raw(Box::new(
            StringBuffer::from(value.alpha_modes.into_iter().map(|v| {
                match v {
                    CompositeAlphaMode::Auto => "auto".to_string(),
                    CompositeAlphaMode::Opaque => "opaque".to_string(),
                    CompositeAlphaMode::PreMultiplied => "premultiplied".to_string(),
                    CompositeAlphaMode::PostMultiplied => "postmultiplied".to_string(),
                    CompositeAlphaMode::Inherit => "inherit".to_string()
                }
            }).collect::<Vec<_>>())
        ));

        Self {
            formats,
            present_modes,
            alpha_modes,
            usages: value.usages.bits(),
        }
    }
}

impl Drop for CanvasSurfaceCapabilities {
    fn drop(&mut self) {
        if !self.formats.is_null() {
            let _ = unsafe { Box::from_raw(self.formats as *mut StringBuffer) };
        }

        if !self.present_modes.is_null() {
            let _ = unsafe { Box::from_raw(self.present_modes as *mut StringBuffer) };
        }

        if !self.alpha_modes.is_null() {
            let _ = unsafe { Box::from_raw(self.alpha_modes as *mut StringBuffer) };
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_struct_surface_capabilities_release(
    cap: *mut CanvasSurfaceCapabilities
) {
    if cap.is_null() {
        return;
    }
    let _ = Box::from_raw(cap);
}


