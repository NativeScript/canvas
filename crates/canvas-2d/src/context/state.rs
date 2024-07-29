use crate::context::Context;

impl Context {
    pub fn save(&mut self) {
        let stack = self.state.clone();
        self.state_stack.push(stack);
        self.with_canvas(|canvas| {
            canvas.save();
        })
    }

    pub fn restore(&mut self) {
        if let Some(state) = self.state_stack.pop() {
            self.with_canvas(|canvas| {
                canvas.restore();
            });
            self.state = state;
        }
    }
}
