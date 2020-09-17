use legion::*;
use skulpin::skia_safe::Rect;

pub struct Game {
    pub world: World,
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
        let mut world = World::default();
        let entities: &[Entity] = world.extend(vec![
            (Position { x: 0.0, y: 0.0 }, Velocity { dx: 0.0, dy: 0.0 }),
            (Position { x: 1.0, y: 1.0 }, Velocity { dx: 0.0, dy: 0.0 }),
            (Position { x: 2.0, y: 2.0 }, Velocity { dx: 0.0, dy: 0.0 }),
        ]);
        Self { world }
    }
}
