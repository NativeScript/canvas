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