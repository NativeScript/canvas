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
