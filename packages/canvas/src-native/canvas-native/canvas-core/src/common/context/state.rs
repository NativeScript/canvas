use crate::common::context::Context;

impl Context {
    pub fn save(&mut self) {
        self.surface.canvas().save();
        let stack = self.state.clone();
        self.state_stack.push(stack);
    }

    pub fn restore(&mut self) {
        if let Some(state) = self.state_stack.pop() {
            self.surface.canvas().restore();
            self.state = state;
        }
    }
}
