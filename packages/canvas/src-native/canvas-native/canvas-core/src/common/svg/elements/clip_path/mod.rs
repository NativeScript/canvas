use crate::common::svg::elements::clip_path::shapes::ClipShape;
use crate::common::svg::units::Units;

pub mod shapes;

#[derive(Clone, Debug)]
pub struct ClipPath {
    units: Units,
    clips: Vec<ClipShape>,
}

impl ClipPath {
    pub fn units(&self) -> Units {
        self.units
    }

    pub fn set_units(&mut self, units: Units) {
        self.units = units;
    }

    pub fn clips(&self) -> &[ClipShape] {
        self.clips.as_slice()
    }

    pub fn set_clips(&mut self, clips: &[ClipShape]) {
        self.clips.as_mut_slice().clone_from_slice(clips)
    }

    pub fn new(units: Units, clips: &mut [ClipShape]) -> Self {
        Self {
            units,
            clips: clips.to_vec(),
        }
    }

    pub fn is_empty(&self) -> bool {
        return self.clips.len() == 1
            && match self.clips.get(0) {
            None => true,
            Some(val) => *val == ClipShape::None,
        };
    }
}

impl Default for ClipPath {
    fn default() -> Self {
        Self {
            units: Units::ObjectBoundingBox,
            clips: vec![ClipShape::None],
        }
    }
}
