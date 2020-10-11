pub mod physics;
use physics::*;
mod level;
use legion::*;
use level::*;
use skulpin::winit::event::VirtualKeyCode as Keycode;
// use super::deno::Deno;
use super::python::Python;
pub mod components;
pub mod entities;
use entities::{MachineType, MovementInput};
use skulpin::winit::event::ElementState;

pub struct Game {
    pub world: World,
    pub physics: Physics,
    pub nsteps: usize,
    pub python: Python,
    character_handle: Entity,
}

impl Default for Game {
    fn default() -> Self {
        let mut world = World::default();
        let mut physics = Physics::new();
        let mut python = Python::default();
        python.init();

        let level = Level::new();
        let character_handle = level.init(&mut world, &mut physics);

        Self {
            world,
            physics,
            nsteps: 3,
            python,
            character_handle,
        }
    }
}

impl Game {
    // fn load_level(&mut self, level: Level) {
    //     level.init(&mut self.world, &mut self.physics);
    // }
}

impl Game {
    pub fn send(&mut self, keycode: Option<Keycode>, key_state: ElementState) {
        // construct a query from a "view tuple"
        let mut query = <(&MovementInput, &mut MachineType)>::query();
        if let Ok((_, machine_type)) = query.get_mut(&mut self.world, self.character_handle) {
            match machine_type {
                MachineType::Character(machine) => {
                    if key_state == ElementState::Pressed {
                        MovementInput::process(keycode, machine);
                    } else {
                        MovementInput::interrupt(keycode, machine);
                    }
                } // _ => {}
            }
        }
    }
}
