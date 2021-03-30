use skia_safe::Vector;

use crate::common::svg::view_box::ViewBox;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct AlignMeetOrSlice {
    align: AspectRatioAlign,
    meet_or_slice: AspectRatioMeetOrSlice,
}

impl AlignMeetOrSlice {
    pub fn align(&self) -> AspectRatioAlign {
        self.align
    }

    pub fn align_set(&mut self, value: AspectRatioAlign) {
        self.align = value
    }

    pub fn meet_or_slice(&self) -> AspectRatioMeetOrSlice {
        self.meet_or_slice
    }

    pub fn meet_or_slice_set(&mut self, value: AspectRatioMeetOrSlice) {
        self.meet_or_slice = value
    }
}

impl Default for AlignMeetOrSlice {
    fn default() -> Self {
        Self {
            align: AspectRatioAlign::default(),
            meet_or_slice: AspectRatioMeetOrSlice::default(),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum AspectRatioAlign {
    None,
    XminYmin,
    XmidYmin,
    XmaxYmin,
    XminYmid,
    XmidYmid,
    XmaxYmid,
    XminYmax,
    XmidYmax,
    XmaxYmax,
}

impl Default for AspectRatioAlign {
    fn default() -> Self {
        Self::XmidYmid
    }
}

impl From<&str> for AspectRatioAlign {
    fn from(val: &str) -> Self {
        match val {
            "none" => Self::None,
            "xMinYMin" => Self::XminYmin,
            "xMidYMin" => Self::XmidYmin,
            "xMaxYMin" => Self::XmaxYmin,
            "xMinYMid" => Self::XminYmid,
            "xMidYMid" => Self::XmidYmid,
            "xMaxYMid" => Self::XmaxYmid,
            "xMinYMax" => Self::XminYmax,
            "xMidYMax" => Self::XmidYmax,
            "xMaxYMax" => Self::XmaxYmax,
            _ => Self::default(),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum AspectRatioMeetOrSlice {
    Meet,
    Slice,
}

impl Default for AspectRatioMeetOrSlice {
    fn default() -> Self {
        Self::Meet
    }
}

impl From<&str> for AspectRatioMeetOrSlice {
    fn from(val: &str) -> Self {
        match val {
            "slice" => Self::Slice,
            _ => Self::default(),
        }
    }
}

pub fn view_box_to_transform(
    view_box: &ViewBox,
    aspect: AlignMeetOrSlice,
    width: f32,
    height: f32,
) -> skia_safe::Matrix {
    let vr = view_box;

    let sx = width / vr.width();
    let sy = height / vr.height();

    let (sx, sy) = if aspect.align == AspectRatioAlign::None {
        (sx, sy)
    } else {
        let s = if aspect.meet_or_slice() == AspectRatioMeetOrSlice::Slice {
            if sx < sy {
                sy
            } else {
                sx
            }
        } else {
            if sx > sy {
                sy
            } else {
                sx
            }
        };

        (s, s)
    };

    let x = -vr.x() * sx;
    let y = -vr.y() * sy;
    let w = width - vr.width() * sx;
    let h = height - vr.height() * sy;

    let (tx, ty) = aligned_pos(aspect.align(), x, y, w, h);
    let aff = [sx, 0.0, 0.0, sy, tx, ty];
    skia_safe::Matrix::from_affine(&aff)
}

pub fn aligned_pos(align: AspectRatioAlign, x: f32, y: f32, w: f32, h: f32) -> (f32, f32) {
    match align {
        AspectRatioAlign::None => (x, y),
        AspectRatioAlign::XminYmin => (x, y),
        AspectRatioAlign::XmidYmin => (x + w / 2.0, y),
        AspectRatioAlign::XmaxYmin => (x + w, y),
        AspectRatioAlign::XminYmid => (x, y + h / 2.0),
        AspectRatioAlign::XmidYmid => (x + w / 2.0, y + h / 2.0),
        AspectRatioAlign::XmaxYmid => (x + w, y + h / 2.0),
        AspectRatioAlign::XminYmax => (x, y + h),
        AspectRatioAlign::XmidYmax => (x + w / 2.0, y + h),
        AspectRatioAlign::XmaxYmax => (x + w, y + h),
    }
}
