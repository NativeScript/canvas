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

impl Into<wgpu_types::Extent3d> for CanvasExtent3d{
    fn into(self) -> wgpu_types::Extent3d {
        wgpu_types::Extent3d { width: self.width, height: self.height, depth_or_array_layers: self.depth_or_array_layers }
    }
}


impl From<wgpu_types::Extent3d> for CanvasExtent3d{
    fn from(value: wgpu_types::Extent3d) -> Self {
       Self {
        width: value.width,
        height: value.height,
        depth_or_array_layers: value.depth_or_array_layers,
    }
    }
} 