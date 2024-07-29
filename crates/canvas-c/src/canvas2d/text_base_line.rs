#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum TextBaseLine {
    TOP = 0,
    HANGING = 1,
    MIDDLE = 2,
    ALPHABETIC = 3,
    IDEOGRAPHIC = 4,
    BOTTOM = 5,
}

impl Into<canvas_2d::context::text_styles::text_baseline::TextBaseLine> for TextBaseLine {
    fn into(self) -> canvas_2d::context::text_styles::text_baseline::TextBaseLine {
        match self {
            TextBaseLine::TOP => canvas_2d::context::text_styles::text_baseline::TextBaseLine::TOP,
            TextBaseLine::HANGING => {
                canvas_2d::context::text_styles::text_baseline::TextBaseLine::HANGING
            }
            TextBaseLine::MIDDLE => {
                canvas_2d::context::text_styles::text_baseline::TextBaseLine::MIDDLE
            }
            TextBaseLine::ALPHABETIC => {
                canvas_2d::context::text_styles::text_baseline::TextBaseLine::ALPHABETIC
            }
            TextBaseLine::IDEOGRAPHIC => {
                canvas_2d::context::text_styles::text_baseline::TextBaseLine::IDEOGRAPHIC
            }
            TextBaseLine::BOTTOM => {
                canvas_2d::context::text_styles::text_baseline::TextBaseLine::BOTTOM
            }
        }
    }
}

impl Into<TextBaseLine> for canvas_2d::context::text_styles::text_baseline::TextBaseLine {
    fn into(self) -> TextBaseLine {
        match self {
            canvas_2d::context::text_styles::text_baseline::TextBaseLine::TOP => TextBaseLine::TOP,
            canvas_2d::context::text_styles::text_baseline::TextBaseLine::HANGING => {
                TextBaseLine::HANGING
            }
            canvas_2d::context::text_styles::text_baseline::TextBaseLine::MIDDLE => {
                TextBaseLine::MIDDLE
            }
            canvas_2d::context::text_styles::text_baseline::TextBaseLine::ALPHABETIC => {
                TextBaseLine::ALPHABETIC
            }
            canvas_2d::context::text_styles::text_baseline::TextBaseLine::IDEOGRAPHIC => {
                TextBaseLine::IDEOGRAPHIC
            }
            canvas_2d::context::text_styles::text_baseline::TextBaseLine::BOTTOM => {
                TextBaseLine::BOTTOM
            }
        }
    }
}
