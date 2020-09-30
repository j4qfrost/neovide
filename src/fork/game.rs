use super::physics::*;
use legion::*;

// use super::deno::Deno;
use super::python::Python;

pub struct Game {
    pub world: World,
    pub physics: Physics,
    pub nsteps: usize,
    pub python: Python,
}

// a component is any type that is 'static, sized, send and sync
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Velocity {
    pub dx: f32,
    pub dy: f32,
}

impl Default for Game {
    fn default() -> Self {
        let world = World::default();
        let physics = Physics::new();
        let mut python = Python::default();
        python.init();

        Self {
            world,
            physics,
            nsteps: 3,
            python,
        }
    }
}
