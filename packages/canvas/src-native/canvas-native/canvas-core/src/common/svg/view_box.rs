use std::iter::FromIterator;
use std::str::FromStr;

use crate::common::context::Context;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ViewBox {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl Default for ViewBox {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
        }
    }
}

impl ViewBox {
    pub fn new_with_context(context: &Context) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: context.surface.width() as f32,
            height: context.surface.height() as f32,
        }
    }
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
    pub fn new_wh(width: f32, height: f32) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width,
            height,
        }
    }

    pub fn size(&self) -> skia_safe::Size {
        skia_safe::Size::new(self.width, self.height)
    }

    pub fn x(&self) -> f32 {
        self.x
    }
    pub fn y(&self) -> f32 {
        self.y
    }
    pub fn width(&self) -> f32 {
        self.width
    }
    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn x_set(&mut self, x: f32) {
        self.x = x
    }
    pub fn y_set(&mut self, y: f32) {
        self.y = y
    }
    pub fn width_set(&mut self, width: f32) {
        self.width = width
    }
    pub fn height_set(&mut self, height: f32) {
        self.height = height
    }
}

impl FromStr for ViewBox {
    type Err = super::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let val = s.replace(",", " ");
        let val = val.split(" ");
        let mut value = ViewBox::default();
        let mut position = 0;
        for i in val.into_iter() {
            match i.parse::<f32>() {
                Ok(val) => match position {
                    0 => value.x = val,
                    1 => value.y = val,
                    2 => value.width = val,
                    3 => value.height = val,
                    _ => {}
                },
                Err(e) => return Err(super::error::Error::InvalidNumber),
            };
            position += 1;
        }
        Ok(value)
    }
}
