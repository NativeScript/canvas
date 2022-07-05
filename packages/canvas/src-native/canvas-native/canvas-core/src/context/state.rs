use crate::context::Context;

impl Context {

    #[inline]
    pub fn save(&mut self) {
        self.surface.canvas().save();
        let stack = self.state.clone();
        self.state_stack.push(stack);
    }

    #[inline]
    pub fn restore(&mut self) {
        if let Some(state) = self.state_stack.pop() {
            self.surface.canvas().restore();
            self.state = state;
        }
    }
}
