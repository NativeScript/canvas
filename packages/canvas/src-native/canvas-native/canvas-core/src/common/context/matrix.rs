use std::ops::{Index, IndexMut};
use std::os::raw::c_float;

use skia_safe::M44;

#[derive(Clone)]
pub struct Matrix {
    pub(crate) matrix: skia_safe::M44,
}

impl Default for Matrix {
    fn default() -> Self {
        Self::new()
    }
}

impl From<&skia_safe::Matrix> for Matrix {
    fn from(matrix: &skia_safe::Matrix) -> Self {
        Self {
            matrix: M44::from(matrix),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Member3D {
    M11 = 0,
    M12 = 1,
    M13 = 2,
    M14 = 3,
    M21 = 4,
    M22 = 5,
    M23 = 6,
    M24 = 7,
    M31 = 8,
    M32 = 9,
    M33 = 10,
    M34 = 11,
    M41 = 12,
    M42 = 13,
    M43 = 14,
    M44 = 15,
}

impl Index<Member3D> for [f32] {
    type Output = f32;

    fn index(&self, index: Member3D) -> &Self::Output {
        &self[index as usize]
    }
}

impl IndexMut<Member3D> for [f32] {
    fn index_mut(&mut self, index: Member3D) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Member2D {
    A = 0,
    B = 1,
    C = 4,
    D = 5,
    E = 12,
    F = 13,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Member2DName {
    ScaleX = 0,
    SkewX = 4,
    TransX = 12,
    SkewY = 1,
    ScaleY = 5,
    TransY = 13,
    Persp0 = 3,
    Persp1 = 7,
    Persp2 = 15,
}

impl Index<Member2D> for [f32] {
    type Output = f32;

    fn index(&self, index: Member2D) -> &Self::Output {
        &self[index as usize]
    }
}

impl IndexMut<Member2D> for [f32] {
    fn index_mut(&mut self, index: Member2D) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

impl Index<Member2DName> for [f32] {
    type Output = f32;

    fn index(&self, index: Member2DName) -> &Self::Output {
        &self[index as usize]
    }
}

impl IndexMut<Member2DName> for [f32] {
    fn index_mut(&mut self, index: Member2DName) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

impl Matrix {
    pub fn new() -> Self {
        Self {
            matrix: skia_safe::M44::new_identity(),
        }
    }

    fn member_2d(&self, member: Member2D) -> c_float {
        let mut m = [
            1.0f32, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ];
        self.matrix.get_col_major(&mut m);
        m[member]
    }

    fn set_member_2d(&mut self, member: Member2D, value: c_float) {
        let mut m = [
            1.0f32, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ];
        self.matrix.get_col_major(&mut m);
        m[member] = value;
        self.matrix = M44::row_major(&m);
    }

    fn member_2d_name(&self, member: Member2DName) -> c_float {
        let mut m = [
            1.0f32, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ];
        self.matrix.get_col_major(&mut m);
        m[member]
    }

    fn set_member_2d_name(&mut self, member: Member2DName, value: c_float) {
        let mut m = [
            1.0f32, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ];
        self.matrix.get_col_major(&mut m);
        m[member] = value;
        self.matrix = M44::row_major(&m);
    }

    fn member_3d(&self, member: Member3D) -> c_float {
        let mut m = [
            1.0f32, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ];
        self.matrix.get_col_major(&mut m);
        m[member]
    }

    fn set_member_3d(&mut self, member: Member3D, value: c_float) {
        let mut m = [
            1.0f32, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ];
        self.matrix.get_col_major(&mut m);
        m[member] = value;
        self.matrix = M44::row_major(&m);
    }

    pub fn affine(&self) -> Vec<c_float> {
        vec![self.a(), self.b(), self.c(), self.d(), self.e(), self.f()]
    }

    pub fn set_affine(&mut self, value: &[c_float; 6]) {
        let mut m = [
            1.0f32, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ];
        self.matrix.get_col_major(&mut m);
        m[Member2D::A] = value[0];
        m[Member2D::B] = value[1];
        m[Member2D::C] = value[2];
        m[Member2D::D] = value[3];
        m[Member2D::E] = value[4];
        m[Member2D::F] = value[5];
        self.matrix = M44::row_major(&m);
    }

    pub fn a(&self) -> c_float {
        self.member_2d(Member2D::A)
    }

    pub fn set_a(&mut self, a: c_float) {
        self.set_member_2d(Member2D::A, a)
    }

    pub fn b(&self) -> c_float {
        self.member_2d(Member2D::B)
    }

    pub fn set_b(&mut self, b: c_float) {
        self.set_member_2d(Member2D::B, b)
    }

    pub fn skew_x(&mut self) -> c_float {
        self.member_2d_name(Member2DName::SkewX)
    }

    pub fn set_skew_x(&mut self, x: c_float) {
        self.set_member_2d_name(Member2DName::SkewX, x)
    }

    pub fn skew_y(&mut self) -> c_float {
        self.member_2d_name(Member2DName::SkewY)
    }

    pub fn set_skew_y(&mut self, y: c_float) {
        self.set_member_2d_name(Member2DName::SkewY, y)
    }

    pub fn c(&self) -> c_float {
        self.member_2d(Member2D::C)
    }

    pub fn set_c(&mut self, c: c_float) {
        self.set_member_2d(Member2D::C, c)
    }

    pub fn d(&self) -> c_float {
        self.member_2d(Member2D::D)
    }

    pub fn set_d(&mut self, d: c_float) {
        self.set_member_2d(Member2D::D, d)
    }

    pub fn e(&self) -> c_float {
        self.member_2d(Member2D::E)
    }

    pub fn set_e(&mut self, e: c_float) {
        self.set_member_2d(Member2D::E, e)
    }

    pub fn f(&self) -> c_float {
        self.member_2d(Member2D::F)
    }

    pub fn set_f(&mut self, f: c_float) {
        self.set_member_2d(Member2D::F, f)
    }

    pub fn m11(&self) -> c_float {
        self.member_3d(Member3D::M11)
    }

    pub fn set_m11(&mut self, m11: c_float) {
        self.set_member_3d(Member3D::M11, m11)
    }

    pub fn m12(&self) -> c_float {
        self.member_3d(Member3D::M12)
    }

    pub fn set_m12(&mut self, m12: c_float) {
        self.set_member_3d(Member3D::M12, m12)
    }

    pub fn m13(&self) -> c_float {
        self.member_3d(Member3D::M13)
    }

    pub fn set_m13(&mut self, m13: c_float) {
        self.set_member_3d(Member3D::M13, m13)
    }

    pub fn m14(&self) -> c_float {
        self.member_3d(Member3D::M14)
    }

    pub fn set_m14(&mut self, m14: c_float) {
        self.set_member_3d(Member3D::M14, m14)
    }

    pub fn m21(&self) -> c_float {
        self.member_3d(Member3D::M21)
    }

    pub fn set_m21(&mut self, m21: c_float) {
        self.set_member_3d(Member3D::M21, m21)
    }

    pub fn m22(&self) -> c_float {
        self.member_3d(Member3D::M22)
    }

    pub fn set_m22(&mut self, m22: c_float) {
        self.set_member_3d(Member3D::M22, m22)
    }

    pub fn m23(&self) -> c_float {
        self.member_3d(Member3D::M23)
    }

    pub fn set_m23(&mut self, m23: c_float) {
        self.set_member_3d(Member3D::M23, m23)
    }

    pub fn m24(&self) -> c_float {
        self.member_3d(Member3D::M24)
    }

    pub fn set_m24(&mut self, m24: c_float) {
        self.set_member_3d(Member3D::M24, m24)
    }

    pub fn m31(&self) -> c_float {
        self.member_3d(Member3D::M31)
    }

    pub fn set_m31(&mut self, m31: c_float) {
        self.set_member_3d(Member3D::M31, m31)
    }

    pub fn m32(&self) -> c_float {
        self.member_3d(Member3D::M32)
    }

    pub fn set_m32(&mut self, m32: c_float) {
        self.set_member_3d(Member3D::M32, m32)
    }

    pub fn m33(&self) -> c_float {
        self.member_3d(Member3D::M33)
    }

    pub fn set_m33(&mut self, m33: c_float) {
        self.set_member_3d(Member3D::M33, m33)
    }

    pub fn m34(&self) -> c_float {
        self.member_3d(Member3D::M34)
    }

    pub fn set_m34(&mut self, m34: c_float) {
        self.set_member_3d(Member3D::M34, m34)
    }

    pub fn m41(&self) -> c_float {
        self.member_3d(Member3D::M41)
    }

    pub fn set_m41(&mut self, m41: c_float) {
        self.set_member_3d(Member3D::M41, m41)
    }

    pub fn m42(&self) -> c_float {
        self.member_3d(Member3D::M42)
    }

    pub fn set_m42(&mut self, m42: c_float) {
        self.set_member_3d(Member3D::M42, m42)
    }

    pub fn m43(&self) -> c_float {
        self.member_3d(Member3D::M43)
    }

    pub fn set_m43(&mut self, m43: c_float) {
        self.set_member_3d(Member3D::M43, m43)
    }

    pub fn m44(&self) -> c_float {
        self.member_3d(Member3D::M44)
    }

    pub fn set_m44(&mut self, m44: c_float) {
        self.set_member_3d(Member3D::M44, m44)
    }
}
