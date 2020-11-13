use skia_safe::{Data, Image, Shader, TileMode};

use crate::common::context::matrix::Matrix;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum Repetition {
    Repeat = 0,
    RepeatX = 1,
    RepeatY = 2,
    NoRepeat = 3,
}

#[derive(Clone)]
pub struct Pattern {
    image: Image,
    repetition: Repetition,
    matrix: skia_safe::Matrix,
}

impl Pattern {
    pub fn to_pattern_shader(pattern: &Pattern) -> Shader {
        let mode: (TileMode, TileMode) = match pattern.repetition {
            Repetition::NoRepeat => (TileMode::Clamp, TileMode::Clamp),
            Repetition::RepeatX => (TileMode::Repeat, TileMode::Clamp),
            Repetition::RepeatY => (TileMode::Clamp, TileMode::Repeat),
            _ => (TileMode::Repeat, TileMode::Repeat),
        };
        pattern.image().to_shader(mode, pattern.matrix())
    }

    pub fn new(image: Image, repetition: Repetition) -> Self {
        Self {
            image,
            repetition,
            matrix: skia_safe::Matrix::new_identity(),
        }
    }

    pub fn set_pattern_transform(&mut self, matrix: &Matrix) {
        let mut affine: [f32;6] = [0f32;6];
        let slice = matrix.affine();
        affine.copy_from_slice(slice.as_slice());
        self.matrix.set_affine(&affine);
    }

    pub fn image(&self) -> &Image {
        &self.image
    }

    pub fn matrix(&self) -> &skia_safe::Matrix {
        &self.matrix
    }
}
