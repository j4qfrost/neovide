pub mod physics;
use physics::*;
mod level;
use legion::*;
mod systems;
use level::*;
use skulpin::winit::event::VirtualKeyCode as Keycode;
// use super::deno::Deno;
use super::python::Python;
pub mod components;
pub mod entities;
use components::animate::Animate;
use components::input::MovementInput;
use skulpin::winit::event::ElementState;

pub struct Game {
    pub world: World,
    pub schedule: Schedule,
    pub resources: Resources,
    pub physics: Physics,
    pub nsteps: usize,
    pub python: Python,
    character_handle: Entity,
}

impl Default for Game {
    fn default() -> Self {
        let mut world = World::default();
        let schedule = Schedule::builder()
            .add_system(systems::animate_entities_system())
            .build();
        let resources = Resources::default();
        let mut physics = Physics::new();
        let mut python = Python::default();
        python.init();

        let level = Level::new();
        let character_handle = level.init(&mut world, &mut physics);

        Self {
            world,
            schedule,
            resources,
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
        let mut query = <&mut Animate>::query();
        if let Ok(animate) = query.get_mut(&mut self.world, self.character_handle) {
            if key_state == ElementState::Pressed {
                MovementInput::process(keycode, animate);
            } else {
                MovementInput::interrupt(keycode, animate);
            }
        }
    }
}
