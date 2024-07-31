use crate::context::Context;

impl Context {
    pub fn save(&mut self) {
        let new_state = self.state.clone();
        self.state_stack.push(new_state);
    }

    pub fn restore(&mut self) {
        if let Some(old_state) = self.state_stack.pop(){
            self.state = old_state;

            self.with_recorder(|mut recorder|{
                recorder.set_matrix(self.state.matrix);
                recorder.set_clip(&self.state.clip);
            });
        }
    }
}
