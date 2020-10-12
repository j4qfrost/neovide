use super::super::entities::character::CharacterInput;
use super::animate::Animate;
use skulpin::winit::event::VirtualKeyCode as Keycode;
pub struct MovementInput {}

impl MovementInput {
    pub fn process(keycode: Option<Keycode>, controlled_character: &mut Animate) {
        match keycode.unwrap() {
            Keycode::Left => controlled_character.delta(CharacterInput::Left as u32),
            Keycode::Right => controlled_character.delta(CharacterInput::Right as u32),
            _ => {}
        }
    }
    pub fn interrupt(_keycode: Option<Keycode>, controlled_character: &mut Animate) {
        controlled_character.delta(CharacterInput::Interrupt as u32);
        controlled_character.ticks = 0;
    }
}
