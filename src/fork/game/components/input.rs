use super::animate::Animate;
use skulpin::winit::event::ElementState;
use skulpin::winit::event::VirtualKeyCode as Keycode;

type ProcessFunction = fn(Option<Keycode>, &ElementState, &mut Animate);

pub struct KeyInputHandler {
    process_fn: ProcessFunction,
}

impl KeyInputHandler {
    pub fn new(process_fn: ProcessFunction) -> Self {
        Self { process_fn }
    }

    pub fn process(
        &self,
        keycode: Option<Keycode>,
        key_state: &ElementState,
        controlled_character: &mut Animate,
    ) {
        (self.process_fn)(keycode, key_state, controlled_character);
    }
}
