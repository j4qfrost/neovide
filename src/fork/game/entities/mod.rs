use nier::*;
use nier_macros::*;
use num_traits::FromPrimitive;
use skulpin::winit::event::VirtualKeyCode as Keycode;

#[derive(Debug, Copy, Clone, State)]
pub enum CharacterState {
    Idle = 0,
    RunningLeft = 1,
    RunningRight = 2,
}

pub trait Animate {
    fn animate(&mut self);
}

pub enum MachineType {
    Character(Character),
}

impl FromPrimitive for CharacterState {
    fn from_u32(n: u32) -> Option<Self> {
        match n {
            0 => Some(Self::Idle),
            1 => Some(Self::RunningLeft),
            2 => Some(Self::RunningRight),
            _ => None,
        }
    }

    fn from_i64(n: i64) -> Option<Self> {
        match n {
            0 => Some(Self::Idle),
            1 => Some(Self::RunningLeft),
            2 => Some(Self::RunningRight),
            _ => None,
        }
    }

    fn from_u64(n: u64) -> Option<Self> {
        match n {
            0 => Some(Self::Idle),
            1 => Some(Self::RunningLeft),
            2 => Some(Self::RunningRight),
            _ => None,
        }
    }
}

#[derive(Debug, Alphabet)]
pub enum CharacterInput {
    Left,
    Right,
    Interrupt,
}

#[derive(Automaton)]
#[nier(state = "CharacterState")]
pub struct Character {
    pub ticks: u32,
    pub state: CharacterState,
}

impl Default for Character {
    fn default() -> Self {
        Self {
            ticks: 0,
            state: CharacterState::Idle,
        }
    }
}

impl Deterministic<CharacterState, CharacterInput> for Character {
    fn initial() -> CharacterState {
        CharacterState::Idle
    }

    fn delta(
        state: &CharacterState,
        input: CharacterInput,
    ) -> Result<CharacterState, Reject<CharacterState, CharacterInput>> {
        match (state, input) {
            (_, CharacterInput::Left) => Ok(CharacterState::RunningLeft),
            (_, CharacterInput::Right) => Ok(CharacterState::RunningRight),
            (_, CharacterInput::Interrupt) => Ok(CharacterState::Idle),
        }
    }
}

impl Animate for Character {
    fn animate(&mut self) {
        let states = match self.state {
            CharacterState::Idle => 4,
            CharacterState::RunningLeft | CharacterState::RunningRight => 8,
        };
        self.ticks = (self.ticks + 1) % states;
    }
}

#[derive(Default)]
pub struct MovementInput {}

impl MovementInput {
    pub fn process(keycode: Option<Keycode>, controlled_character: &mut Character) {
        match keycode.unwrap() {
            Keycode::Left => {
                controlled_character.state =
                    Character::delta(&controlled_character.state, CharacterInput::Left).unwrap()
            }
            Keycode::Right => {
                controlled_character.state =
                    Character::delta(&controlled_character.state, CharacterInput::Right).unwrap()
            }
            _ => {}
        }
    }
    pub fn interrupt(_keycode: Option<Keycode>, controlled_character: &mut Character) {
        controlled_character.state =
            Character::delta(&controlled_character.state, CharacterInput::Interrupt).unwrap();
        controlled_character.ticks = 0;
    }
}
