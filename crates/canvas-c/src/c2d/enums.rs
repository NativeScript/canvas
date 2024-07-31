use canvas_2d::context::drawing_paths::fill_rule::FillRule;
use canvas_2d::context::fill_and_stroke_styles::pattern::Repetition;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum CanvasRepetition {
    Repeat = 0,
    RepeatX = 1,
    RepeatY = 2,
    NoRepeat = 3,
}

impl Into<Repetition> for CanvasRepetition {
    fn into(self) -> Repetition {
        match self {
            CanvasRepetition::Repeat => Repetition::Repeat,
            CanvasRepetition::RepeatX => Repetition::RepeatX,
            CanvasRepetition::RepeatY => Repetition::RepeatY,
            CanvasRepetition::NoRepeat => Repetition::NoRepeat
        }
    }
}


impl From<Repetition> for CanvasRepetition {
    fn from(value: Repetition) -> Self {
        match value {
            Repetition::Repeat => Self::Repeat,
            Repetition::RepeatX => Self::RepeatX,
            Repetition::RepeatY => Self::RepeatY,
            Repetition::NoRepeat => Self::NoRepeat
        }
    }
}


#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum CanvasFillRule {
    NonZero = 0,
    EvenOdd = 1,
}

impl Into<FillRule> for CanvasFillRule {
    fn into(self) -> FillRule {
        match self {
            CanvasFillRule::NonZero => FillRule::NonZero,
            CanvasFillRule::EvenOdd => FillRule::NonZero
        }
    }
}

impl From<FillRule> for CanvasFillRule {
    fn from(value: FillRule) -> Self {
        match value {
            FillRule::NonZero => Self::NonZero,
            FillRule::EvenOdd => Self::EvenOdd,
        }
    }
}