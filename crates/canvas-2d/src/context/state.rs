use crate::context::Context;

impl Context {
    #[inline]
    pub fn save(&mut self) {
        let canvas = self.canvas();
        let matrix = canvas.local_to_device();
        canvas.save();
        let mut saved = self.state.clone();
        saved.saved_matrix = matrix;
        self.state_stack.push(saved);
        self.state.clips.clear();
    }

    #[inline]
    pub fn restore(&mut self) {
        if let Some(state) = self.state_stack.pop() {
            self.canvas().restore();
            self.state = state;
        }
    }
}
