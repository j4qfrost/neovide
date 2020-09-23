use super::physics::*;
use legion::*;

pub struct Game {
    pub world: World,
    pub physics: Physics,
    pub nsteps: usize,
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

        Self {
            world,
            physics,
            nsteps: 3,
        }
    }
}
