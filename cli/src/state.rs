use gl_toolkit::ClearFlag;

pub trait State: Sized {
    fn new() -> Self;

    fn key_up(&self);
    fn key_down(&self);
    fn resize(&self, width: u32, height: u32);
    fn update(&self, elapsed_time: f32);
    fn render(&self);
}

pub struct FiniteStateMachine<S: State> {
    states: Vec<S>,
}

impl<S: State> FiniteStateMachine<S> {
    pub fn new() -> FiniteStateMachine<S> {
        FiniteStateMachine {
            states: Vec::new(),
        }
    }

    pub fn push(&mut self, state: S) {
        self.states.push(state);
    }

    pub fn pop(&mut self) {
        self.states.pop();
    }

    pub fn update(&self, elapsed_time: f32) {
        if let Some(state) = self.states.last() {
            state.update(elapsed_time);
        }
    }

    pub fn render(&self) {
        gl_toolkit::clear(ClearFlag::Color | ClearFlag::Depth);

        if let Some(state) = self.states.last() {
            state.render();
        }
    }
}
