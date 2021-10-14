use std::os::raw::c_float;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct TextMetrics {
    pub(crate) width: c_float,
    pub(crate) actual_bounding_box_left: c_float,
    pub(crate) actual_bounding_box_right: c_float,
    pub(crate) font_bounding_box_ascent: c_float,
    pub(crate) font_bounding_box_descent: c_float,
    pub(crate) actual_bounding_box_ascent: c_float,
    pub(crate) actual_bounding_box_descent: c_float,
    pub(crate) em_height_ascent: c_float,
    pub(crate) em_height_descent: c_float,
    pub(crate) hanging_baseline: c_float,
    pub(crate) alphabetic_baseline: c_float,
    pub(crate) ideographic_baseline: c_float,
}

impl TextMetrics {
    pub fn width(&self) -> c_float {
        self.width
    }
    pub fn actual_bounding_box_left(&self) -> c_float {
        self.actual_bounding_box_left
    }
    pub fn actual_bounding_box_right(&self) -> c_float {
        self.actual_bounding_box_right
    }
    pub fn font_bounding_box_ascent(&self) -> c_float {
        self.font_bounding_box_ascent
    }
    pub fn font_bounding_box_descent(&self) -> c_float {
        self.font_bounding_box_descent
    }
    pub fn actual_bounding_box_ascent(&self) -> c_float {
        self.actual_bounding_box_ascent
    }
    pub fn actual_bounding_box_descent(&self) -> c_float {
        self.actual_bounding_box_descent
    }
    pub fn em_height_ascent(&self) -> c_float {
        self.em_height_ascent
    }
    pub fn em_height_descent(&self) -> c_float {
        self.em_height_descent
    }
    pub fn hanging_baseline(&self) -> c_float {
        self.hanging_baseline
    }
    pub fn alphabetic_baseline(&self) -> c_float {
        self.alphabetic_baseline
    }
    pub fn ideographic_baseline(&self) -> c_float {
        self.ideographic_baseline
    }
}
