pub mod physics;
use physics::*;
mod level;
use legion::*;
mod systems;
use level::*;
use skulpin::winit::event::VirtualKeyCode as Keycode;
use systems::DeltaTime;
// use super::deno::Deno;
use super::python::Python;
pub mod components;
pub mod entities;
use components::animate::Animate;
use components::input::KeyInputHandler;
use skulpin::winit::event::ElementState;

pub struct Game {
    pub world: World,
    pub schedule: Schedule,
    pub resources: Resources,
    pub nsteps: usize,
    pub python: Python,
    character_handle: Entity,
}

impl Default for Game {
    fn default() -> Self {
        let mut world = World::default();
        let schedule = Schedule::builder()
            .add_system(systems::physics_system())
            .add_system(systems::animate_entities_system())
            .build();
        let mut resources = Resources::default();
        resources.insert(DeltaTime::default());

        let physics = Physics::new(&mut resources);
        resources.insert(physics);

        let mut python = Python::default();
        python.init();

        let level = Level::new();
        let character_handle = level.init(&mut world, &mut resources);

        Self {
            world,
            schedule,
            resources,
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
        let mut query = <(&KeyInputHandler, &mut Animate)>::query();
        if let Ok((input_handler, animate)) = query.get_mut(&mut self.world, self.character_handle)
        {
            input_handler.process(keycode, &key_state, animate);
        }
    }
}
