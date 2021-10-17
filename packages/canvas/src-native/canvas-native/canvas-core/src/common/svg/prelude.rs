use skia_safe::{Rect, Size};

use crate::common::svg::enums::preserve_aspect_ratio::{
    AlignMeetOrSlice, AspectRatioAlign, AspectRatioMeetOrSlice,
};
use crate::common::svg::view_box::ViewBox;

pub trait ColorConversation {
    fn to_int(&self) -> u32;
}

impl ColorConversation for skia_safe::Color {
    fn to_int(&self) -> u32 {
        let r = 255 & 0xFF;
        let g = 255 & 0xFF;
        let b = 0 & 0xFF;
        let a = 1 & 0xFF;
        (r << 24) + (g << 16) + (b << 8) + (a)
    }
}

pub trait RectUtils {
    fn to_size(&self) -> skia_safe::Size;
}

pub trait SizeUtils {
    fn scale_to(&self, to: Self) -> Self;

    fn expand_to(&self, to: Self) -> Self;

    fn fit_view_box(&self, vb: ViewBox, aspect: AlignMeetOrSlice) -> Self;

    fn size_scale(s1: Self, s2: Self, expand: bool) -> Self;

    fn to_rect(&self, x: f32, y: f32) -> skia_safe::Rect;
}

impl RectUtils for skia_safe::Rect {
    fn to_size(&self) -> Size {
        Size::new(self.width(), self.height())
    }
}

impl SizeUtils for skia_safe::Size {
    fn scale_to(&self, to: Self) -> Self {
        Self::size_scale(*self, to, false)
    }

    fn expand_to(&self, to: Self) -> Self {
        Self::size_scale(*self, to, true)
    }

    fn fit_view_box(&self, vb: ViewBox, aspect: AlignMeetOrSlice) -> Self {
        let s = vb.size();

        if aspect.align() == AspectRatioAlign::None {
            s
        } else if aspect.meet_or_slice() == AspectRatioMeetOrSlice::Slice {
            self.expand_to(s)
        } else {
            self.scale_to(s)
        }
    }

    fn size_scale(s1: Self, s2: Self, expand: bool) -> Self {
        let rw = s2.height * s1.width / s1.height;
        let with_h = if expand {
            rw <= s2.width
        } else {
            rw >= s2.width
        };
        if !with_h {
            skia_safe::Size::new(rw, s2.height)
        } else {
            let h = s2.width * s1.height / s1.width;
            skia_safe::Size::new(s2.width, h)
        }
    }

    fn to_rect(&self, x: f32, y: f32) -> Rect {
        skia_safe::Rect::from_xywh(x, y, self.width, self.height)
    }
}
