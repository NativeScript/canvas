use std::{ffi::CString, os::raw::c_char};

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum CanvasTextureDimension {
    D1,
    D2,
    D3,
}

impl From<wgpu_types::TextureDimension> for CanvasTextureDimension {
    fn from(value: wgpu_types::TextureDimension) -> Self {
        match value {
            wgpu_types::TextureDimension::D1 => Self::D1,
            wgpu_types::TextureDimension::D2 => Self::D2,
            wgpu_types::TextureDimension::D3 => Self::D3,
        }
    }
}

impl Into<wgpu_types::TextureDimension> for CanvasTextureDimension {
    fn into(self) -> wgpu_types::TextureDimension {
        match self {
            CanvasTextureDimension::D1 => wgpu_types::TextureDimension::D1,
            CanvasTextureDimension::D2 => wgpu_types::TextureDimension::D2,
            CanvasTextureDimension::D3 => wgpu_types::TextureDimension::D3,
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
    Rg11b10Float,

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

impl From<wgpu_types::TextureFormat> for CanvasGPUTextureFormat {
    fn from(value: wgpu_types::TextureFormat) -> Self {
        match value {
            wgpu_types::TextureFormat::Rgba8Unorm => CanvasGPUTextureFormat::Rgba8Unorm,
            wgpu_types::TextureFormat::Bgra8Unorm => CanvasGPUTextureFormat::Bgra8Unorm,
            wgpu_types::TextureFormat::R8Unorm => CanvasGPUTextureFormat::R8Unorm,
            wgpu_types::TextureFormat::R8Snorm => CanvasGPUTextureFormat::R8Snorm,
            wgpu_types::TextureFormat::R8Uint => CanvasGPUTextureFormat::R8Uint,
            wgpu_types::TextureFormat::R8Sint => CanvasGPUTextureFormat::R8Sint,
            wgpu_types::TextureFormat::R16Uint => CanvasGPUTextureFormat::R16Uint,
            wgpu_types::TextureFormat::R16Sint => CanvasGPUTextureFormat::R16Sint,
            wgpu_types::TextureFormat::R16Unorm => CanvasGPUTextureFormat::R16Unorm,
            wgpu_types::TextureFormat::R16Snorm => CanvasGPUTextureFormat::R16Snorm,
            wgpu_types::TextureFormat::R16Float => CanvasGPUTextureFormat::R16Float,
            wgpu_types::TextureFormat::Rg8Unorm => CanvasGPUTextureFormat::Rg8Unorm,
            wgpu_types::TextureFormat::Rg8Snorm => CanvasGPUTextureFormat::Rg8Snorm,
            wgpu_types::TextureFormat::Rg8Uint => CanvasGPUTextureFormat::Rg8Uint,
            wgpu_types::TextureFormat::Rg8Sint => CanvasGPUTextureFormat::Rg8Sint,
            wgpu_types::TextureFormat::R32Uint => CanvasGPUTextureFormat::R32Uint,
            wgpu_types::TextureFormat::R32Sint => CanvasGPUTextureFormat::R32Sint,
            wgpu_types::TextureFormat::R32Float => CanvasGPUTextureFormat::R32Float,
            wgpu_types::TextureFormat::Rg16Uint => CanvasGPUTextureFormat::Rg16Uint,
            wgpu_types::TextureFormat::Rg16Sint => CanvasGPUTextureFormat::Rg16Sint,
            wgpu_types::TextureFormat::Rg16Unorm => CanvasGPUTextureFormat::Rg16Unorm,
            wgpu_types::TextureFormat::Rg16Snorm => CanvasGPUTextureFormat::Rg16Snorm,
            wgpu_types::TextureFormat::Rg16Float => CanvasGPUTextureFormat::Rg16Float,
            wgpu_types::TextureFormat::Rgba8Unorm => CanvasGPUTextureFormat::Rgba8Unorm,
            wgpu_types::TextureFormat::Rgba8UnormSrgb => CanvasGPUTextureFormat::Rgba8UnormSrgb,
            wgpu_types::TextureFormat::Rgba8Snorm => CanvasGPUTextureFormat::Rgba8Snorm,
            wgpu_types::TextureFormat::Rgba8Uint => CanvasGPUTextureFormat::Rgba8Uint,
            wgpu_types::TextureFormat::Rgba8Sint => CanvasGPUTextureFormat::Rgba8Sint,
            wgpu_types::TextureFormat::Bgra8Unorm => CanvasGPUTextureFormat::Bgra8Unorm,
            wgpu_types::TextureFormat::Bgra8UnormSrgb => CanvasGPUTextureFormat::Bgra8UnormSrgb,
            wgpu_types::TextureFormat::Rgb9e5Ufloat => CanvasGPUTextureFormat::Rgb9e5Ufloat,
            wgpu_types::TextureFormat::Rgb10a2Uint => CanvasGPUTextureFormat::Rgb10a2Uint,
            wgpu_types::TextureFormat::Rgb10a2Unorm => CanvasGPUTextureFormat::Rgb10a2Unorm,
            wgpu_types::TextureFormat::Rg11b10Float => CanvasGPUTextureFormat::Rg11b10Float,
            wgpu_types::TextureFormat::Rg32Uint => CanvasGPUTextureFormat::Rg32Uint,
            wgpu_types::TextureFormat::Rg32Sint => CanvasGPUTextureFormat::Rg32Sint,
            wgpu_types::TextureFormat::Rg32Float => CanvasGPUTextureFormat::Rg32Float,
            wgpu_types::TextureFormat::Rgba16Uint => CanvasGPUTextureFormat::Rgba16Uint,
            wgpu_types::TextureFormat::Rgba16Sint => CanvasGPUTextureFormat::Rgba16Sint,
            wgpu_types::TextureFormat::Rgba16Unorm => CanvasGPUTextureFormat::Rgba16Unorm,
            wgpu_types::TextureFormat::Rgba16Snorm => CanvasGPUTextureFormat::Rgba16Snorm,
            wgpu_types::TextureFormat::Rgba16Float => CanvasGPUTextureFormat::Rgba16Float,
            wgpu_types::TextureFormat::Rgba32Uint => CanvasGPUTextureFormat::Rgba32Uint,
            wgpu_types::TextureFormat::Rgba32Sint => CanvasGPUTextureFormat::Rgba32Sint,
            wgpu_types::TextureFormat::Rgba32Float => CanvasGPUTextureFormat::Rgba32Float,
            wgpu_types::TextureFormat::Stencil8 => CanvasGPUTextureFormat::Stencil8,
            wgpu_types::TextureFormat::Depth16Unorm => CanvasGPUTextureFormat::Depth16Unorm,
            wgpu_types::TextureFormat::Depth24Plus => CanvasGPUTextureFormat::Depth24Plus,
            wgpu_types::TextureFormat::Depth24PlusStencil8 => {
                CanvasGPUTextureFormat::Depth24PlusStencil8
            }
            wgpu_types::TextureFormat::Depth32Float => CanvasGPUTextureFormat::Depth32Float,
            wgpu_types::TextureFormat::Depth32FloatStencil8 => {
                CanvasGPUTextureFormat::Depth32FloatStencil8
            }
            wgpu_types::TextureFormat::NV12 => CanvasGPUTextureFormat::NV12,
            wgpu_types::TextureFormat::Bc1RgbaUnorm => CanvasGPUTextureFormat::Bc1RgbaUnorm,
            wgpu_types::TextureFormat::Bc1RgbaUnormSrgb => CanvasGPUTextureFormat::Bc1RgbaUnormSrgb,
            wgpu_types::TextureFormat::Bc2RgbaUnorm => CanvasGPUTextureFormat::Bc2RgbaUnorm,
            wgpu_types::TextureFormat::Bc2RgbaUnormSrgb => CanvasGPUTextureFormat::Bc2RgbaUnormSrgb,
            wgpu_types::TextureFormat::Bc3RgbaUnorm => CanvasGPUTextureFormat::Bc3RgbaUnorm,
            wgpu_types::TextureFormat::Bc3RgbaUnormSrgb => CanvasGPUTextureFormat::Bc3RgbaUnormSrgb,
            wgpu_types::TextureFormat::Bc4RUnorm => CanvasGPUTextureFormat::Bc4RUnorm,
            wgpu_types::TextureFormat::Bc4RSnorm => CanvasGPUTextureFormat::Bc4RSnorm,
            wgpu_types::TextureFormat::Bc5RgUnorm => CanvasGPUTextureFormat::Bc5RgUnorm,
            wgpu_types::TextureFormat::Bc5RgSnorm => CanvasGPUTextureFormat::Bc5RgSnorm,
            wgpu_types::TextureFormat::Bc6hRgbUfloat => CanvasGPUTextureFormat::Bc6hRgbUfloat,
            wgpu_types::TextureFormat::Bc6hRgbFloat => CanvasGPUTextureFormat::Bc6hRgbFloat,
            wgpu_types::TextureFormat::Bc7RgbaUnorm => CanvasGPUTextureFormat::Bc7RgbaUnorm,
            wgpu_types::TextureFormat::Bc7RgbaUnormSrgb => CanvasGPUTextureFormat::Bc7RgbaUnormSrgb,
            wgpu_types::TextureFormat::Etc2Rgb8Unorm => CanvasGPUTextureFormat::Etc2Rgb8Unorm,
            wgpu_types::TextureFormat::Etc2Rgb8UnormSrgb => {
                CanvasGPUTextureFormat::Etc2Rgb8UnormSrgb
            }
            wgpu_types::TextureFormat::Etc2Rgb8A1Unorm => CanvasGPUTextureFormat::Etc2Rgb8A1Unorm,
            wgpu_types::TextureFormat::Etc2Rgb8A1UnormSrgb => {
                CanvasGPUTextureFormat::Etc2Rgb8A1UnormSrgb
            }
            wgpu_types::TextureFormat::Etc2Rgba8Unorm => CanvasGPUTextureFormat::Etc2Rgba8Unorm,
            wgpu_types::TextureFormat::Etc2Rgba8UnormSrgb => {
                CanvasGPUTextureFormat::Etc2Rgba8UnormSrgb
            }
            wgpu_types::TextureFormat::EacR11Unorm => CanvasGPUTextureFormat::EacR11Unorm,
            wgpu_types::TextureFormat::EacR11Snorm => CanvasGPUTextureFormat::EacR11Snorm,
            wgpu_types::TextureFormat::EacRg11Unorm => CanvasGPUTextureFormat::EacRg11Unorm,
            wgpu_types::TextureFormat::EacRg11Snorm => CanvasGPUTextureFormat::EacRg11Snorm,
            wgpu_types::TextureFormat::Astc { block, channel } => CanvasGPUTextureFormat::Astc {
                block: block.into(),
                channel: channel.into(),
            },
        }
    }
}

impl Into<wgpu_types::TextureFormat> for CanvasGPUTextureFormat {
    fn into(self) -> wgpu_types::TextureFormat {
        match self {
            CanvasGPUTextureFormat::Rgba8Unorm => wgpu_types::TextureFormat::Rgba8Unorm,
            CanvasGPUTextureFormat::Bgra8Unorm => wgpu_types::TextureFormat::Bgra8Unorm,
            CanvasGPUTextureFormat::R8Unorm => wgpu_types::TextureFormat::R8Unorm,
            CanvasGPUTextureFormat::R8Snorm => wgpu_types::TextureFormat::R8Snorm,
            CanvasGPUTextureFormat::R8Uint => wgpu_types::TextureFormat::R8Uint,
            CanvasGPUTextureFormat::R8Sint => wgpu_types::TextureFormat::R8Sint,
            CanvasGPUTextureFormat::R16Uint => wgpu_types::TextureFormat::R16Uint,
            CanvasGPUTextureFormat::R16Sint => wgpu_types::TextureFormat::R16Sint,
            CanvasGPUTextureFormat::R16Unorm => wgpu_types::TextureFormat::R16Unorm,
            CanvasGPUTextureFormat::R16Snorm => wgpu_types::TextureFormat::R16Snorm,
            CanvasGPUTextureFormat::R16Float => wgpu_types::TextureFormat::R16Float,
            CanvasGPUTextureFormat::Rg8Unorm => wgpu_types::TextureFormat::Rg8Unorm,
            CanvasGPUTextureFormat::Rg8Snorm => wgpu_types::TextureFormat::Rg8Snorm,
            CanvasGPUTextureFormat::Rg8Uint => wgpu_types::TextureFormat::Rg8Uint,
            CanvasGPUTextureFormat::Rg8Sint => wgpu_types::TextureFormat::Rg8Sint,
            CanvasGPUTextureFormat::R32Uint => wgpu_types::TextureFormat::R32Uint,
            CanvasGPUTextureFormat::R32Sint => wgpu_types::TextureFormat::R32Sint,
            CanvasGPUTextureFormat::R32Float => wgpu_types::TextureFormat::R32Float,
            CanvasGPUTextureFormat::Rg16Uint => wgpu_types::TextureFormat::Rg16Uint,
            CanvasGPUTextureFormat::Rg16Sint => wgpu_types::TextureFormat::Rg16Sint,
            CanvasGPUTextureFormat::Rg16Unorm => wgpu_types::TextureFormat::Rg16Unorm,
            CanvasGPUTextureFormat::Rg16Snorm => wgpu_types::TextureFormat::Rg16Snorm,
            CanvasGPUTextureFormat::Rg16Float => wgpu_types::TextureFormat::Rg16Float,
            CanvasGPUTextureFormat::Rgba8Unorm => wgpu_types::TextureFormat::Rgba8Unorm,
            CanvasGPUTextureFormat::Rgba8UnormSrgb => wgpu_types::TextureFormat::Rgba8UnormSrgb,
            CanvasGPUTextureFormat::Rgba8Snorm => wgpu_types::TextureFormat::Rgba8Snorm,
            CanvasGPUTextureFormat::Rgba8Uint => wgpu_types::TextureFormat::Rgba8Uint,
            CanvasGPUTextureFormat::Rgba8Sint => wgpu_types::TextureFormat::Rgba8Sint,
            CanvasGPUTextureFormat::Bgra8Unorm => wgpu_types::TextureFormat::Bgra8Unorm,
            CanvasGPUTextureFormat::Bgra8UnormSrgb => wgpu_types::TextureFormat::Bgra8UnormSrgb,
            CanvasGPUTextureFormat::Rgb9e5Ufloat => wgpu_types::TextureFormat::Rgb9e5Ufloat,
            CanvasGPUTextureFormat::Rgb10a2Uint => wgpu_types::TextureFormat::Rgb10a2Uint,
            CanvasGPUTextureFormat::Rgb10a2Unorm => wgpu_types::TextureFormat::Rgb10a2Unorm,
            CanvasGPUTextureFormat::Rg11b10Float => wgpu_types::TextureFormat::Rg11b10Float,
            CanvasGPUTextureFormat::Rg32Uint => wgpu_types::TextureFormat::Rg32Uint,
            CanvasGPUTextureFormat::Rg32Sint => wgpu_types::TextureFormat::Rg32Sint,
            CanvasGPUTextureFormat::Rg32Float => wgpu_types::TextureFormat::Rg32Float,
            CanvasGPUTextureFormat::Rgba16Uint => wgpu_types::TextureFormat::Rgba16Uint,
            CanvasGPUTextureFormat::Rgba16Sint => wgpu_types::TextureFormat::Rgba16Sint,
            CanvasGPUTextureFormat::Rgba16Unorm => wgpu_types::TextureFormat::Rgba16Unorm,
            CanvasGPUTextureFormat::Rgba16Snorm => wgpu_types::TextureFormat::Rgba16Snorm,
            CanvasGPUTextureFormat::Rgba16Float => wgpu_types::TextureFormat::Rgba16Float,
            CanvasGPUTextureFormat::Rgba32Uint => wgpu_types::TextureFormat::Rgba32Uint,
            CanvasGPUTextureFormat::Rgba32Sint => wgpu_types::TextureFormat::Rgba32Sint,
            CanvasGPUTextureFormat::Rgba32Float => wgpu_types::TextureFormat::Rgba32Float,
            CanvasGPUTextureFormat::Stencil8 => wgpu_types::TextureFormat::Stencil8,
            CanvasGPUTextureFormat::Depth16Unorm => wgpu_types::TextureFormat::Depth16Unorm,
            CanvasGPUTextureFormat::Depth24Plus => wgpu_types::TextureFormat::Depth24Plus,
            CanvasGPUTextureFormat::Depth24PlusStencil8 => {
                wgpu_types::TextureFormat::Depth24PlusStencil8
            }
            CanvasGPUTextureFormat::Depth32Float => wgpu_types::TextureFormat::Depth32Float,
            CanvasGPUTextureFormat::Depth32FloatStencil8 => {
                wgpu_types::TextureFormat::Depth32FloatStencil8
            }
            CanvasGPUTextureFormat::NV12 => wgpu_types::TextureFormat::NV12,
            CanvasGPUTextureFormat::Bc1RgbaUnorm => wgpu_types::TextureFormat::Bc1RgbaUnorm,
            CanvasGPUTextureFormat::Bc1RgbaUnormSrgb => wgpu_types::TextureFormat::Bc1RgbaUnormSrgb,
            CanvasGPUTextureFormat::Bc2RgbaUnorm => wgpu_types::TextureFormat::Bc2RgbaUnorm,
            CanvasGPUTextureFormat::Bc2RgbaUnormSrgb => wgpu_types::TextureFormat::Bc2RgbaUnormSrgb,
            CanvasGPUTextureFormat::Bc3RgbaUnorm => wgpu_types::TextureFormat::Bc3RgbaUnorm,
            CanvasGPUTextureFormat::Bc3RgbaUnormSrgb => wgpu_types::TextureFormat::Bc3RgbaUnormSrgb,
            CanvasGPUTextureFormat::Bc4RUnorm => wgpu_types::TextureFormat::Bc4RUnorm,
            CanvasGPUTextureFormat::Bc4RSnorm => wgpu_types::TextureFormat::Bc4RSnorm,
            CanvasGPUTextureFormat::Bc5RgUnorm => wgpu_types::TextureFormat::Bc5RgUnorm,
            CanvasGPUTextureFormat::Bc5RgSnorm => wgpu_types::TextureFormat::Bc5RgSnorm,
            CanvasGPUTextureFormat::Bc6hRgbUfloat => wgpu_types::TextureFormat::Bc6hRgbUfloat,
            CanvasGPUTextureFormat::Bc6hRgbFloat => wgpu_types::TextureFormat::Bc6hRgbFloat,
            CanvasGPUTextureFormat::Bc7RgbaUnorm => wgpu_types::TextureFormat::Bc7RgbaUnorm,
            CanvasGPUTextureFormat::Bc7RgbaUnormSrgb => wgpu_types::TextureFormat::Bc7RgbaUnormSrgb,
            CanvasGPUTextureFormat::Etc2Rgb8Unorm => wgpu_types::TextureFormat::Etc2Rgb8Unorm,
            CanvasGPUTextureFormat::Etc2Rgb8UnormSrgb => {
                wgpu_types::TextureFormat::Etc2Rgb8UnormSrgb
            }
            CanvasGPUTextureFormat::Etc2Rgb8A1Unorm => wgpu_types::TextureFormat::Etc2Rgb8A1Unorm,
            CanvasGPUTextureFormat::Etc2Rgb8A1UnormSrgb => {
                wgpu_types::TextureFormat::Etc2Rgb8A1UnormSrgb
            }
            CanvasGPUTextureFormat::Etc2Rgba8Unorm => wgpu_types::TextureFormat::Etc2Rgba8Unorm,
            CanvasGPUTextureFormat::Etc2Rgba8UnormSrgb => {
                wgpu_types::TextureFormat::Etc2Rgba8UnormSrgb
            }
            CanvasGPUTextureFormat::EacR11Unorm => wgpu_types::TextureFormat::EacR11Unorm,
            CanvasGPUTextureFormat::EacR11Snorm => wgpu_types::TextureFormat::EacR11Snorm,
            CanvasGPUTextureFormat::EacRg11Unorm => wgpu_types::TextureFormat::EacRg11Unorm,
            CanvasGPUTextureFormat::EacRg11Snorm => wgpu_types::TextureFormat::EacRg11Snorm,
            CanvasGPUTextureFormat::Astc { block, channel } => wgpu_types::TextureFormat::Astc {
                block: block.into(),
                channel: channel.into(),
            },
        }
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
        CanvasGPUTextureFormat::Rg11b10Float => "rg11b10ufloat",
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

impl From<wgpu_types::AstcChannel> for CanvasAstcChannel {
    fn from(value: wgpu_types::AstcChannel) -> Self {
        match value {
            wgpu_types::AstcChannel::Unorm => Self::Unorm,
            wgpu_types::AstcChannel::UnormSrgb => Self::UnormSrgb,
            wgpu_types::AstcChannel::Hdr => Self::Hdr,
        }
    }
}

impl Into<wgpu_types::AstcChannel> for CanvasAstcChannel {
    fn into(self) -> wgpu_types::AstcChannel {
        match self {
            CanvasAstcChannel::Unorm => wgpu_types::AstcChannel::Unorm,
            CanvasAstcChannel::UnormSrgb => wgpu_types::AstcChannel::UnormSrgb,
            CanvasAstcChannel::Hdr => wgpu_types::AstcChannel::Hdr,
        }
    }
}

impl From<wgpu_types::AstcBlock> for CanvasAstcBlock {
    fn from(value: wgpu_types::AstcBlock) -> Self {
        match value {
            wgpu_types::AstcBlock::B4x4 => Self::B4x4,
            wgpu_types::AstcBlock::B5x4 => Self::B5x4,
            wgpu_types::AstcBlock::B5x5 => Self::B5x5,
            wgpu_types::AstcBlock::B6x5 => Self::B6x5,
            wgpu_types::AstcBlock::B6x6 => Self::B6x6,
            wgpu_types::AstcBlock::B8x5 => Self::B8x5,
            wgpu_types::AstcBlock::B8x6 => Self::B8x6,
            wgpu_types::AstcBlock::B8x8 => Self::B8x8,
            wgpu_types::AstcBlock::B10x5 => Self::B10x5,
            wgpu_types::AstcBlock::B10x6 => Self::B10x6,
            wgpu_types::AstcBlock::B10x8 => Self::B10x8,
            wgpu_types::AstcBlock::B10x10 => Self::B10x10,
            wgpu_types::AstcBlock::B12x10 => Self::B12x10,
            wgpu_types::AstcBlock::B12x12 => Self::B12x12,
        }
    }
}

impl Into<wgpu_types::AstcBlock> for CanvasAstcBlock {
    fn into(self) -> wgpu_types::AstcBlock {
        match self {
            CanvasAstcBlock::B4x4 => wgpu_types::AstcBlock::B4x4,
            CanvasAstcBlock::B5x4 => wgpu_types::AstcBlock::B5x4,
            CanvasAstcBlock::B5x5 => wgpu_types::AstcBlock::B5x5,
            CanvasAstcBlock::B6x5 => wgpu_types::AstcBlock::B6x5,
            CanvasAstcBlock::B6x6 => wgpu_types::AstcBlock::B6x6,
            CanvasAstcBlock::B8x5 => wgpu_types::AstcBlock::B8x5,
            CanvasAstcBlock::B8x6 => wgpu_types::AstcBlock::B8x6,
            CanvasAstcBlock::B8x8 => wgpu_types::AstcBlock::B8x8,
            CanvasAstcBlock::B10x5 => wgpu_types::AstcBlock::B10x5,
            CanvasAstcBlock::B10x6 => wgpu_types::AstcBlock::B10x6,
            CanvasAstcBlock::B10x8 => wgpu_types::AstcBlock::B10x8,
            CanvasAstcBlock::B10x10 => wgpu_types::AstcBlock::B10x10,
            CanvasAstcBlock::B12x10 => wgpu_types::AstcBlock::B12x10,
            CanvasAstcBlock::B12x12 => wgpu_types::AstcBlock::B12x12,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
pub enum CanvasTextureAspect {
    /// Depth, Stencil, and Color.
    #[default]
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

impl From<wgpu_types::TextureAspect> for CanvasTextureAspect {
    fn from(value: wgpu_types::TextureAspect) -> Self {
        match value {
            wgpu_types::TextureAspect::All => CanvasTextureAspect::All,
            wgpu_types::TextureAspect::StencilOnly => CanvasTextureAspect::StencilOnly,
            wgpu_types::TextureAspect::DepthOnly => CanvasTextureAspect::DepthOnly,
            wgpu_types::TextureAspect::Plane0 => CanvasTextureAspect::Plane0,
            wgpu_types::TextureAspect::Plane1 => CanvasTextureAspect::Plane1,
            wgpu_types::TextureAspect::Plane2 => CanvasTextureAspect::Plane2,
        }
    }
}


impl Into<wgpu_types::TextureAspect> for CanvasTextureAspect {
    fn into(self) -> wgpu_types::TextureAspect {
        match self {
            CanvasTextureAspect::All => wgpu_types::TextureAspect::All,
            CanvasTextureAspect::StencilOnly => wgpu_types::TextureAspect::StencilOnly,
            CanvasTextureAspect::DepthOnly => wgpu_types::TextureAspect::DepthOnly,
            CanvasTextureAspect::Plane0 => wgpu_types::TextureAspect::Plane0,
            CanvasTextureAspect::Plane1 => wgpu_types::TextureAspect::Plane1,
            CanvasTextureAspect::Plane2 => wgpu_types::TextureAspect::Plane2,
        }
    }
}
