use skia_safe::{Image, Shader, TileMode};

use crate::common::context::filter_quality::FilterQuality;
use crate::common::context::matrix::Matrix;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum Repetition {
    Repeat = 0,
    RepeatX = 1,
    RepeatY = 2,
    NoRepeat = 3,
}

impl From<i32> for Repetition {
    fn from(value: i32) -> Self {
        match value {
            0 => Repetition::Repeat,
            1 => Repetition::RepeatX,
            2 => Repetition::RepeatY,
            3 => Repetition::NoRepeat,
            _ => Repetition::Repeat,
        }
    }
}

impl Into<i32> for Repetition {
    fn into(self) -> i32 {
        match self {
            Repetition::Repeat => 0,
            Repetition::RepeatX => 1,
            Repetition::RepeatY => 2,
            Repetition::NoRepeat => 3,
        }
    }
}

#[derive(Clone)]
pub struct Pattern {
    image: Image,
    repetition: Repetition,
    matrix: skia_safe::Matrix,
}

impl Pattern {
    pub fn to_pattern_shader(
        pattern: &Pattern,
        image_smoothing_quality: FilterQuality,
    ) -> Option<Shader> {
        let mode: (TileMode, TileMode) = match pattern.repetition {
            Repetition::NoRepeat => (TileMode::Clamp, TileMode::Clamp),
            Repetition::RepeatX => (TileMode::Repeat, TileMode::Clamp),
            Repetition::RepeatY => (TileMode::Clamp, TileMode::Repeat),
            _ => (TileMode::Repeat, TileMode::Repeat),
        };
        pattern
            .image()
            .to_shader(Some(mode), image_smoothing_quality, Some(&pattern.matrix))
    }

    pub fn new(image: Image, repetition: Repetition) -> Self {
        Self {
            image,
            repetition,
            matrix: skia_safe::Matrix::default(),
        }
    }

    pub fn set_pattern_transform(&mut self, matrix: &Matrix) {
        /* let mut affine: [f32; 6] = [0f32; 6];
        let slice = matrix.affine();
        affine.copy_from_slice(slice.as_slice());
        self.matrix.set_affine(&affine);*/
        let matrix = matrix.matrix.to_m33();
        self.matrix.pre_concat(&matrix);
    }

    pub(crate) fn matrix_mut(&mut self) -> &mut skia_safe::Matrix {
        &mut self.matrix
    }

    pub fn image(&self) -> &Image {
        &self.image
    }

    pub fn matrix(&self) -> &skia_safe::Matrix {
        &self.matrix
    }
}
