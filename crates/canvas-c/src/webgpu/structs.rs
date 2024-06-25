use wgpu_core::naga::valid;

use super::enums::{CanvasGPUTextureFormat, CanvasTextureAspect, CanvasVertexFormat};

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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
        Self::Color {
            r: value.r,
            g: value.g,
            b: value.b,
            a: value.a,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CanvasImageDataLayout {
    offset: u64,
    bytes_per_row: Option<u32>,
    rows_per_image: Option<u32>,
}

impl From<wgpu_types::ImageDataLayout> for CanvasImageDataLayout {
    fn from(value: wgpu_types::ImageDataLayout) -> Self {
        Self {
            offset: value.offset,
            bytes_per_row: value.bytes_per_row,
            rows_per_image: value.rows_per_image,
        }
    }
}

impl Into<wgpu_types::ImageDataLayout> for CanvasImageDataLayout {
    fn into(self) -> wgpu_types::ImageDataLayout {
        wgpu_types::ImageDataLayout {
            offset: self.offset,
            bytes_per_row: self.bytes_per_row,
            rows_per_image: self.rows_per_image,
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
            mip_level_count: value.mip_level_count.try_into().unwrap_or(-1),
            base_array_layer: value.base_array_layer,
            array_layer_count: value.array_layer_count.try_into().unwrap_or(-1),
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
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CanvasColorTargetState {
    pub format: CanvasGPUTextureFormat,
    pub blend: CanvasOptionalBlendState,
    pub write_mask: u32,
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq)]
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

impl From<Option<wgpu_types::BlendState>> for CanvasOptionalBlendState {
    fn from(value: Option<wgpu_types::BlendState>) -> Self {
        match value {
            Some(value) => Some(value.into()),
            None => Self::None,
        }
    }
}

impl Into<Option<wgpu_types::BlendState>> for CanvasOptionalBlendState {
    fn into(self) -> Option<wgpu_types::BlendState> {
        match self {
            CanvasOptionalBlendState::None => None,
            CanvasOptionalBlendState::Some(value) => Some(value.into()),
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

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
pub enum CanvasBlendOperation {
    /// Src + Dst
    #[default]
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
