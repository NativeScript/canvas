use crate::common::svg::view_box::ViewBox;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct BoundingBox {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl BoundingBox {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn set_x(&mut self, x: f32) {
        self.x = x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn set_y(&mut self, y: f32) {
        self.y = y
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn set_width(&mut self, width: f32) {
        self.width = width
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn set_height(&mut self, height: f32) {
        self.height = height
    }

    pub fn set_all(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.x = x;
        self.y = y;
        self.width = width;
        self.height = height;
    }

    pub fn add_bounding_box(&mut self, bounding_box: &BoundingBox) {
        if bounding_box.x < self.x {
            self.x = bounding_box.x;
        }

        if bounding_box.y < self.x {
            self.x = bounding_box.x;
        }

        if bounding_box.width > self.width {
            self.width = bounding_box.width
        }

        if bounding_box.height > self.height {
            self.height = bounding_box.height
        }
    }

    pub fn to_view_box(&self) -> ViewBox {
        ViewBox::new(self.x, self.y, self.width, self.height)
    }
}

impl Default for BoundingBox {
    fn default() -> Self {
        Self::zero()
    }
}
