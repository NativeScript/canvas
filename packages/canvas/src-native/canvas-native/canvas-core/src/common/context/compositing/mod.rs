use std::os::raw::c_float;

use crate::common::context::compositing::composite_operation_type::CompositeOperationType;
use crate::common::context::Context;

pub mod composite_operation_type;

impl Context {
    pub fn set_global_alpha(&mut self, alpha: c_float) {
        if alpha <= 1.0 && alpha >= 0.0 {
            self.state.global_alpha = alpha;
            self.state.paint.fill_paint_mut().set_alpha_f(alpha);
            self.state.paint.stroke_paint_mut().set_alpha_f(alpha);
            self.state.paint.image_paint_mut().set_alpha_f(alpha);
        }
    }

    pub fn global_alpha(&self) -> c_float {
        self.state.global_alpha
    }

    pub fn set_global_composite_operation(&mut self, operation: CompositeOperationType) {
        self.state.global_composite_operation = operation;
        self.state.paint.fill_paint_mut().set_blend_mode(operation.get_blend_mode());
        self.state.paint.stroke_paint_mut().set_blend_mode(operation.get_blend_mode());
        self.state.paint.image_paint_mut().set_blend_mode(operation.get_blend_mode());
    }

    pub fn global_composite_operation(&self) -> CompositeOperationType {
        self.state.global_composite_operation
    }
}
