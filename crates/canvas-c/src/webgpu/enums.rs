use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
};

use wgpu_core::binding_model::{BindGroupEntry, BufferBinding};
use wgt::{
    AddressMode, BindGroupLayoutEntry, BufferBindingType, CompareFunction, FilterMode,
    QueryType, SamplerBindingType, StorageTextureAccess, TextureSampleType,
};

use crate::webgpu::gpu_buffer::CanvasGPUBuffer;
use crate::webgpu::gpu_sampler::CanvasGPUSampler;
use crate::webgpu::gpu_texture_view::CanvasGPUTextureView;


#[repr(C)]
#[derive(Copy, Clone, Debug,Eq, PartialEq)]
pub enum SurfaceGetCurrentTextureStatus {
    Success = 0x00000000,
    Timeout = 0x00000001,
    Outdated = 0x00000002,
    Lost = 0x00000003,
    OutOfMemory = 0x00000004,
    DeviceLost = 0x00000005,
    Force32 = 0x7FFFFFFF
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum CanvasTextureDimension {
    D1,
    D2,
    D3,
}

impl From<wgt::TextureDimension> for CanvasTextureDimension {
    fn from(value: wgt::TextureDimension) -> Self {
        match value {
            wgt::TextureDimension::D1 => Self::D1,
            wgt::TextureDimension::D2 => Self::D2,
            wgt::TextureDimension::D3 => Self::D3,
        }
    }
}

impl Into<wgt::TextureDimension> for CanvasTextureDimension {
    fn into(self) -> wgt::TextureDimension {
        match self {
            CanvasTextureDimension::D1 => wgt::TextureDimension::D1,
            CanvasTextureDimension::D2 => wgt::TextureDimension::D2,
            CanvasTextureDimension::D3 => wgt::TextureDimension::D3,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum CanvasOptionalTextureDimension {
    None,
    D1,
    D2,
    D3,
}

impl From<Option<wgt::TextureDimension>> for CanvasOptionalTextureDimension {
    fn from(value: Option<wgt::TextureDimension>) -> Self {
        match value {
            Some(value) => match value {
                wgt::TextureDimension::D1 => Self::D1,
                wgt::TextureDimension::D2 => Self::D2,
                wgt::TextureDimension::D3 => Self::D3,
            },
            None => Self::None,
        }
    }
}

impl Into<Option<wgt::TextureDimension>> for CanvasOptionalTextureDimension {
    fn into(self) -> Option<wgt::TextureDimension> {
        match self {
            CanvasOptionalTextureDimension::None => None,
            CanvasOptionalTextureDimension::D1 => Some(wgt::TextureDimension::D1),
            CanvasOptionalTextureDimension::D2 => Some(wgt::TextureDimension::D2),
            CanvasOptionalTextureDimension::D3 => Some(wgt::TextureDimension::D3),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum CanvasTextureViewDimension {
    D1,
    D2,
    D2Array,
    Cube,
    CubeArray,
    D3,
}

impl CanvasTextureViewDimension {
    /// Get the texture dimension required of this texture view dimension.
    pub fn compatible_texture_dimension(self) -> CanvasTextureViewDimension {
        match self {
            Self::D1 => CanvasTextureViewDimension::D1,
            Self::D2 | Self::D2Array | Self::Cube | Self::CubeArray => {
                CanvasTextureViewDimension::D2
            }
            Self::D3 => CanvasTextureViewDimension::D3,
        }
    }
}

impl From<wgt::TextureViewDimension> for CanvasTextureViewDimension {
    fn from(value: wgt::TextureViewDimension) -> Self {
        match value {
            wgt::TextureViewDimension::D1 => Self::D1,
            wgt::TextureViewDimension::D2 => Self::D2,
            wgt::TextureViewDimension::D2Array => Self::D2Array,
            wgt::TextureViewDimension::Cube => Self::Cube,
            wgt::TextureViewDimension::CubeArray => Self::CubeArray,
            wgt::TextureViewDimension::D3 => Self::D3,
        }
    }
}

impl Into<wgt::TextureViewDimension> for CanvasTextureViewDimension {
    fn into(self) -> wgt::TextureViewDimension {
        match self {
            CanvasTextureViewDimension::D1 => wgt::TextureViewDimension::D1,
            CanvasTextureViewDimension::D2 => wgt::TextureViewDimension::D2,
            CanvasTextureViewDimension::D2Array => wgt::TextureViewDimension::D2Array,
            CanvasTextureViewDimension::Cube => wgt::TextureViewDimension::Cube,
            CanvasTextureViewDimension::CubeArray => wgt::TextureViewDimension::CubeArray,
            CanvasTextureViewDimension::D3 => wgt::TextureViewDimension::D3,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum CanvasOptionalTextureViewDimension {
    None,
    D1,

    D2,
    D2Array,
    Cube,
    CubeArray,
    D3,
}

impl From<Option<wgt::TextureViewDimension>> for CanvasOptionalTextureViewDimension {
    fn from(value: Option<wgt::TextureViewDimension>) -> Self {
        match value {
            Some(value) => match value {
                wgt::TextureViewDimension::D1 => Self::D1,
                wgt::TextureViewDimension::D2 => Self::D2,
                wgt::TextureViewDimension::D2Array => Self::D2Array,
                wgt::TextureViewDimension::Cube => Self::Cube,
                wgt::TextureViewDimension::CubeArray => Self::CubeArray,
                wgt::TextureViewDimension::D3 => Self::D3,
            },
            None => Self::None,
        }
    }
}

impl Into<Option<wgt::TextureViewDimension>> for CanvasOptionalTextureViewDimension {
    fn into(self) -> Option<wgt::TextureViewDimension> {
        match self {
            CanvasOptionalTextureViewDimension::None => None,
            CanvasOptionalTextureViewDimension::D1 => Some(wgt::TextureViewDimension::D1),
            CanvasOptionalTextureViewDimension::D2 => Some(wgt::TextureViewDimension::D2),
            CanvasOptionalTextureViewDimension::D2Array => {
                Some(wgt::TextureViewDimension::D2Array)
            }
            CanvasOptionalTextureViewDimension::Cube => {
                Some(wgt::TextureViewDimension::Cube)
            }
            CanvasOptionalTextureViewDimension::CubeArray => {
                Some(wgt::TextureViewDimension::CubeArray)
            }
            CanvasOptionalTextureViewDimension::D3 => Some(wgt::TextureViewDimension::D3),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum CanvasGPUTextureFormat {
    // Normal 8 bit formats
    /// Red channel only. 8 bit integer per channel. [0, 255] converted to/from float [0, 1] in shader.
    R8Unorm,
    /// Red channel only. 8 bit integer per channel. [-127, 127] converted to/from float [-1, 1] in shader.
    R8Snorm,
    /// Red channel only. 8 bit integer per channel. Unsigned in shader.
    R8Uint,
    /// Red channel only. 8 bit integer per channel. Signed in shader.
    R8Sint,

    // Normal 16 bit formats
    /// Red channel only. 16 bit integer per channel. Unsigned in shader.
    R16Uint,
    /// Red channel only. 16 bit integer per channel. Signed in shader.
    R16Sint,
    /// Red channel only. 16 bit integer per channel. [0, 65535] converted to/from float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_FORMAT_16BIT_NORM`] must be enabled to use this texture format.
    R16Unorm,
    /// Red channel only. 16 bit integer per channel. [0, 65535] converted to/from float [-1, 1] in shader.
    ///
    /// [`Features::TEXTURE_FORMAT_16BIT_NORM`] must be enabled to use this texture format.
    R16Snorm,
    /// Red channel only. 16 bit float per channel. Float in shader.
    R16Float,
    /// Red and green channels. 8 bit integer per channel. [0, 255] converted to/from float [0, 1] in shader.
    Rg8Unorm,
    /// Red and green channels. 8 bit integer per channel. [-127, 127] converted to/from float [-1, 1] in shader.
    Rg8Snorm,
    /// Red and green channels. 8 bit integer per channel. Unsigned in shader.
    Rg8Uint,
    /// Red and green channels. 8 bit integer per channel. Signed in shader.
    Rg8Sint,

    // Normal 32 bit formats
    /// Red channel only. 32 bit integer per channel. Unsigned in shader.
    R32Uint,
    /// Red channel only. 32 bit integer per channel. Signed in shader.
    R32Sint,
    /// Red channel only. 32 bit float per channel. Float in shader.
    R32Float,
    /// Red and green channels. 16 bit integer per channel. Unsigned in shader.
    Rg16Uint,
    /// Red and green channels. 16 bit integer per channel. Signed in shader.
    Rg16Sint,
    /// Red and green channels. 16 bit integer per channel. [0, 65535] converted to/from float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_FORMAT_16BIT_NORM`] must be enabled to use this texture format.
    Rg16Unorm,
    /// Red and green channels. 16 bit integer per channel. [0, 65535] converted to/from float [-1, 1] in shader.
    ///
    /// [`Features::TEXTURE_FORMAT_16BIT_NORM`] must be enabled to use this texture format.
    Rg16Snorm,
    /// Red and green channels. 16 bit float per channel. Float in shader.
    Rg16Float,
    /// Red, green, blue, and alpha channels. 8 bit integer per channel. [0, 255] converted to/from float [0, 1] in shader.
    Rgba8Unorm,
    /// Red, green, blue, and alpha channels. 8 bit integer per channel. Srgb-color [0, 255] converted to/from linear-color float [0, 1] in shader.
    Rgba8UnormSrgb,
    /// Red, green, blue, and alpha channels. 8 bit integer per channel. [-127, 127] converted to/from float [-1, 1] in shader.
    Rgba8Snorm,
    /// Red, green, blue, and alpha channels. 8 bit integer per channel. Unsigned in shader.
    Rgba8Uint,
    /// Red, green, blue, and alpha channels. 8 bit integer per channel. Signed in shader.
    Rgba8Sint,
    /// Blue, green, red, and alpha channels. 8 bit integer per channel. [0, 255] converted to/from float [0, 1] in shader.
    Bgra8Unorm,
    /// Blue, green, red, and alpha channels. 8 bit integer per channel. Srgb-color [0, 255] converted to/from linear-color float [0, 1] in shader.
    Bgra8UnormSrgb,

    // Packed 32 bit formats
    /// Packed unsigned float with 9 bits mantisa for each RGB component, then a common 5 bits exponent
    Rgb9e5Ufloat,
    /// Red, green, blue, and alpha channels. 10 bit integer for RGB channels, 2 bit integer for alpha channel. Unsigned in shader.
    Rgb10a2Uint,
    /// Red, green, blue, and alpha channels. 10 bit integer for RGB channels, 2 bit integer for alpha channel. [0, 1023] ([0, 3] for alpha) converted to/from float [0, 1] in shader.
    Rgb10a2Unorm,
    /// Red, green, and blue channels. 11 bit float with no sign bit for RG channels. 10 bit float with no sign bit for blue channel. Float in shader.
    Rg11b10UFloat,

    // Normal 64 bit formats
    /// Red and green channels. 32 bit integer per channel. Unsigned in shader.
    Rg32Uint,
    /// Red and green channels. 32 bit integer per channel. Signed in shader.
    Rg32Sint,
    /// Red and green channels. 32 bit float per channel. Float in shader.
    Rg32Float,
    /// Red, green, blue, and alpha channels. 16 bit integer per channel. Unsigned in shader.
    Rgba16Uint,
    /// Red, green, blue, and alpha channels. 16 bit integer per channel. Signed in shader.
    Rgba16Sint,
    /// Red, green, blue, and alpha channels. 16 bit integer per channel. [0, 65535] converted to/from float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_FORMAT_16BIT_NORM`] must be enabled to use this texture format.
    Rgba16Unorm,
    /// Red, green, blue, and alpha. 16 bit integer per channel. [0, 65535] converted to/from float [-1, 1] in shader.
    ///
    /// [`Features::TEXTURE_FORMAT_16BIT_NORM`] must be enabled to use this texture format.
    Rgba16Snorm,
    /// Red, green, blue, and alpha channels. 16 bit float per channel. Float in shader.
    Rgba16Float,

    // Normal 128 bit formats
    /// Red, green, blue, and alpha channels. 32 bit integer per channel. Unsigned in shader.
    Rgba32Uint,
    /// Red, green, blue, and alpha channels. 32 bit integer per channel. Signed in shader.
    Rgba32Sint,
    /// Red, green, blue, and alpha channels. 32 bit float per channel. Float in shader.
    Rgba32Float,

    // Depth and stencil formats
    /// Stencil format with 8 bit integer stencil.
    Stencil8,
    /// Special depth format with 16 bit integer depth.
    Depth16Unorm,
    /// Special depth format with at least 24 bit integer depth.
    Depth24Plus,
    /// Special depth/stencil format with at least 24 bit integer depth and 8 bits integer stencil.
    Depth24PlusStencil8,
    /// Special depth format with 32 bit floating point depth.
    Depth32Float,
    /// Special depth/stencil format with 32 bit floating point depth and 8 bits integer stencil.
    ///
    /// [`Features::DEPTH32FLOAT_STENCIL8`] must be enabled to use this texture format.
    Depth32FloatStencil8,

    /// YUV 4:2:0 chroma subsampled format.
    ///
    /// Contains two planes:
    /// - 0: Single 8 bit channel luminance.
    /// - 1: Dual 8 bit channel chrominance at half width and half height.
    ///
    /// Valid view formats for luminance are [`TextureFormat::R8Unorm`].
    ///
    /// Valid view formats for chrominance are [`TextureFormat::Rg8Unorm`].
    ///
    /// Width and height must be even.
    ///
    /// [`Features::TEXTURE_FORMAT_NV12`] must be enabled to use this texture format.
    NV12,

    // Compressed textures usable with `TEXTURE_COMPRESSION_BC` feature.
    /// 4x4 block compressed texture. 8 bytes per block (4 bit/px). 4 color + alpha pallet. 5 bit R + 6 bit G + 5 bit B + 1 bit alpha.
    /// [0, 63] ([0, 1] for alpha) converted to/from float [0, 1] in shader.
    ///
    /// Also known as DXT1.
    ///
    /// [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
    Bc1RgbaUnorm,
    /// 4x4 block compressed texture. 8 bytes per block (4 bit/px). 4 color + alpha pallet. 5 bit R + 6 bit G + 5 bit B + 1 bit alpha.
    /// Srgb-color [0, 63] ([0, 1] for alpha) converted to/from linear-color float [0, 1] in shader.
    ///
    /// Also known as DXT1.
    ///
    /// [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
    Bc1RgbaUnormSrgb,
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px). 4 color pallet. 5 bit R + 6 bit G + 5 bit B + 4 bit alpha.
    /// [0, 63] ([0, 15] for alpha) converted to/from float [0, 1] in shader.
    ///
    /// Also known as DXT3.
    ///
    /// [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
    Bc2RgbaUnorm,
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px). 4 color pallet. 5 bit R + 6 bit G + 5 bit B + 4 bit alpha.
    /// Srgb-color [0, 63] ([0, 255] for alpha) converted to/from linear-color float [0, 1] in shader.
    ///
    /// Also known as DXT3.
    ///
    /// [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
    Bc2RgbaUnormSrgb,
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px). 4 color pallet + 8 alpha pallet. 5 bit R + 6 bit G + 5 bit B + 8 bit alpha.
    /// [0, 63] ([0, 255] for alpha) converted to/from float [0, 1] in shader.
    ///
    /// Also known as DXT5.
    ///
    /// [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
    Bc3RgbaUnorm,
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px). 4 color pallet + 8 alpha pallet. 5 bit R + 6 bit G + 5 bit B + 8 bit alpha.
    /// Srgb-color [0, 63] ([0, 255] for alpha) converted to/from linear-color float [0, 1] in shader.
    ///
    /// Also known as DXT5.
    ///
    /// [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
    Bc3RgbaUnormSrgb,
    /// 4x4 block compressed texture. 8 bytes per block (4 bit/px). 8 color pallet. 8 bit R.
    /// [0, 255] converted to/from float [0, 1] in shader.
    ///
    /// Also known as RGTC1.
    ///
    /// [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
    Bc4RUnorm,
    /// 4x4 block compressed texture. 8 bytes per block (4 bit/px). 8 color pallet. 8 bit R.
    /// [-127, 127] converted to/from float [-1, 1] in shader.
    ///
    /// Also known as RGTC1.
    ///
    /// [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
    Bc4RSnorm,
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px). 8 color red pallet + 8 color green pallet. 8 bit RG.
    /// [0, 255] converted to/from float [0, 1] in shader.
    ///
    /// Also known as RGTC2.
    ///
    /// [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
    Bc5RgUnorm,
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px). 8 color red pallet + 8 color green pallet. 8 bit RG.
    /// [-127, 127] converted to/from float [-1, 1] in shader.
    ///
    /// Also known as RGTC2.
    ///
    /// [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
    Bc5RgSnorm,
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px). Variable sized pallet. 16 bit unsigned float RGB. Float in shader.
    ///
    /// Also known as BPTC (float).
    ///
    /// [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
    Bc6hRgbUfloat,
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px). Variable sized pallet. 16 bit signed float RGB. Float in shader.
    ///
    /// Also known as BPTC (float).
    ///
    /// [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
    Bc6hRgbFloat,
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px). Variable sized pallet. 8 bit integer RGBA.
    /// [0, 255] converted to/from float [0, 1] in shader.
    ///
    /// Also known as BPTC (unorm).
    ///
    /// [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
    Bc7RgbaUnorm,
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px). Variable sized pallet. 8 bit integer RGBA.
    /// Srgb-color [0, 255] converted to/from linear-color float [0, 1] in shader.
    ///
    /// Also known as BPTC (unorm).
    ///
    /// [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
    Bc7RgbaUnormSrgb,
    /// 4x4 block compressed texture. 8 bytes per block (4 bit/px). Complex pallet. 8 bit integer RGB.
    /// [0, 255] converted to/from float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ETC2`] must be enabled to use this texture format.
    Etc2Rgb8Unorm,
    /// 4x4 block compressed texture. 8 bytes per block (4 bit/px). Complex pallet. 8 bit integer RGB.
    /// Srgb-color [0, 255] converted to/from linear-color float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ETC2`] must be enabled to use this texture format.
    Etc2Rgb8UnormSrgb,
    /// 4x4 block compressed texture. 8 bytes per block (4 bit/px). Complex pallet. 8 bit integer RGB + 1 bit alpha.
    /// [0, 255] ([0, 1] for alpha) converted to/from float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ETC2`] must be enabled to use this texture format.
    Etc2Rgb8A1Unorm,
    /// 4x4 block compressed texture. 8 bytes per block (4 bit/px). Complex pallet. 8 bit integer RGB + 1 bit alpha.
    /// Srgb-color [0, 255] ([0, 1] for alpha) converted to/from linear-color float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ETC2`] must be enabled to use this texture format.
    Etc2Rgb8A1UnormSrgb,
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px). Complex pallet. 8 bit integer RGB + 8 bit alpha.
    /// [0, 255] converted to/from float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ETC2`] must be enabled to use this texture format.
    Etc2Rgba8Unorm,
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px). Complex pallet. 8 bit integer RGB + 8 bit alpha.
    /// Srgb-color [0, 255] converted to/from linear-color float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ETC2`] must be enabled to use this texture format.
    Etc2Rgba8UnormSrgb,
    /// 4x4 block compressed texture. 8 bytes per block (4 bit/px). Complex pallet. 11 bit integer R.
    /// [0, 255] converted to/from float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ETC2`] must be enabled to use this texture format.
    EacR11Unorm,
    /// 4x4 block compressed texture. 8 bytes per block (4 bit/px). Complex pallet. 11 bit integer R.
    /// [-127, 127] converted to/from float [-1, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ETC2`] must be enabled to use this texture format.
    EacR11Snorm,
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px). Complex pallet. 11 bit integer R + 11 bit integer G.
    /// [0, 255] converted to/from float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ETC2`] must be enabled to use this texture format.
    EacRg11Unorm,
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px). Complex pallet. 11 bit integer R + 11 bit integer G.
    /// [-127, 127] converted to/from float [-1, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ETC2`] must be enabled to use this texture format.
    EacRg11Snorm,
    /// block compressed texture. 16 bytes per block.
    ///
    /// Features [`TEXTURE_COMPRESSION_ASTC`] or [`TEXTURE_COMPRESSION_ASTC_HDR`]
    /// must be enabled to use this texture format.
    ///
    /// [`TEXTURE_COMPRESSION_ASTC`]: Features::TEXTURE_COMPRESSION_ASTC
    /// [`TEXTURE_COMPRESSION_ASTC_HDR`]: Features::TEXTURE_COMPRESSION_ASTC_HDR
    Astc {
        /// compressed block dimensions
        block: CanvasAstcBlock,
        /// ASTC RGBA channel
        channel: CanvasAstcChannel,
    },
}

impl From<wgt::TextureFormat> for CanvasGPUTextureFormat {
    fn from(value: wgt::TextureFormat) -> Self {
        match value {
            wgt::TextureFormat::Rgba8Unorm => CanvasGPUTextureFormat::Rgba8Unorm,
            wgt::TextureFormat::Bgra8Unorm => CanvasGPUTextureFormat::Bgra8Unorm,
            wgt::TextureFormat::R8Unorm => CanvasGPUTextureFormat::R8Unorm,
            wgt::TextureFormat::R8Snorm => CanvasGPUTextureFormat::R8Snorm,
            wgt::TextureFormat::R8Uint => CanvasGPUTextureFormat::R8Uint,
            wgt::TextureFormat::R8Sint => CanvasGPUTextureFormat::R8Sint,
            wgt::TextureFormat::R16Uint => CanvasGPUTextureFormat::R16Uint,
            wgt::TextureFormat::R16Sint => CanvasGPUTextureFormat::R16Sint,
            wgt::TextureFormat::R16Unorm => CanvasGPUTextureFormat::R16Unorm,
            wgt::TextureFormat::R16Snorm => CanvasGPUTextureFormat::R16Snorm,
            wgt::TextureFormat::R16Float => CanvasGPUTextureFormat::R16Float,
            wgt::TextureFormat::Rg8Unorm => CanvasGPUTextureFormat::Rg8Unorm,
            wgt::TextureFormat::Rg8Snorm => CanvasGPUTextureFormat::Rg8Snorm,
            wgt::TextureFormat::Rg8Uint => CanvasGPUTextureFormat::Rg8Uint,
            wgt::TextureFormat::Rg8Sint => CanvasGPUTextureFormat::Rg8Sint,
            wgt::TextureFormat::R32Uint => CanvasGPUTextureFormat::R32Uint,
            wgt::TextureFormat::R32Sint => CanvasGPUTextureFormat::R32Sint,
            wgt::TextureFormat::R32Float => CanvasGPUTextureFormat::R32Float,
            wgt::TextureFormat::Rg16Uint => CanvasGPUTextureFormat::Rg16Uint,
            wgt::TextureFormat::Rg16Sint => CanvasGPUTextureFormat::Rg16Sint,
            wgt::TextureFormat::Rg16Unorm => CanvasGPUTextureFormat::Rg16Unorm,
            wgt::TextureFormat::Rg16Snorm => CanvasGPUTextureFormat::Rg16Snorm,
            wgt::TextureFormat::Rg16Float => CanvasGPUTextureFormat::Rg16Float,
            wgt::TextureFormat::Rgba8UnormSrgb => CanvasGPUTextureFormat::Rgba8UnormSrgb,
            wgt::TextureFormat::Rgba8Snorm => CanvasGPUTextureFormat::Rgba8Snorm,
            wgt::TextureFormat::Rgba8Uint => CanvasGPUTextureFormat::Rgba8Uint,
            wgt::TextureFormat::Rgba8Sint => CanvasGPUTextureFormat::Rgba8Sint,
            wgt::TextureFormat::Bgra8UnormSrgb => CanvasGPUTextureFormat::Bgra8UnormSrgb,
            wgt::TextureFormat::Rgb9e5Ufloat => CanvasGPUTextureFormat::Rgb9e5Ufloat,
            wgt::TextureFormat::Rgb10a2Uint => CanvasGPUTextureFormat::Rgb10a2Uint,
            wgt::TextureFormat::Rgb10a2Unorm => CanvasGPUTextureFormat::Rgb10a2Unorm,
            wgt::TextureFormat::Rg11b10Ufloat => CanvasGPUTextureFormat::Rg11b10UFloat,
            wgt::TextureFormat::Rg32Uint => CanvasGPUTextureFormat::Rg32Uint,
            wgt::TextureFormat::Rg32Sint => CanvasGPUTextureFormat::Rg32Sint,
            wgt::TextureFormat::Rg32Float => CanvasGPUTextureFormat::Rg32Float,
            wgt::TextureFormat::Rgba16Uint => CanvasGPUTextureFormat::Rgba16Uint,
            wgt::TextureFormat::Rgba16Sint => CanvasGPUTextureFormat::Rgba16Sint,
            wgt::TextureFormat::Rgba16Unorm => CanvasGPUTextureFormat::Rgba16Unorm,
            wgt::TextureFormat::Rgba16Snorm => CanvasGPUTextureFormat::Rgba16Snorm,
            wgt::TextureFormat::Rgba16Float => CanvasGPUTextureFormat::Rgba16Float,
            wgt::TextureFormat::Rgba32Uint => CanvasGPUTextureFormat::Rgba32Uint,
            wgt::TextureFormat::Rgba32Sint => CanvasGPUTextureFormat::Rgba32Sint,
            wgt::TextureFormat::Rgba32Float => CanvasGPUTextureFormat::Rgba32Float,
            wgt::TextureFormat::Stencil8 => CanvasGPUTextureFormat::Stencil8,
            wgt::TextureFormat::Depth16Unorm => CanvasGPUTextureFormat::Depth16Unorm,
            wgt::TextureFormat::Depth24Plus => CanvasGPUTextureFormat::Depth24Plus,
            wgt::TextureFormat::Depth24PlusStencil8 => {
                CanvasGPUTextureFormat::Depth24PlusStencil8
            }
            wgt::TextureFormat::Depth32Float => CanvasGPUTextureFormat::Depth32Float,
            wgt::TextureFormat::Depth32FloatStencil8 => {
                CanvasGPUTextureFormat::Depth32FloatStencil8
            }
            wgt::TextureFormat::NV12 => CanvasGPUTextureFormat::NV12,
            wgt::TextureFormat::Bc1RgbaUnorm => CanvasGPUTextureFormat::Bc1RgbaUnorm,
            wgt::TextureFormat::Bc1RgbaUnormSrgb => CanvasGPUTextureFormat::Bc1RgbaUnormSrgb,
            wgt::TextureFormat::Bc2RgbaUnorm => CanvasGPUTextureFormat::Bc2RgbaUnorm,
            wgt::TextureFormat::Bc2RgbaUnormSrgb => CanvasGPUTextureFormat::Bc2RgbaUnormSrgb,
            wgt::TextureFormat::Bc3RgbaUnorm => CanvasGPUTextureFormat::Bc3RgbaUnorm,
            wgt::TextureFormat::Bc3RgbaUnormSrgb => CanvasGPUTextureFormat::Bc3RgbaUnormSrgb,
            wgt::TextureFormat::Bc4RUnorm => CanvasGPUTextureFormat::Bc4RUnorm,
            wgt::TextureFormat::Bc4RSnorm => CanvasGPUTextureFormat::Bc4RSnorm,
            wgt::TextureFormat::Bc5RgUnorm => CanvasGPUTextureFormat::Bc5RgUnorm,
            wgt::TextureFormat::Bc5RgSnorm => CanvasGPUTextureFormat::Bc5RgSnorm,
            wgt::TextureFormat::Bc6hRgbUfloat => CanvasGPUTextureFormat::Bc6hRgbUfloat,
            wgt::TextureFormat::Bc6hRgbFloat => CanvasGPUTextureFormat::Bc6hRgbFloat,
            wgt::TextureFormat::Bc7RgbaUnorm => CanvasGPUTextureFormat::Bc7RgbaUnorm,
            wgt::TextureFormat::Bc7RgbaUnormSrgb => CanvasGPUTextureFormat::Bc7RgbaUnormSrgb,
            wgt::TextureFormat::Etc2Rgb8Unorm => CanvasGPUTextureFormat::Etc2Rgb8Unorm,
            wgt::TextureFormat::Etc2Rgb8UnormSrgb => {
                CanvasGPUTextureFormat::Etc2Rgb8UnormSrgb
            }
            wgt::TextureFormat::Etc2Rgb8A1Unorm => CanvasGPUTextureFormat::Etc2Rgb8A1Unorm,
            wgt::TextureFormat::Etc2Rgb8A1UnormSrgb => {
                CanvasGPUTextureFormat::Etc2Rgb8A1UnormSrgb
            }
            wgt::TextureFormat::Etc2Rgba8Unorm => CanvasGPUTextureFormat::Etc2Rgba8Unorm,
            wgt::TextureFormat::Etc2Rgba8UnormSrgb => {
                CanvasGPUTextureFormat::Etc2Rgba8UnormSrgb
            }
            wgt::TextureFormat::EacR11Unorm => CanvasGPUTextureFormat::EacR11Unorm,
            wgt::TextureFormat::EacR11Snorm => CanvasGPUTextureFormat::EacR11Snorm,
            wgt::TextureFormat::EacRg11Unorm => CanvasGPUTextureFormat::EacRg11Unorm,
            wgt::TextureFormat::EacRg11Snorm => CanvasGPUTextureFormat::EacRg11Snorm,
            wgt::TextureFormat::Astc { block, channel } => CanvasGPUTextureFormat::Astc {
                block: block.into(),
                channel: channel.into(),
            },
        }
    }
}

impl Into<wgt::TextureFormat> for CanvasGPUTextureFormat {
    fn into(self) -> wgt::TextureFormat {
        match self {
            CanvasGPUTextureFormat::Rgba8Unorm => wgt::TextureFormat::Rgba8Unorm,
            CanvasGPUTextureFormat::Bgra8Unorm => wgt::TextureFormat::Bgra8Unorm,
            CanvasGPUTextureFormat::R8Unorm => wgt::TextureFormat::R8Unorm,
            CanvasGPUTextureFormat::R8Snorm => wgt::TextureFormat::R8Snorm,
            CanvasGPUTextureFormat::R8Uint => wgt::TextureFormat::R8Uint,
            CanvasGPUTextureFormat::R8Sint => wgt::TextureFormat::R8Sint,
            CanvasGPUTextureFormat::R16Uint => wgt::TextureFormat::R16Uint,
            CanvasGPUTextureFormat::R16Sint => wgt::TextureFormat::R16Sint,
            CanvasGPUTextureFormat::R16Unorm => wgt::TextureFormat::R16Unorm,
            CanvasGPUTextureFormat::R16Snorm => wgt::TextureFormat::R16Snorm,
            CanvasGPUTextureFormat::R16Float => wgt::TextureFormat::R16Float,
            CanvasGPUTextureFormat::Rg8Unorm => wgt::TextureFormat::Rg8Unorm,
            CanvasGPUTextureFormat::Rg8Snorm => wgt::TextureFormat::Rg8Snorm,
            CanvasGPUTextureFormat::Rg8Uint => wgt::TextureFormat::Rg8Uint,
            CanvasGPUTextureFormat::Rg8Sint => wgt::TextureFormat::Rg8Sint,
            CanvasGPUTextureFormat::R32Uint => wgt::TextureFormat::R32Uint,
            CanvasGPUTextureFormat::R32Sint => wgt::TextureFormat::R32Sint,
            CanvasGPUTextureFormat::R32Float => wgt::TextureFormat::R32Float,
            CanvasGPUTextureFormat::Rg16Uint => wgt::TextureFormat::Rg16Uint,
            CanvasGPUTextureFormat::Rg16Sint => wgt::TextureFormat::Rg16Sint,
            CanvasGPUTextureFormat::Rg16Unorm => wgt::TextureFormat::Rg16Unorm,
            CanvasGPUTextureFormat::Rg16Snorm => wgt::TextureFormat::Rg16Snorm,
            CanvasGPUTextureFormat::Rg16Float => wgt::TextureFormat::Rg16Float,
            CanvasGPUTextureFormat::Rgba8UnormSrgb => wgt::TextureFormat::Rgba8UnormSrgb,
            CanvasGPUTextureFormat::Rgba8Snorm => wgt::TextureFormat::Rgba8Snorm,
            CanvasGPUTextureFormat::Rgba8Uint => wgt::TextureFormat::Rgba8Uint,
            CanvasGPUTextureFormat::Rgba8Sint => wgt::TextureFormat::Rgba8Sint,
            CanvasGPUTextureFormat::Bgra8UnormSrgb => wgt::TextureFormat::Bgra8UnormSrgb,
            CanvasGPUTextureFormat::Rgb9e5Ufloat => wgt::TextureFormat::Rgb9e5Ufloat,
            CanvasGPUTextureFormat::Rgb10a2Uint => wgt::TextureFormat::Rgb10a2Uint,
            CanvasGPUTextureFormat::Rgb10a2Unorm => wgt::TextureFormat::Rgb10a2Unorm,
            CanvasGPUTextureFormat::Rg11b10UFloat => wgt::TextureFormat::Rg11b10Ufloat,
            CanvasGPUTextureFormat::Rg32Uint => wgt::TextureFormat::Rg32Uint,
            CanvasGPUTextureFormat::Rg32Sint => wgt::TextureFormat::Rg32Sint,
            CanvasGPUTextureFormat::Rg32Float => wgt::TextureFormat::Rg32Float,
            CanvasGPUTextureFormat::Rgba16Uint => wgt::TextureFormat::Rgba16Uint,
            CanvasGPUTextureFormat::Rgba16Sint => wgt::TextureFormat::Rgba16Sint,
            CanvasGPUTextureFormat::Rgba16Unorm => wgt::TextureFormat::Rgba16Unorm,
            CanvasGPUTextureFormat::Rgba16Snorm => wgt::TextureFormat::Rgba16Snorm,
            CanvasGPUTextureFormat::Rgba16Float => wgt::TextureFormat::Rgba16Float,
            CanvasGPUTextureFormat::Rgba32Uint => wgt::TextureFormat::Rgba32Uint,
            CanvasGPUTextureFormat::Rgba32Sint => wgt::TextureFormat::Rgba32Sint,
            CanvasGPUTextureFormat::Rgba32Float => wgt::TextureFormat::Rgba32Float,
            CanvasGPUTextureFormat::Stencil8 => wgt::TextureFormat::Stencil8,
            CanvasGPUTextureFormat::Depth16Unorm => wgt::TextureFormat::Depth16Unorm,
            CanvasGPUTextureFormat::Depth24Plus => wgt::TextureFormat::Depth24Plus,
            CanvasGPUTextureFormat::Depth24PlusStencil8 => {
                wgt::TextureFormat::Depth24PlusStencil8
            }
            CanvasGPUTextureFormat::Depth32Float => wgt::TextureFormat::Depth32Float,
            CanvasGPUTextureFormat::Depth32FloatStencil8 => {
                wgt::TextureFormat::Depth32FloatStencil8
            }
            CanvasGPUTextureFormat::NV12 => wgt::TextureFormat::NV12,
            CanvasGPUTextureFormat::Bc1RgbaUnorm => wgt::TextureFormat::Bc1RgbaUnorm,
            CanvasGPUTextureFormat::Bc1RgbaUnormSrgb => wgt::TextureFormat::Bc1RgbaUnormSrgb,
            CanvasGPUTextureFormat::Bc2RgbaUnorm => wgt::TextureFormat::Bc2RgbaUnorm,
            CanvasGPUTextureFormat::Bc2RgbaUnormSrgb => wgt::TextureFormat::Bc2RgbaUnormSrgb,
            CanvasGPUTextureFormat::Bc3RgbaUnorm => wgt::TextureFormat::Bc3RgbaUnorm,
            CanvasGPUTextureFormat::Bc3RgbaUnormSrgb => wgt::TextureFormat::Bc3RgbaUnormSrgb,
            CanvasGPUTextureFormat::Bc4RUnorm => wgt::TextureFormat::Bc4RUnorm,
            CanvasGPUTextureFormat::Bc4RSnorm => wgt::TextureFormat::Bc4RSnorm,
            CanvasGPUTextureFormat::Bc5RgUnorm => wgt::TextureFormat::Bc5RgUnorm,
            CanvasGPUTextureFormat::Bc5RgSnorm => wgt::TextureFormat::Bc5RgSnorm,
            CanvasGPUTextureFormat::Bc6hRgbUfloat => wgt::TextureFormat::Bc6hRgbUfloat,
            CanvasGPUTextureFormat::Bc6hRgbFloat => wgt::TextureFormat::Bc6hRgbFloat,
            CanvasGPUTextureFormat::Bc7RgbaUnorm => wgt::TextureFormat::Bc7RgbaUnorm,
            CanvasGPUTextureFormat::Bc7RgbaUnormSrgb => wgt::TextureFormat::Bc7RgbaUnormSrgb,
            CanvasGPUTextureFormat::Etc2Rgb8Unorm => wgt::TextureFormat::Etc2Rgb8Unorm,
            CanvasGPUTextureFormat::Etc2Rgb8UnormSrgb => {
                wgt::TextureFormat::Etc2Rgb8UnormSrgb
            }
            CanvasGPUTextureFormat::Etc2Rgb8A1Unorm => wgt::TextureFormat::Etc2Rgb8A1Unorm,
            CanvasGPUTextureFormat::Etc2Rgb8A1UnormSrgb => {
                wgt::TextureFormat::Etc2Rgb8A1UnormSrgb
            }
            CanvasGPUTextureFormat::Etc2Rgba8Unorm => wgt::TextureFormat::Etc2Rgba8Unorm,
            CanvasGPUTextureFormat::Etc2Rgba8UnormSrgb => {
                wgt::TextureFormat::Etc2Rgba8UnormSrgb
            }
            CanvasGPUTextureFormat::EacR11Unorm => wgt::TextureFormat::EacR11Unorm,
            CanvasGPUTextureFormat::EacR11Snorm => wgt::TextureFormat::EacR11Snorm,
            CanvasGPUTextureFormat::EacRg11Unorm => wgt::TextureFormat::EacRg11Unorm,
            CanvasGPUTextureFormat::EacRg11Snorm => wgt::TextureFormat::EacRg11Snorm,
            CanvasGPUTextureFormat::Astc { block, channel } => wgt::TextureFormat::Astc {
                block: block.into(),
                channel: channel.into(),
            },
        }
    }
}
impl From<Option<wgt::TextureFormat>> for CanvasOptionalGPUTextureFormat {
    fn from(value: Option<wgt::TextureFormat>) -> Self {
        match value {
            Some(value) => {
                let value: CanvasGPUTextureFormat = value.into();
                Self::Some(value)
            }
            None => Self::None,
        }
    }
}

impl Into<Option<wgt::TextureFormat>> for CanvasOptionalGPUTextureFormat {
    fn into(self) -> Option<wgt::TextureFormat> {
        match self {
            CanvasOptionalGPUTextureFormat::None => None,
            CanvasOptionalGPUTextureFormat::Some(value) => {
                let value: wgt::TextureFormat = value.into();
                Some(value)
            }
        }
    }
}

impl Into<String> for CanvasGPUTextureFormat {
    fn into(self) -> String {
        let s: String;
        let name = match self {
            CanvasGPUTextureFormat::R8Unorm => "r8unorm",
            CanvasGPUTextureFormat::R8Snorm => "r8snorm",
            CanvasGPUTextureFormat::R8Uint => "r8uint",
            CanvasGPUTextureFormat::R8Sint => "r8sint",
            CanvasGPUTextureFormat::R16Uint => "r16uint",
            CanvasGPUTextureFormat::R16Sint => "r16sint",
            CanvasGPUTextureFormat::R16Unorm => "r16unorm",
            CanvasGPUTextureFormat::R16Snorm => "r16snorm",
            CanvasGPUTextureFormat::R16Float => "r16float",
            CanvasGPUTextureFormat::Rg8Unorm => "rg8unorm",
            CanvasGPUTextureFormat::Rg8Snorm => "rg8snorm",
            CanvasGPUTextureFormat::Rg8Uint => "rg8uint",
            CanvasGPUTextureFormat::Rg8Sint => "rg8sint",
            CanvasGPUTextureFormat::R32Uint => "r32uint",
            CanvasGPUTextureFormat::R32Sint => "r32sint",
            CanvasGPUTextureFormat::R32Float => "r32float",
            CanvasGPUTextureFormat::Rg16Uint => "rg16uint",
            CanvasGPUTextureFormat::Rg16Sint => "rg16sint",
            CanvasGPUTextureFormat::Rg16Unorm => "rg16unorm",
            CanvasGPUTextureFormat::Rg16Snorm => "rg16snorm",
            CanvasGPUTextureFormat::Rg16Float => "rg16float",
            CanvasGPUTextureFormat::Rgba8Unorm => "rgba8unorm",
            CanvasGPUTextureFormat::Rgba8UnormSrgb => "rgba8unorm-srgb",
            CanvasGPUTextureFormat::Rgba8Snorm => "rgba8snorm",
            CanvasGPUTextureFormat::Rgba8Uint => "rgba8uint",
            CanvasGPUTextureFormat::Rgba8Sint => "rgba8sint",
            CanvasGPUTextureFormat::Bgra8Unorm => "bgra8unorm",
            CanvasGPUTextureFormat::Bgra8UnormSrgb => "bgra8unorm-srgb",
            CanvasGPUTextureFormat::Rgb10a2Uint => "rgb10a2uint",
            CanvasGPUTextureFormat::Rgb10a2Unorm => "rgb10a2unorm",
            CanvasGPUTextureFormat::Rg11b10UFloat => "rg11b10ufloat",
            CanvasGPUTextureFormat::Rg32Uint => "rg32uint",
            CanvasGPUTextureFormat::Rg32Sint => "rg32sint",
            CanvasGPUTextureFormat::Rg32Float => "rg32float",
            CanvasGPUTextureFormat::Rgba16Uint => "rgba16uint",
            CanvasGPUTextureFormat::Rgba16Sint => "rgba16sint",
            CanvasGPUTextureFormat::Rgba16Unorm => "rgba16unorm",
            CanvasGPUTextureFormat::Rgba16Snorm => "rgba16snorm",
            CanvasGPUTextureFormat::Rgba16Float => "rgba16float",
            CanvasGPUTextureFormat::Rgba32Uint => "rgba32uint",
            CanvasGPUTextureFormat::Rgba32Sint => "rgba32sint",
            CanvasGPUTextureFormat::Rgba32Float => "rgba32float",
            CanvasGPUTextureFormat::Stencil8 => "stencil8",
            CanvasGPUTextureFormat::Depth32Float => "depth32float",
            CanvasGPUTextureFormat::Depth16Unorm => "depth16unorm",
            CanvasGPUTextureFormat::Depth32FloatStencil8 => "depth32float-stencil8",
            CanvasGPUTextureFormat::Depth24Plus => "depth24plus",
            CanvasGPUTextureFormat::Depth24PlusStencil8 => "depth24plus-stencil8",
            CanvasGPUTextureFormat::NV12 => "nv12",
            CanvasGPUTextureFormat::Rgb9e5Ufloat => "rgb9e5ufloat",
            CanvasGPUTextureFormat::Bc1RgbaUnorm => "bc1-rgba-unorm",
            CanvasGPUTextureFormat::Bc1RgbaUnormSrgb => "bc1-rgba-unorm-srgb",
            CanvasGPUTextureFormat::Bc2RgbaUnorm => "bc2-rgba-unorm",
            CanvasGPUTextureFormat::Bc2RgbaUnormSrgb => "bc2-rgba-unorm-srgb",
            CanvasGPUTextureFormat::Bc3RgbaUnorm => "bc3-rgba-unorm",
            CanvasGPUTextureFormat::Bc3RgbaUnormSrgb => "bc3-rgba-unorm-srgb",
            CanvasGPUTextureFormat::Bc4RUnorm => "bc4-r-unorm",
            CanvasGPUTextureFormat::Bc4RSnorm => "bc4-r-snorm",
            CanvasGPUTextureFormat::Bc5RgUnorm => "bc5-rg-unorm",
            CanvasGPUTextureFormat::Bc5RgSnorm => "bc5-rg-snorm",
            CanvasGPUTextureFormat::Bc6hRgbUfloat => "bc6h-rgb-ufloat",
            CanvasGPUTextureFormat::Bc6hRgbFloat => "bc6h-rgb-float",
            CanvasGPUTextureFormat::Bc7RgbaUnorm => "bc7-rgba-unorm",
            CanvasGPUTextureFormat::Bc7RgbaUnormSrgb => "bc7-rgba-unorm-srgb",
            CanvasGPUTextureFormat::Etc2Rgb8Unorm => "etc2-rgb8unorm",
            CanvasGPUTextureFormat::Etc2Rgb8UnormSrgb => "etc2-rgb8unorm-srgb",
            CanvasGPUTextureFormat::Etc2Rgb8A1Unorm => "etc2-rgb8a1unorm",
            CanvasGPUTextureFormat::Etc2Rgb8A1UnormSrgb => "etc2-rgb8a1unorm-srgb",
            CanvasGPUTextureFormat::Etc2Rgba8Unorm => "etc2-rgba8unorm",
            CanvasGPUTextureFormat::Etc2Rgba8UnormSrgb => "etc2-rgba8unorm-srgb",
            CanvasGPUTextureFormat::EacR11Unorm => "eac-r11unorm",
            CanvasGPUTextureFormat::EacR11Snorm => "eac-r11snorm",
            CanvasGPUTextureFormat::EacRg11Unorm => "eac-rg11unorm",
            CanvasGPUTextureFormat::EacRg11Snorm => "eac-rg11snorm",
            CanvasGPUTextureFormat::Astc { block, channel } => {
                let block = match block {
                    CanvasAstcBlock::B4x4 => "4x4",
                    CanvasAstcBlock::B5x4 => "5x4",
                    CanvasAstcBlock::B5x5 => "5x5",
                    CanvasAstcBlock::B6x5 => "6x5",
                    CanvasAstcBlock::B6x6 => "6x6",
                    CanvasAstcBlock::B8x5 => "8x5",
                    CanvasAstcBlock::B8x6 => "8x6",
                    CanvasAstcBlock::B8x8 => "8x8",
                    CanvasAstcBlock::B10x5 => "10x5",
                    CanvasAstcBlock::B10x6 => "10x6",
                    CanvasAstcBlock::B10x8 => "10x8",
                    CanvasAstcBlock::B10x10 => "10x10",
                    CanvasAstcBlock::B12x10 => "12x10",
                    CanvasAstcBlock::B12x12 => "12x12",
                };

                let channel = match channel {
                    CanvasAstcChannel::Unorm => "unorm",
                    CanvasAstcChannel::UnormSrgb => "unorm-srgb",
                    CanvasAstcChannel::Hdr => "hdr",
                };

                s = format!("astc-{block}-{channel}");
                &s
            }
        };
        name.to_string()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_enum_gpu_texture_to_string(
    value: CanvasGPUTextureFormat,
) -> *mut c_char {
    let s: String;
    let name = match value {
        CanvasGPUTextureFormat::R8Unorm => "r8unorm",
        CanvasGPUTextureFormat::R8Snorm => "r8snorm",
        CanvasGPUTextureFormat::R8Uint => "r8uint",
        CanvasGPUTextureFormat::R8Sint => "r8sint",
        CanvasGPUTextureFormat::R16Uint => "r16uint",
        CanvasGPUTextureFormat::R16Sint => "r16sint",
        CanvasGPUTextureFormat::R16Unorm => "r16unorm",
        CanvasGPUTextureFormat::R16Snorm => "r16snorm",
        CanvasGPUTextureFormat::R16Float => "r16float",
        CanvasGPUTextureFormat::Rg8Unorm => "rg8unorm",
        CanvasGPUTextureFormat::Rg8Snorm => "rg8snorm",
        CanvasGPUTextureFormat::Rg8Uint => "rg8uint",
        CanvasGPUTextureFormat::Rg8Sint => "rg8sint",
        CanvasGPUTextureFormat::R32Uint => "r32uint",
        CanvasGPUTextureFormat::R32Sint => "r32sint",
        CanvasGPUTextureFormat::R32Float => "r32float",
        CanvasGPUTextureFormat::Rg16Uint => "rg16uint",
        CanvasGPUTextureFormat::Rg16Sint => "rg16sint",
        CanvasGPUTextureFormat::Rg16Unorm => "rg16unorm",
        CanvasGPUTextureFormat::Rg16Snorm => "rg16snorm",
        CanvasGPUTextureFormat::Rg16Float => "rg16float",
        CanvasGPUTextureFormat::Rgba8Unorm => "rgba8unorm",
        CanvasGPUTextureFormat::Rgba8UnormSrgb => "rgba8unorm-srgb",
        CanvasGPUTextureFormat::Rgba8Snorm => "rgba8snorm",
        CanvasGPUTextureFormat::Rgba8Uint => "rgba8uint",
        CanvasGPUTextureFormat::Rgba8Sint => "rgba8sint",
        CanvasGPUTextureFormat::Bgra8Unorm => "bgra8unorm",
        CanvasGPUTextureFormat::Bgra8UnormSrgb => "bgra8unorm-srgb",
        CanvasGPUTextureFormat::Rgb10a2Uint => "rgb10a2uint",
        CanvasGPUTextureFormat::Rgb10a2Unorm => "rgb10a2unorm",
        CanvasGPUTextureFormat::Rg11b10UFloat => "rg11b10ufloat",
        CanvasGPUTextureFormat::Rg32Uint => "rg32uint",
        CanvasGPUTextureFormat::Rg32Sint => "rg32sint",
        CanvasGPUTextureFormat::Rg32Float => "rg32float",
        CanvasGPUTextureFormat::Rgba16Uint => "rgba16uint",
        CanvasGPUTextureFormat::Rgba16Sint => "rgba16sint",
        CanvasGPUTextureFormat::Rgba16Unorm => "rgba16unorm",
        CanvasGPUTextureFormat::Rgba16Snorm => "rgba16snorm",
        CanvasGPUTextureFormat::Rgba16Float => "rgba16float",
        CanvasGPUTextureFormat::Rgba32Uint => "rgba32uint",
        CanvasGPUTextureFormat::Rgba32Sint => "rgba32sint",
        CanvasGPUTextureFormat::Rgba32Float => "rgba32float",
        CanvasGPUTextureFormat::Stencil8 => "stencil8",
        CanvasGPUTextureFormat::Depth32Float => "depth32float",
        CanvasGPUTextureFormat::Depth16Unorm => "depth16unorm",
        CanvasGPUTextureFormat::Depth32FloatStencil8 => "depth32float-stencil8",
        CanvasGPUTextureFormat::Depth24Plus => "depth24plus",
        CanvasGPUTextureFormat::Depth24PlusStencil8 => "depth24plus-stencil8",
        CanvasGPUTextureFormat::NV12 => "nv12",
        CanvasGPUTextureFormat::Rgb9e5Ufloat => "rgb9e5ufloat",
        CanvasGPUTextureFormat::Bc1RgbaUnorm => "bc1-rgba-unorm",
        CanvasGPUTextureFormat::Bc1RgbaUnormSrgb => "bc1-rgba-unorm-srgb",
        CanvasGPUTextureFormat::Bc2RgbaUnorm => "bc2-rgba-unorm",
        CanvasGPUTextureFormat::Bc2RgbaUnormSrgb => "bc2-rgba-unorm-srgb",
        CanvasGPUTextureFormat::Bc3RgbaUnorm => "bc3-rgba-unorm",
        CanvasGPUTextureFormat::Bc3RgbaUnormSrgb => "bc3-rgba-unorm-srgb",
        CanvasGPUTextureFormat::Bc4RUnorm => "bc4-r-unorm",
        CanvasGPUTextureFormat::Bc4RSnorm => "bc4-r-snorm",
        CanvasGPUTextureFormat::Bc5RgUnorm => "bc5-rg-unorm",
        CanvasGPUTextureFormat::Bc5RgSnorm => "bc5-rg-snorm",
        CanvasGPUTextureFormat::Bc6hRgbUfloat => "bc6h-rgb-ufloat",
        CanvasGPUTextureFormat::Bc6hRgbFloat => "bc6h-rgb-float",
        CanvasGPUTextureFormat::Bc7RgbaUnorm => "bc7-rgba-unorm",
        CanvasGPUTextureFormat::Bc7RgbaUnormSrgb => "bc7-rgba-unorm-srgb",
        CanvasGPUTextureFormat::Etc2Rgb8Unorm => "etc2-rgb8unorm",
        CanvasGPUTextureFormat::Etc2Rgb8UnormSrgb => "etc2-rgb8unorm-srgb",
        CanvasGPUTextureFormat::Etc2Rgb8A1Unorm => "etc2-rgb8a1unorm",
        CanvasGPUTextureFormat::Etc2Rgb8A1UnormSrgb => "etc2-rgb8a1unorm-srgb",
        CanvasGPUTextureFormat::Etc2Rgba8Unorm => "etc2-rgba8unorm",
        CanvasGPUTextureFormat::Etc2Rgba8UnormSrgb => "etc2-rgba8unorm-srgb",
        CanvasGPUTextureFormat::EacR11Unorm => "eac-r11unorm",
        CanvasGPUTextureFormat::EacR11Snorm => "eac-r11snorm",
        CanvasGPUTextureFormat::EacRg11Unorm => "eac-rg11unorm",
        CanvasGPUTextureFormat::EacRg11Snorm => "eac-rg11snorm",
        CanvasGPUTextureFormat::Astc { block, channel } => {
            let block = match block {
                CanvasAstcBlock::B4x4 => "4x4",
                CanvasAstcBlock::B5x4 => "5x4",
                CanvasAstcBlock::B5x5 => "5x5",
                CanvasAstcBlock::B6x5 => "6x5",
                CanvasAstcBlock::B6x6 => "6x6",
                CanvasAstcBlock::B8x5 => "8x5",
                CanvasAstcBlock::B8x6 => "8x6",
                CanvasAstcBlock::B8x8 => "8x8",
                CanvasAstcBlock::B10x5 => "10x5",
                CanvasAstcBlock::B10x6 => "10x6",
                CanvasAstcBlock::B10x8 => "10x8",
                CanvasAstcBlock::B10x10 => "10x10",
                CanvasAstcBlock::B12x10 => "12x10",
                CanvasAstcBlock::B12x12 => "12x12",
            };

            let channel = match channel {
                CanvasAstcChannel::Unorm => "unorm",
                CanvasAstcChannel::UnormSrgb => "unorm-srgb",
                CanvasAstcChannel::Hdr => "hdr",
            };

            s = format!("astc-{block}-{channel}");
            &s
        }
    };
    CString::new(name).unwrap().into_raw()
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum CanvasOptionalGPUTextureFormat {
    None,
    Some(CanvasGPUTextureFormat),
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_enum_string_to_gpu_texture(
    format: *const c_char,
) -> CanvasOptionalGPUTextureFormat {
    let format = CStr::from_ptr(format);
    let format = format.to_string_lossy();
    let format = match format.as_ref() {
        "r8unorm" => CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::R8Unorm),
        "r8snorm" => CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::R8Snorm),
        "r8uint" => CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::R8Uint),
        "r8sint" => CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::R8Sint),
        "r16uint" => CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::R16Uint),
        "r16sint" => CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::R16Sint),
        "r16unorm" => CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::R16Unorm),
        "r16snorm" => CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::R16Snorm),
        "r16float" => CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::R16Float),
        "rg8unorm" => CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::Rg8Unorm),
        "rg8snorm" => CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::Rg8Snorm),
        "rg8uint" => CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::Rg8Uint),
        "rg8sint" => CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::Rg8Sint),
        "r32uint" => CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::R32Uint),
        "r32sint" => CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::R32Sint),
        "r32float" => CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::R32Float),
        "rg16uint" => CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::Rg16Uint),
        "rg16sint" => CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::Rg16Sint),
        "rg16unorm" => CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::Rg16Unorm),
        "rg16snorm" => CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::Rg16Snorm),
        "rg16float" => CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::Rg16Float),
        "rgba8unorm" => CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::Rgba8Unorm),
        "rgba8unorm-srgb" => {
            CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::Rgba8UnormSrgb)
        }
        "rgba8snorm" => CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::Rgba8Snorm),
        "rgba8uint" => CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::Rgba8Uint),
        "rgba8sint" => CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::Rgba8Sint),
        "bgra8unorm" => CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::Bgra8Unorm),
        "bgra8unorm-srgb" => {
            CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::Bgra8UnormSrgb)
        }
        "rgb10a2uint" => CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::Rgb10a2Uint),
        "rgb10a2unorm" => {
            CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::Rgb10a2Unorm)
        }
        "rg11b10ufloat" => {
            CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::Rg11b10UFloat)
        }
        "rg32uint" => CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::Rg32Uint),
        "rg32sint" => CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::Rg32Sint),
        "rg32float" => CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::Rg32Float),
        "rgba16uint" => CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::Rgba16Uint),
        "rgba16sint" => CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::Rgba16Sint),
        "rgba16unorm" => CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::Rgba16Unorm),
        "rgba16snorm" => CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::Rgba16Snorm),
        "rgba16float" => CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::Rgba16Float),
        "rgba32uint" => CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::Rgba32Uint),
        "rgba32sint" => CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::Rgba32Sint),
        "rgba32float" => CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::Rgba32Float),
        "stencil8" => CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::Stencil8),
        "depth32float" => {
            CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::Depth32Float)
        }
        "depth32float-stencil8" => {
            CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::Depth32FloatStencil8)
        }
        "depth16unorm" => {
            CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::Depth16Unorm)
        }
        "depth24plus" => CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::Depth24Plus),
        "depth24plus-stencil8" => {
            CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::Depth24PlusStencil8)
        }
        "nv12" => CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::NV12),
        "rgb9e5ufloat" => {
            CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::Rgb9e5Ufloat)
        }
        "bc1-rgba-unorm" => {
            CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::Bc1RgbaUnorm)
        }
        "bc1-rgba-unorm-srgb" => {
            CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::Bc1RgbaUnormSrgb)
        }
        "bc2-rgba-unorm" => {
            CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::Bc2RgbaUnorm)
        }
        "bc2-rgba-unorm-srgb" => {
            CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::Bc2RgbaUnormSrgb)
        }
        "bc3-rgba-unorm" => {
            CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::Bc3RgbaUnorm)
        }
        "bc3-rgba-unorm-srgb" => {
            CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::Bc3RgbaUnormSrgb)
        }
        "bc4-r-unorm" => CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::Bc4RUnorm),
        "bc4-r-snorm" => CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::Bc4RSnorm),
        "bc5-rg-unorm" => CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::Bc5RgUnorm),
        "bc5-rg-snorm" => CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::Bc5RgSnorm),
        "bc6h-rgb-ufloat" => {
            CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::Bc6hRgbUfloat)
        }
        "bc6h-rgb-float" => {
            CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::Bc6hRgbFloat)
        }
        "bc7-rgba-unorm" => {
            CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::Bc7RgbaUnorm)
        }
        "bc7-rgba-unorm-srgb" => {
            CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::Bc7RgbaUnormSrgb)
        }
        "etc2-rgb8unorm" => {
            CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::Etc2Rgb8Unorm)
        }
        "etc2-rgb8unorm-srgb" => {
            CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::Etc2Rgb8UnormSrgb)
        }
        "etc2-rgb8a1unorm" => {
            CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::Etc2Rgb8A1Unorm)
        }
        "etc2-rgb8a1unorm-srgb" => {
            CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::Etc2Rgb8A1UnormSrgb)
        }
        "etc2-rgba8unorm" => {
            CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::Etc2Rgba8Unorm)
        }
        "etc2-rgba8unorm-srgb" => {
            CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::Etc2Rgba8UnormSrgb)
        }
        "eac-r11unorm" => CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::EacR11Unorm),
        "eac-r11snorm" => CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::EacR11Snorm),
        "eac-rg11unorm" => {
            CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::EacRg11Unorm)
        }
        "eac-rg11snorm" => {
            CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::EacRg11Snorm)
        }
        other => {
            if let Some(parts) = other.strip_prefix("astc-") {
                if let Some((block, channel)) = parts.split_once('-') {
                    let block = match block {
                        "4x4" => CanvasAstcBlock::B4x4,
                        "5x4" => CanvasAstcBlock::B5x4,
                        "5x5" => CanvasAstcBlock::B5x5,
                        "6x5" => CanvasAstcBlock::B6x5,
                        "6x6" => CanvasAstcBlock::B6x6,
                        "8x5" => CanvasAstcBlock::B8x5,
                        "8x6" => CanvasAstcBlock::B8x6,
                        "8x8" => CanvasAstcBlock::B8x8,
                        "10x5" => CanvasAstcBlock::B10x5,
                        "10x6" => CanvasAstcBlock::B10x6,
                        "10x8" => CanvasAstcBlock::B10x8,
                        "10x10" => CanvasAstcBlock::B10x10,
                        "12x10" => CanvasAstcBlock::B12x10,
                        "12x12" => CanvasAstcBlock::B12x12,
                        _ => {
                            return CanvasOptionalGPUTextureFormat::None;
                        }
                    };

                    let channel = match channel {
                        "unorm" => CanvasAstcChannel::Unorm,
                        "unorm-srgb" => CanvasAstcChannel::UnormSrgb,
                        "hdr" => CanvasAstcChannel::Hdr,
                        _ => {
                            return CanvasOptionalGPUTextureFormat::None;
                        }
                    };

                    return CanvasOptionalGPUTextureFormat::Some(CanvasGPUTextureFormat::Astc {
                        block,
                        channel,
                    });
                }

                CanvasOptionalGPUTextureFormat::None
            } else {
                CanvasOptionalGPUTextureFormat::None
            }
        }
    };

    return format;
}

/// ASTC block dimensions
#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum CanvasAstcBlock {
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px).
    B4x4,
    /// 5x4 block compressed texture. 16 bytes per block (6.4 bit/px).
    B5x4,
    /// 5x5 block compressed texture. 16 bytes per block (5.12 bit/px).
    B5x5,
    /// 6x5 block compressed texture. 16 bytes per block (4.27 bit/px).
    B6x5,
    /// 6x6 block compressed texture. 16 bytes per block (3.56 bit/px).
    B6x6,
    /// 8x5 block compressed texture. 16 bytes per block (3.2 bit/px).
    B8x5,
    /// 8x6 block compressed texture. 16 bytes per block (2.67 bit/px).
    B8x6,
    /// 8x8 block compressed texture. 16 bytes per block (2 bit/px).
    B8x8,
    /// 10x5 block compressed texture. 16 bytes per block (2.56 bit/px).
    B10x5,
    /// 10x6 block compressed texture. 16 bytes per block (2.13 bit/px).
    B10x6,
    /// 10x8 block compressed texture. 16 bytes per block (1.6 bit/px).
    B10x8,
    /// 10x10 block compressed texture. 16 bytes per block (1.28 bit/px).
    B10x10,
    /// 12x10 block compressed texture. 16 bytes per block (1.07 bit/px).
    B12x10,
    /// 12x12 block compressed texture. 16 bytes per block (0.89 bit/px).
    B12x12,
}

/// ASTC RGBA channel
#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum CanvasAstcChannel {
    /// 8 bit integer RGBA, [0, 255] converted to/from linear-color float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ASTC`] must be enabled to use this channel.
    Unorm,
    /// 8 bit integer RGBA, Srgb-color [0, 255] converted to/from linear-color float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ASTC`] must be enabled to use this channel.
    UnormSrgb,
    /// floating-point RGBA, linear-color float can be outside of the [0, 1] range.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ASTC_HDR`] must be enabled to use this channel.
    Hdr,
}

impl From<wgt::AstcChannel> for CanvasAstcChannel {
    fn from(value: wgt::AstcChannel) -> Self {
        match value {
            wgt::AstcChannel::Unorm => Self::Unorm,
            wgt::AstcChannel::UnormSrgb => Self::UnormSrgb,
            wgt::AstcChannel::Hdr => Self::Hdr,
        }
    }
}

impl Into<wgt::AstcChannel> for CanvasAstcChannel {
    fn into(self) -> wgt::AstcChannel {
        match self {
            CanvasAstcChannel::Unorm => wgt::AstcChannel::Unorm,
            CanvasAstcChannel::UnormSrgb => wgt::AstcChannel::UnormSrgb,
            CanvasAstcChannel::Hdr => wgt::AstcChannel::Hdr,
        }
    }
}

impl From<wgt::AstcBlock> for CanvasAstcBlock {
    fn from(value: wgt::AstcBlock) -> Self {
        match value {
            wgt::AstcBlock::B4x4 => Self::B4x4,
            wgt::AstcBlock::B5x4 => Self::B5x4,
            wgt::AstcBlock::B5x5 => Self::B5x5,
            wgt::AstcBlock::B6x5 => Self::B6x5,
            wgt::AstcBlock::B6x6 => Self::B6x6,
            wgt::AstcBlock::B8x5 => Self::B8x5,
            wgt::AstcBlock::B8x6 => Self::B8x6,
            wgt::AstcBlock::B8x8 => Self::B8x8,
            wgt::AstcBlock::B10x5 => Self::B10x5,
            wgt::AstcBlock::B10x6 => Self::B10x6,
            wgt::AstcBlock::B10x8 => Self::B10x8,
            wgt::AstcBlock::B10x10 => Self::B10x10,
            wgt::AstcBlock::B12x10 => Self::B12x10,
            wgt::AstcBlock::B12x12 => Self::B12x12,
        }
    }
}

impl Into<wgt::AstcBlock> for CanvasAstcBlock {
    fn into(self) -> wgt::AstcBlock {
        match self {
            CanvasAstcBlock::B4x4 => wgt::AstcBlock::B4x4,
            CanvasAstcBlock::B5x4 => wgt::AstcBlock::B5x4,
            CanvasAstcBlock::B5x5 => wgt::AstcBlock::B5x5,
            CanvasAstcBlock::B6x5 => wgt::AstcBlock::B6x5,
            CanvasAstcBlock::B6x6 => wgt::AstcBlock::B6x6,
            CanvasAstcBlock::B8x5 => wgt::AstcBlock::B8x5,
            CanvasAstcBlock::B8x6 => wgt::AstcBlock::B8x6,
            CanvasAstcBlock::B8x8 => wgt::AstcBlock::B8x8,
            CanvasAstcBlock::B10x5 => wgt::AstcBlock::B10x5,
            CanvasAstcBlock::B10x6 => wgt::AstcBlock::B10x6,
            CanvasAstcBlock::B10x8 => wgt::AstcBlock::B10x8,
            CanvasAstcBlock::B10x10 => wgt::AstcBlock::B10x10,
            CanvasAstcBlock::B12x10 => wgt::AstcBlock::B12x10,
            CanvasAstcBlock::B12x12 => wgt::AstcBlock::B12x12,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum CanvasTextureAspect {
    /// Depth, Stencil, and Color.
    All,
    /// Stencil.
    StencilOnly,
    /// Depth.
    DepthOnly,
    /// Plane 0.
    Plane0,
    /// Plane 1.
    Plane1,
    /// Plane 2.
    Plane2,
}

impl From<wgt::TextureAspect> for CanvasTextureAspect {
    fn from(value: wgt::TextureAspect) -> Self {
        match value {
            wgt::TextureAspect::All => CanvasTextureAspect::All,
            wgt::TextureAspect::StencilOnly => CanvasTextureAspect::StencilOnly,
            wgt::TextureAspect::DepthOnly => CanvasTextureAspect::DepthOnly,
            wgt::TextureAspect::Plane0 => CanvasTextureAspect::Plane0,
            wgt::TextureAspect::Plane1 => CanvasTextureAspect::Plane1,
            wgt::TextureAspect::Plane2 => CanvasTextureAspect::Plane2,
        }
    }
}

impl Into<wgt::TextureAspect> for CanvasTextureAspect {
    fn into(self) -> wgt::TextureAspect {
        match self {
            CanvasTextureAspect::All => wgt::TextureAspect::All,
            CanvasTextureAspect::StencilOnly => wgt::TextureAspect::StencilOnly,
            CanvasTextureAspect::DepthOnly => wgt::TextureAspect::DepthOnly,
            CanvasTextureAspect::Plane0 => wgt::TextureAspect::Plane0,
            CanvasTextureAspect::Plane1 => wgt::TextureAspect::Plane1,
            CanvasTextureAspect::Plane2 => wgt::TextureAspect::Plane2,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum CanvasIndexFormat {
    /// Indices are 16 bit unsigned integers.
    Uint16 = 0,
    /// Indices are 32 bit unsigned integers.
    Uint32 = 1,
}

impl From<wgt::IndexFormat> for CanvasIndexFormat {
    fn from(value: wgt::IndexFormat) -> Self {
        match value {
            wgt::IndexFormat::Uint16 => Self::Uint16,
            wgt::IndexFormat::Uint32 => Self::Uint32,
        }
    }
}

impl Into<wgt::IndexFormat> for CanvasIndexFormat {
    fn into(self) -> wgt::IndexFormat {
        match self {
            CanvasIndexFormat::Uint16 => wgt::IndexFormat::Uint16,
            CanvasIndexFormat::Uint32 => wgt::IndexFormat::Uint32,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum CanvasOptionalIndexFormat {
    None,
    Some(CanvasIndexFormat),
}

impl From<Option<wgt::IndexFormat>> for CanvasOptionalIndexFormat {
    fn from(value: Option<wgt::IndexFormat>) -> Self {
        match value {
            Some(value) => Self::Some(match value {
                wgt::IndexFormat::Uint16 => CanvasIndexFormat::Uint16,
                wgt::IndexFormat::Uint32 => CanvasIndexFormat::Uint32,
            }),
            None => Self::None,
        }
    }
}

impl Into<Option<wgt::IndexFormat>> for CanvasOptionalIndexFormat {
    fn into(self) -> Option<wgt::IndexFormat> {
        match self {
            CanvasOptionalIndexFormat::None => None,
            CanvasOptionalIndexFormat::Some(value) => Some(match value {
                CanvasIndexFormat::Uint16 => wgt::IndexFormat::Uint16,
                CanvasIndexFormat::Uint32 => wgt::IndexFormat::Uint32,
            }),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum CanvasVertexStepMode {
    Vertex = 0,
    Instance = 1,
}

impl From<wgt::VertexStepMode> for CanvasVertexStepMode {
    fn from(value: wgt::VertexStepMode) -> Self {
        match value {
            wgt::VertexStepMode::Vertex => Self::Vertex,
            wgt::VertexStepMode::Instance => Self::Instance,
        }
    }
}

impl Into<wgt::VertexStepMode> for CanvasVertexStepMode {
    fn into(self) -> wgt::VertexStepMode {
        match self {
            CanvasVertexStepMode::Vertex => wgt::VertexStepMode::Vertex,
            CanvasVertexStepMode::Instance => wgt::VertexStepMode::Instance,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum CanvasVertexFormat {
    Uint8x2 = 0,
    Uint8x4 = 1,
    Sint8x2 = 2,
    Sint8x4 = 3,
    Unorm8x2 = 4,
    Unorm8x4 = 5,
    Snorm8x2 = 6,
    Snorm8x4 = 7,
    Uint16x2 = 8,
    Uint16x4 = 9,
    Sint16x2 = 10,
    Sint16x4 = 11,
    Unorm16x2 = 12,
    Unorm16x4 = 13,
    Snorm16x2 = 14,
    Snorm16x4 = 15,
    Float16x2 = 16,
    Float16x4 = 17,
    Float32 = 18,
    Float32x2 = 19,
    Float32x3 = 20,
    Float32x4 = 21,
    Uint32 = 22,
    Uint32x2 = 23,
    Uint32x3 = 24,
    Uint32x4 = 25,
    Sint32 = 26,
    Sint32x2 = 27,
    Sint32x3 = 28,
    Sint32x4 = 29,
    Float64 = 30,
    Float64x2 = 31,
    Float64x3 = 32,
    Float64x4 = 33,
    Unorm10_10_10_2 = 34,
}

impl CanvasVertexFormat {
    pub const fn size(&self) -> u64 {
        match self {
            Self::Uint8x2 | Self::Sint8x2 | Self::Unorm8x2 | Self::Snorm8x2 => 2,
            Self::Uint8x4
            | Self::Sint8x4
            | Self::Unorm8x4
            | Self::Snorm8x4
            | Self::Uint16x2
            | Self::Sint16x2
            | Self::Unorm16x2
            | Self::Snorm16x2
            | Self::Float16x2
            | Self::Float32
            | Self::Uint32
            | Self::Sint32
            | Self::Unorm10_10_10_2 => 4,
            Self::Uint16x4
            | Self::Sint16x4
            | Self::Unorm16x4
            | Self::Snorm16x4
            | Self::Float16x4
            | Self::Float32x2
            | Self::Uint32x2
            | Self::Sint32x2
            | Self::Float64 => 8,
            Self::Float32x3 | Self::Uint32x3 | Self::Sint32x3 => 12,
            Self::Float32x4 | Self::Uint32x4 | Self::Sint32x4 | Self::Float64x2 => 16,
            Self::Float64x3 => 24,
            Self::Float64x4 => 32,
        }
    }
}

impl From<wgt::VertexFormat> for CanvasVertexFormat {
    fn from(value: wgt::VertexFormat) -> Self {
        match value {
            wgt::VertexFormat::Uint8x2 => Self::Uint8x2,
            wgt::VertexFormat::Uint8x4 => Self::Uint8x4,
            wgt::VertexFormat::Sint8x2 => Self::Sint8x2,
            wgt::VertexFormat::Sint8x4 => Self::Sint8x4,
            wgt::VertexFormat::Unorm8x2 => Self::Unorm8x2,
            wgt::VertexFormat::Unorm8x4 => Self::Unorm8x4,
            wgt::VertexFormat::Snorm8x2 => Self::Snorm8x2,
            wgt::VertexFormat::Snorm8x4 => Self::Snorm8x4,
            wgt::VertexFormat::Uint16x2 => Self::Uint16x2,
            wgt::VertexFormat::Uint16x4 => Self::Uint16x4,
            wgt::VertexFormat::Sint16x2 => Self::Sint16x2,
            wgt::VertexFormat::Sint16x4 => Self::Sint16x4,
            wgt::VertexFormat::Unorm16x2 => Self::Unorm16x2,
            wgt::VertexFormat::Unorm16x4 => Self::Unorm16x4,
            wgt::VertexFormat::Snorm16x2 => Self::Snorm16x2,
            wgt::VertexFormat::Snorm16x4 => Self::Snorm16x4,
            wgt::VertexFormat::Float16x2 => Self::Float16x2,
            wgt::VertexFormat::Float16x4 => Self::Float16x4,
            wgt::VertexFormat::Float32 => Self::Float32,
            wgt::VertexFormat::Float32x2 => Self::Float32x2,
            wgt::VertexFormat::Float32x3 => Self::Float32x3,
            wgt::VertexFormat::Float32x4 => Self::Float32x4,
            wgt::VertexFormat::Uint32 => Self::Uint32,
            wgt::VertexFormat::Uint32x2 => Self::Uint32x2,
            wgt::VertexFormat::Uint32x3 => Self::Uint32x3,
            wgt::VertexFormat::Uint32x4 => Self::Uint32x4,
            wgt::VertexFormat::Sint32 => Self::Sint32,
            wgt::VertexFormat::Sint32x2 => Self::Sint32x2,
            wgt::VertexFormat::Sint32x3 => Self::Sint32x3,
            wgt::VertexFormat::Sint32x4 => Self::Sint32x4,
            wgt::VertexFormat::Float64 => Self::Float64,
            wgt::VertexFormat::Float64x2 => Self::Float64x2,
            wgt::VertexFormat::Float64x3 => Self::Float64x3,
            wgt::VertexFormat::Float64x4 => Self::Float64x4,
            wgt::VertexFormat::Unorm10_10_10_2 => Self::Unorm10_10_10_2,
        }
    }
}

impl Into<wgt::VertexFormat> for CanvasVertexFormat {
    fn into(self) -> wgt::VertexFormat {
        match self {
            Self::Uint8x2 => wgt::VertexFormat::Uint8x2,
            Self::Uint8x4 => wgt::VertexFormat::Uint8x4,
            Self::Sint8x2 => wgt::VertexFormat::Sint8x2,
            Self::Sint8x4 => wgt::VertexFormat::Sint8x4,
            Self::Unorm8x2 => wgt::VertexFormat::Unorm8x2,
            Self::Unorm8x4 => wgt::VertexFormat::Unorm8x4,
            Self::Snorm8x2 => wgt::VertexFormat::Snorm8x2,
            Self::Snorm8x4 => wgt::VertexFormat::Snorm8x4,
            Self::Uint16x2 => wgt::VertexFormat::Uint16x2,
            Self::Uint16x4 => wgt::VertexFormat::Uint16x4,
            Self::Sint16x2 => wgt::VertexFormat::Sint16x2,
            Self::Sint16x4 => wgt::VertexFormat::Sint16x4,
            Self::Unorm16x2 => wgt::VertexFormat::Unorm16x2,
            Self::Unorm16x4 => wgt::VertexFormat::Unorm16x4,
            Self::Snorm16x2 => wgt::VertexFormat::Snorm16x2,
            Self::Snorm16x4 => wgt::VertexFormat::Snorm16x4,
            Self::Float16x2 => wgt::VertexFormat::Float16x2,
            Self::Float16x4 => wgt::VertexFormat::Float16x4,
            Self::Float32 => wgt::VertexFormat::Float32,
            Self::Float32x2 => wgt::VertexFormat::Float32x2,
            Self::Float32x3 => wgt::VertexFormat::Float32x3,
            Self::Float32x4 => wgt::VertexFormat::Float32x4,
            Self::Uint32 => wgt::VertexFormat::Uint32,
            Self::Uint32x2 => wgt::VertexFormat::Uint32x2,
            Self::Uint32x3 => wgt::VertexFormat::Uint32x3,
            Self::Uint32x4 => wgt::VertexFormat::Uint32x4,
            Self::Sint32 => wgt::VertexFormat::Sint32,
            Self::Sint32x2 => wgt::VertexFormat::Sint32x2,
            Self::Sint32x3 => wgt::VertexFormat::Sint32x3,
            Self::Sint32x4 => wgt::VertexFormat::Sint32x4,
            Self::Float64 => wgt::VertexFormat::Float64,
            Self::Float64x2 => wgt::VertexFormat::Float64x2,
            Self::Float64x3 => wgt::VertexFormat::Float64x3,
            Self::Float64x4 => wgt::VertexFormat::Float64x4,
            Self::Unorm10_10_10_2 => wgt::VertexFormat::Unorm10_10_10_2,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum CanvasCompareFunction {
    Never = 1,
    Less = 2,
    Equal = 3,
    LessEqual = 4,
    Greater = 5,
    NotEqual = 6,
    GreaterEqual = 7,
    Always = 8,
}

impl CanvasCompareFunction {
    pub fn needs_ref_value(self) -> bool {
        match self {
            Self::Never | Self::Always => false,
            _ => true,
        }
    }
}

impl From<wgt::CompareFunction> for CanvasCompareFunction {
    fn from(value: wgt::CompareFunction) -> Self {
        match value {
            wgt::CompareFunction::Never => Self::Never,
            wgt::CompareFunction::Less => Self::Less,
            wgt::CompareFunction::Equal => Self::Equal,
            wgt::CompareFunction::LessEqual => Self::LessEqual,
            wgt::CompareFunction::Greater => Self::Greater,
            wgt::CompareFunction::NotEqual => Self::NotEqual,
            wgt::CompareFunction::GreaterEqual => Self::GreaterEqual,
            wgt::CompareFunction::Always => Self::Always,
        }
    }
}

impl Into<wgt::CompareFunction> for CanvasCompareFunction {
    fn into(self) -> wgt::CompareFunction {
        match self {
            CanvasCompareFunction::Never => wgt::CompareFunction::Never,
            CanvasCompareFunction::Less => wgt::CompareFunction::Less,
            CanvasCompareFunction::Equal => wgt::CompareFunction::Equal,
            CanvasCompareFunction::LessEqual => wgt::CompareFunction::LessEqual,
            CanvasCompareFunction::Greater => wgt::CompareFunction::Greater,
            CanvasCompareFunction::NotEqual => wgt::CompareFunction::NotEqual,
            CanvasCompareFunction::GreaterEqual => wgt::CompareFunction::GreaterEqual,
            CanvasCompareFunction::Always => wgt::CompareFunction::Always,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CanvasStencilFaceState {
    pub compare: CanvasCompareFunction,
    pub fail_op: CanvasStencilOperation,
    pub depth_fail_op: CanvasStencilOperation,
    pub pass_op: CanvasStencilOperation,
}

impl CanvasStencilFaceState {
    /// Ignore the stencil state for the face.
    pub const IGNORE: Self = CanvasStencilFaceState {
        compare: CanvasCompareFunction::Always,
        fail_op: CanvasStencilOperation::Keep,
        depth_fail_op: CanvasStencilOperation::Keep,
        pass_op: CanvasStencilOperation::Keep,
    };

    /// Returns true if the face state uses the reference value for testing or operation.
    pub fn needs_ref_value(&self) -> bool {
        self.compare.needs_ref_value()
            || self.fail_op == CanvasStencilOperation::Replace
            || self.depth_fail_op == CanvasStencilOperation::Replace
            || self.pass_op == CanvasStencilOperation::Replace
    }

    /// Returns true if the face state doesn't mutate the target values.
    pub fn is_read_only(&self) -> bool {
        self.pass_op == CanvasStencilOperation::Keep
            && self.depth_fail_op == CanvasStencilOperation::Keep
            && self.fail_op == CanvasStencilOperation::Keep
    }
}

impl Default for CanvasStencilFaceState {
    fn default() -> Self {
        Self::IGNORE
    }
}

impl From<wgt::StencilFaceState> for CanvasStencilFaceState {
    fn from(value: wgt::StencilFaceState) -> Self {
        Self {
            compare: value.compare.into(),
            fail_op: value.fail_op.into(),
            depth_fail_op: value.depth_fail_op.into(),
            pass_op: value.pass_op.into(),
        }
    }
}

impl Into<wgt::StencilFaceState> for CanvasStencilFaceState {
    fn into(self) -> wgt::StencilFaceState {
        wgt::StencilFaceState {
            compare: self.compare.into(),
            fail_op: self.fail_op.into(),
            depth_fail_op: self.depth_fail_op.into(),
            pass_op: self.pass_op.into(),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum CanvasStencilOperation {
    Keep = 0,
    Zero = 1,
    Replace = 2,
    Invert = 3,
    IncrementClamp = 4,
    DecrementClamp = 5,
    IncrementWrap = 6,
    DecrementWrap = 7,
}

impl From<wgt::StencilOperation> for CanvasStencilOperation {
    fn from(value: wgt::StencilOperation) -> Self {
        match value {
            wgt::StencilOperation::Keep => Self::Keep,
            wgt::StencilOperation::Zero => Self::Zero,
            wgt::StencilOperation::Replace => Self::Replace,
            wgt::StencilOperation::Invert => Self::Invert,
            wgt::StencilOperation::IncrementClamp => Self::IncrementClamp,
            wgt::StencilOperation::DecrementClamp => Self::DecrementClamp,
            wgt::StencilOperation::IncrementWrap => Self::IncrementWrap,
            wgt::StencilOperation::DecrementWrap => Self::DecrementWrap,
        }
    }
}

impl Into<wgt::StencilOperation> for CanvasStencilOperation {
    fn into(self) -> wgt::StencilOperation {
        match self {
            CanvasStencilOperation::Keep => wgt::StencilOperation::Keep,
            CanvasStencilOperation::Zero => wgt::StencilOperation::Zero,
            CanvasStencilOperation::Replace => wgt::StencilOperation::Replace,
            CanvasStencilOperation::Invert => wgt::StencilOperation::Invert,
            CanvasStencilOperation::IncrementClamp => wgt::StencilOperation::IncrementClamp,
            CanvasStencilOperation::DecrementClamp => wgt::StencilOperation::DecrementClamp,
            CanvasStencilOperation::IncrementWrap => wgt::StencilOperation::IncrementWrap,
            CanvasStencilOperation::DecrementWrap => wgt::StencilOperation::DecrementWrap,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum CanvasCullMode {
    None,
    Front,
    Back,
}

impl From<CanvasCullMode> for Option<wgt::Face> {
    fn from(value: CanvasCullMode) -> Option<wgt::Face> {
        match value {
            CanvasCullMode::None => None,
            CanvasCullMode::Front => Some(wgt::Face::Front),
            CanvasCullMode::Back => Some(wgt::Face::Back),
        }
    }
}

impl Into<CanvasCullMode> for Option<wgt::Face> {
    fn into(self) -> CanvasCullMode {
        match self {
            Some(value) => match value {
                wgt::Face::Front => CanvasCullMode::Front,
                wgt::Face::Back => CanvasCullMode::Back,
            },
            None => CanvasCullMode::None,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum CanvasPrimitiveTopology {
    PointList = 0,
    LineList = 1,
    LineStrip = 2,

    TriangleList = 3,
    TriangleStrip = 4,
}

impl From<wgt::PrimitiveTopology> for CanvasPrimitiveTopology {
    fn from(value: wgt::PrimitiveTopology) -> Self {
        match value {
            wgt::PrimitiveTopology::PointList => Self::PointList,
            wgt::PrimitiveTopology::LineList => Self::LineList,
            wgt::PrimitiveTopology::LineStrip => Self::LineStrip,
            wgt::PrimitiveTopology::TriangleList => Self::TriangleList,
            wgt::PrimitiveTopology::TriangleStrip => Self::TriangleStrip,
        }
    }
}

impl Into<wgt::PrimitiveTopology> for CanvasPrimitiveTopology {
    fn into(self) -> wgt::PrimitiveTopology {
        match self {
            CanvasPrimitiveTopology::PointList => wgt::PrimitiveTopology::PointList,
            CanvasPrimitiveTopology::LineList => wgt::PrimitiveTopology::LineList,
            CanvasPrimitiveTopology::LineStrip => wgt::PrimitiveTopology::LineStrip,
            CanvasPrimitiveTopology::TriangleList => wgt::PrimitiveTopology::TriangleList,
            CanvasPrimitiveTopology::TriangleStrip => wgt::PrimitiveTopology::TriangleStrip,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum CanvasOptionalPrimitiveTopology {
    None,
    Some(CanvasPrimitiveTopology),
}

impl From<Option<wgt::PrimitiveTopology>> for CanvasOptionalPrimitiveTopology {
    fn from(value: Option<wgt::PrimitiveTopology>) -> Self {
        match value {
            Some(value) => Self::Some(match value {
                wgt::PrimitiveTopology::PointList => CanvasPrimitiveTopology::PointList,
                wgt::PrimitiveTopology::LineList => CanvasPrimitiveTopology::LineList,
                wgt::PrimitiveTopology::LineStrip => CanvasPrimitiveTopology::LineStrip,
                wgt::PrimitiveTopology::TriangleList => {
                    CanvasPrimitiveTopology::TriangleList
                }
                wgt::PrimitiveTopology::TriangleStrip => {
                    CanvasPrimitiveTopology::TriangleStrip
                }
            }),
            None => Self::None,
        }
    }
}

impl Into<Option<wgt::PrimitiveTopology>> for CanvasOptionalPrimitiveTopology {
    fn into(self) -> Option<wgt::PrimitiveTopology> {
        match self {
            CanvasOptionalPrimitiveTopology::None => None,
            CanvasOptionalPrimitiveTopology::Some(value) => Some(match value {
                CanvasPrimitiveTopology::PointList => wgt::PrimitiveTopology::PointList,
                CanvasPrimitiveTopology::LineList => wgt::PrimitiveTopology::LineList,
                CanvasPrimitiveTopology::LineStrip => wgt::PrimitiveTopology::LineStrip,
                CanvasPrimitiveTopology::TriangleList => {
                    wgt::PrimitiveTopology::TriangleList
                }
                CanvasPrimitiveTopology::TriangleStrip => {
                    wgt::PrimitiveTopology::TriangleStrip
                }
            }),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum CanvasFrontFace {
    Ccw = 0,
    Cw = 1,
}

impl From<wgt::FrontFace> for CanvasFrontFace {
    fn from(value: wgt::FrontFace) -> Self {
        match value {
            wgt::FrontFace::Ccw => Self::Ccw,
            wgt::FrontFace::Cw => Self::Cw,
        }
    }
}

impl Into<wgt::FrontFace> for CanvasFrontFace {
    fn into(self) -> wgt::FrontFace {
        match self {
            CanvasFrontFace::Ccw => wgt::FrontFace::Ccw,
            CanvasFrontFace::Cw => wgt::FrontFace::Cw,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum CanvasOptionalFrontFace {
    None,
    Some(CanvasFrontFace),
}

impl From<Option<wgt::FrontFace>> for CanvasOptionalFrontFace {
    fn from(value: Option<wgt::FrontFace>) -> Self {
        match value {
            Some(value) => Self::Some(match value {
                wgt::FrontFace::Ccw => CanvasFrontFace::Ccw,
                wgt::FrontFace::Cw => CanvasFrontFace::Cw,
            }),
            None => Self::None,
        }
    }
}

impl Into<Option<wgt::FrontFace>> for CanvasOptionalFrontFace {
    fn into(self) -> Option<wgt::FrontFace> {
        match self {
            CanvasOptionalFrontFace::None => None,
            CanvasOptionalFrontFace::Some(value) => match value {
                CanvasFrontFace::Ccw => Some(wgt::FrontFace::Ccw),
                CanvasFrontFace::Cw => Some(wgt::FrontFace::Cw),
            },
        }
    }
}

#[repr(C)]
pub struct CanvasBufferBinding {
    pub buffer: *const CanvasGPUBuffer,
    pub offset: i64,
    pub size: i64,
}

impl Into<BufferBinding> for CanvasBufferBinding {
    fn into(self) -> BufferBinding {
        use std::num::NonZeroU64;
        let buffer = unsafe { &*self.buffer };
        let buffer_id = buffer.buffer;
        BufferBinding {
            buffer_id,
            offset: self.offset.try_into().unwrap_or_default(),
            size: self
                .size
                .try_into()
                .map(|value: u64| NonZeroU64::new(value))
                .ok()
                .flatten(),
        }
    }
}

#[repr(C)]
pub enum CanvasBindGroupEntryResource {
    Buffer(CanvasBufferBinding),
    Sampler(*const CanvasGPUSampler),
    TextureView(*const CanvasGPUTextureView),
}

#[repr(C)]
pub struct CanvasBindGroupEntry {
    pub binding: u32,
    pub resource: CanvasBindGroupEntryResource,
}

impl Into<BindGroupEntry<'static>> for CanvasBindGroupEntry {
    fn into(self) -> BindGroupEntry<'static> {
        match self.resource {
            CanvasBindGroupEntryResource::Buffer(buffer) => BindGroupEntry {
                binding: self.binding,
                resource: wgpu_core::binding_model::BindingResource::Buffer(buffer.into()),
            },
            CanvasBindGroupEntryResource::Sampler(sampler) => {
                let sampler = unsafe { &*sampler };
                let sampler_id = sampler.sampler;
                BindGroupEntry {
                    binding: self.binding,
                    resource: wgpu_core::binding_model::BindingResource::Sampler(sampler_id),
                }
            }
            CanvasBindGroupEntryResource::TextureView(view) => {
                let view = unsafe { &*view };
                let view_id = view.texture_view;
                BindGroupEntry {
                    binding: self.binding,
                    resource: wgpu_core::binding_model::BindingResource::TextureView(view_id),
                }
            }
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum CanvasStorageTextureAccess {
    /// The texture can only be written in the shader and it:
    /// - may or may not be annotated with `write` (WGSL).
    /// - must be annotated with `writeonly` (GLSL).
    ///
    /// Example WGSL syntax:
    /// ```rust,ignore
    /// @group(0) @binding(0)
    /// var my_storage_image: texture_storage_2d<f32, write>;
    /// ```
    ///
    /// Example GLSL syntax:
    /// ```cpp,ignore
    /// layout(set=0, binding=0, r32f) writeonly uniform image2D myStorageImage;
    /// ```
    WriteOnly,
    /// The texture can only be read in the shader and it must be annotated with `read` (WGSL) or
    /// `readonly` (GLSL).
    ///
    /// [`Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES`] must be enabled to use this access
    /// mode. This is a native-only extension.
    ///
    /// Example WGSL syntax:
    /// ```rust,ignore
    /// @group(0) @binding(0)
    /// var my_storage_image: texture_storage_2d<f32, read>;
    /// ```
    ///
    /// Example GLSL syntax:
    /// ```cpp,ignore
    /// layout(set=0, binding=0, r32f) readonly uniform image2D myStorageImage;
    /// ```
    ReadOnly,
    /// The texture can be both read and written in the shader and must be annotated with
    /// `read_write` in WGSL.
    ///
    /// [`Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES`] must be enabled to use this access
    /// mode.  This is a nonstandard, native-only extension.
    ///
    /// Example WGSL syntax:
    /// ```rust,ignore
    /// @group(0) @binding(0)
    /// var my_storage_image: texture_storage_2d<f32, read_write>;
    /// ```
    ///
    /// Example GLSL syntax:
    /// ```cpp,ignore
    /// layout(set=0, binding=0, r32f) uniform image2D myStorageImage;
    /// ```
    ReadWrite,
}

impl From<StorageTextureAccess> for CanvasStorageTextureAccess {
    fn from(value: StorageTextureAccess) -> Self {
        match value {
            StorageTextureAccess::WriteOnly => Self::WriteOnly,
            StorageTextureAccess::ReadOnly => Self::ReadOnly,
            StorageTextureAccess::ReadWrite => Self::ReadWrite,
        }
    }
}

impl Into<StorageTextureAccess> for CanvasStorageTextureAccess {
    fn into(self) -> StorageTextureAccess {
        match self {
            CanvasStorageTextureAccess::WriteOnly => StorageTextureAccess::WriteOnly,
            CanvasStorageTextureAccess::ReadOnly => StorageTextureAccess::ReadOnly,
            CanvasStorageTextureAccess::ReadWrite => StorageTextureAccess::ReadWrite,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CanvasTextureBindingLayout {
    sample_type: CanvasTextureSampleType,
    view_dimension: CanvasTextureViewDimension,
    multisampled: bool,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum CanvasTextureSampleType {
    Float,
    UnfilterableFloat,
    Depth,
    Sint,
    Uint,
}

impl From<TextureSampleType> for CanvasTextureSampleType {
    fn from(sample_type: TextureSampleType) -> Self {
        match sample_type {
            TextureSampleType::Float { filterable } => {
                if filterable {
                    Self::Float
                } else {
                    Self::UnfilterableFloat
                }
            }
            TextureSampleType::Depth => Self::Depth,
            TextureSampleType::Sint => Self::Sint,
            TextureSampleType::Uint => Self::Uint,
        }
    }
}

impl Into<TextureSampleType> for CanvasTextureSampleType {
    fn into(self) -> TextureSampleType {
        match self {
            CanvasTextureSampleType::Float => TextureSampleType::Float { filterable: true },
            CanvasTextureSampleType::UnfilterableFloat => {
                TextureSampleType::Float { filterable: false }
            }
            CanvasTextureSampleType::Depth => TextureSampleType::Depth,
            CanvasTextureSampleType::Sint => TextureSampleType::Sint,
            CanvasTextureSampleType::Uint => TextureSampleType::Uint,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CanvasStorageTextureBindingLayout {
    access: CanvasStorageTextureAccess,
    format: CanvasGPUTextureFormat,
    view_dimension: CanvasTextureViewDimension,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum CanvasSamplerBindingType {
    /// The sampling result is produced based on more than a single color sample from a texture,
    /// e.g. when bilinear interpolation is enabled.
    Filtering,
    /// The sampling result is produced based on a single color sample from a texture.
    NonFiltering,
    /// Use as a comparison sampler instead of a normal sampler.
    /// For more info take a look at the analogous functionality in OpenGL: <https://www.khronos.org/opengl/wiki/Sampler_Object#Comparison_mode>.
    Comparison,
}

impl From<SamplerBindingType> for CanvasSamplerBindingType {
    fn from(value: SamplerBindingType) -> Self {
        match value {
            SamplerBindingType::Filtering => Self::Filtering,
            SamplerBindingType::NonFiltering => Self::NonFiltering,
            SamplerBindingType::Comparison => Self::Comparison,
        }
    }
}

impl Into<SamplerBindingType> for CanvasSamplerBindingType {
    fn into(self) -> SamplerBindingType {
        match self {
            CanvasSamplerBindingType::Filtering => SamplerBindingType::Filtering,
            CanvasSamplerBindingType::NonFiltering => SamplerBindingType::NonFiltering,
            CanvasSamplerBindingType::Comparison => SamplerBindingType::Comparison,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CanvasSamplerBindingLayout {
    type_: CanvasSamplerBindingType,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CanvasBufferBindingLayout {
    type_: CanvasBufferBindingType,
    has_dynamic_offset: bool,
    min_binding_size: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum CanvasBufferBindingType {
    Uniform,
    Storage,
    ReadOnlyStorage,
}

impl From<BufferBindingType> for CanvasBufferBindingType {
    fn from(value: BufferBindingType) -> Self {
        match value {
            BufferBindingType::Uniform => Self::Uniform,
            BufferBindingType::Storage { read_only } => {
                if read_only {
                    Self::ReadOnlyStorage
                } else {
                    Self::Storage
                }
            }
        }
    }
}

impl Into<BufferBindingType> for CanvasBufferBindingType {
    fn into(self) -> BufferBindingType {
        match self {
            CanvasBufferBindingType::Uniform => BufferBindingType::Uniform,
            CanvasBufferBindingType::Storage => BufferBindingType::Storage { read_only: false },
            CanvasBufferBindingType::ReadOnlyStorage => {
                BufferBindingType::Storage { read_only: true }
            }
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum CanvasBindingType {
    Buffer(CanvasBufferBindingLayout),
    Sampler(CanvasSamplerBindingLayout),
    Texture(CanvasTextureBindingLayout),
    StorageTexture(CanvasStorageTextureBindingLayout),
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CanvasBindGroupLayoutEntry {
    binding: u32,
    visibility: u32,
    binding_type: CanvasBindingType,
}

impl Into<BindGroupLayoutEntry> for CanvasBindGroupLayoutEntry {
    fn into(self) -> BindGroupLayoutEntry {
        use std::num::NonZeroU64;
        BindGroupLayoutEntry {
            binding: self.binding,
            visibility: wgt::ShaderStages::from_bits(self.visibility).unwrap(),
            ty: match self.binding_type {
                CanvasBindingType::Buffer(buffer) => wgt::BindingType::Buffer {
                    ty: buffer.type_.into(),
                    has_dynamic_offset: buffer.has_dynamic_offset,
                    min_binding_size: buffer
                        .min_binding_size
                        .try_into()
                        .map(|value: u64| NonZeroU64::new(value))
                        .ok()
                        .flatten(),
                },
                CanvasBindingType::Sampler(sampler) => {
                    wgt::BindingType::Sampler(sampler.type_.into())
                }
                CanvasBindingType::Texture(texture) => wgt::BindingType::Texture {
                    sample_type: texture.sample_type.into(),
                    view_dimension: texture.view_dimension.into(),
                    multisampled: texture.multisampled,
                },
                CanvasBindingType::StorageTexture(storage_texture) => {
                    wgt::BindingType::StorageTexture {
                        access: storage_texture.access.into(),
                        format: storage_texture.format.into(),
                        view_dimension: storage_texture.view_dimension.into(),
                    }
                }
            },
            count: None, // native-only
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub enum CanvasQueryType {
    Occlusion,
    Timestamp,
}

impl Into<wgt::QueryType> for CanvasQueryType {
    fn into(self) -> QueryType {
        match self {
            CanvasQueryType::Occlusion => QueryType::Occlusion,
            CanvasQueryType::Timestamp => QueryType::Timestamp,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum CanvasAddressMode {
    /// Clamp the value to the edge of the texture
    ///
    /// -0.25 -> 0.0
    /// 1.25  -> 1.0
    ClampToEdge = 0,
    /// Repeat the texture in a tiling fashion
    ///
    /// -0.25 -> 0.75
    /// 1.25 -> 0.25
    Repeat = 1,
    /// Repeat the texture, mirroring it every repeat
    ///
    /// -0.25 -> 0.25
    /// 1.25 -> 0.75
    MirrorRepeat = 2,
    /// Clamp the value to the border of the texture
    /// Requires feature [`Features::ADDRESS_MODE_CLAMP_TO_BORDER`]
    ///
    /// -0.25 -> border
    /// 1.25 -> border
    ClampToBorder = 3,
}

impl Into<wgt::AddressMode> for CanvasAddressMode {
    fn into(self) -> AddressMode {
        match self {
            CanvasAddressMode::ClampToEdge => AddressMode::ClampToEdge,
            CanvasAddressMode::Repeat => AddressMode::Repeat,
            CanvasAddressMode::MirrorRepeat => AddressMode::MirrorRepeat,
            CanvasAddressMode::ClampToBorder => AddressMode::ClampToBorder,
        }
    }
}

impl From<wgt::AddressMode> for CanvasAddressMode {
    fn from(value: AddressMode) -> Self {
        match value {
            AddressMode::ClampToEdge => Self::ClampToEdge,
            AddressMode::Repeat => Self::Repeat,
            AddressMode::MirrorRepeat => Self::MirrorRepeat,
            AddressMode::ClampToBorder => Self::ClampToBorder,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum CanvasFilterMode {
    Nearest = 0,
    /// Linear Interpolation
    ///
    /// This makes textures smooth but blurry when used as a mag filter.
    Linear = 1,
}

impl Into<wgt::FilterMode> for CanvasFilterMode {
    fn into(self) -> FilterMode {
        match self {
            CanvasFilterMode::Nearest => FilterMode::Nearest,
            CanvasFilterMode::Linear => FilterMode::Linear,
        }
    }
}

impl From<wgt::FilterMode> for CanvasFilterMode {
    fn from(value: FilterMode) -> Self {
        match value {
            FilterMode::Nearest => Self::Nearest,
            FilterMode::Linear => Self::Linear,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum CanvasOptionalCompareFunction {
    None,
    Some(CanvasCompareFunction),
}

impl Into<Option<wgt::CompareFunction>> for CanvasOptionalCompareFunction {
    fn into(self) -> Option<CompareFunction> {
        match self {
            CanvasOptionalCompareFunction::None => None,
            CanvasOptionalCompareFunction::Some(value) => Some(value.into()),
        }
    }
}

impl From<Option<wgt::CompareFunction>> for CanvasOptionalCompareFunction {
    fn from(value: Option<CompareFunction>) -> Self {
        match value {
            None => Self::None,
            Some(value) => Self::Some(value.into()),
        }
    }
}

#[allow(non_upper_case_globals)]
pub const CanvasGPUTextureUsageCopySrc: u32 = 1 << 0;
#[allow(non_upper_case_globals)]
pub const CanvasGPUTextureUsageCopyDst: u32 = 1 << 1;
#[allow(non_upper_case_globals)]
pub const CanvasGPUTextureUsageTextureBinding: u32 = 1 << 2;
#[allow(non_upper_case_globals)]
pub const CanvasGPUTextureUsageStorageBinding: u32 = 1 << 3;
#[allow(non_upper_case_globals)]
pub const CanvasGPUTextureUsageRenderAttachment: u32 = 1 << 4;
