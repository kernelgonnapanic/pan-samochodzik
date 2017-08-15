use piston::input::{Input, Button};

#[derive(Default)]
pub struct InputState {
    pub forward: bool,
    pub left: bool,
    pub right: bool,
    pub back: bool,
}

impl InputState {
    pub fn handle_event(&mut self, e: &Input) {
        use Key::*;

        macro_rules! bind {
            ($f:ident, $k:ident) => (
                match e {
                    &Input::Press(Button::Keyboard($k)) => { self.$f = true; },
                    &Input::Release(Button::Keyboard($k)) => { self.$f = false; }
                    _ => ()
                }
            )
        }

        bind!(forward, W);
        bind!(left, A);
        bind!(right, D);
        bind!(back, S);
    }
}
