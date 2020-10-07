use super::physics::*;
use legion::*;
use skulpin::winit::event::VirtualKeyCode as Keycode;

use image;

// use super::deno::Deno;
use super::python::Python;

pub struct Game {
    pub world: World,
    pub physics: Physics,
    pub nsteps: usize,
    pub python: Python,
    // controlled_character: Character,
}

impl Default for Game {
    fn default() -> Self {
        let mut world = World::default();
        let mut physics = Physics::new();
        let mut python = Python::default();
        python.init();

        let level = Level::new();
        level.init(&mut world, &mut physics);

        Self {
            world,
            physics,
            nsteps: 3,
            python,
        }
    }
}

// This example does a physics demo, because physics is fun :)

use nalgebra as na;

// Used for physics
use na::Vector2;
use ncollide2d::shape::{Ball, Cuboid, ShapeHandle};
use nphysics2d::force_generator::DefaultForceGeneratorSet;
use nphysics2d::joint::DefaultJointConstraintSet;
use nphysics2d::object::{
    BodyPartHandle, ColliderDesc, DefaultBodyHandle, DefaultBodySet, DefaultColliderSet, Ground,
    RigidBodyDesc,
};

impl Game {
    fn load_level(&mut self, level: Level) {
        level.init(&mut self.world, &mut self.physics);
    }
}

struct Level {
    name: String,
}

pub const GROUND_THICKNESS: f32 = 0.2;
pub const GROUND_HALF_EXTENTS_WIDTH: f32 = 3.0;
pub const BALL_RADIUS: f32 = 0.2;
pub const BALL_COUNT: usize = 5;

impl Level {
    fn new() -> Self {
        Self {
            name: "test".to_string(),
        }
    }

    fn init(&self, world: &mut World, physics: &mut Physics) {
        // A rectangle that the balls will fall on
        let ground_shape = ShapeHandle::new(Cuboid::new(Vector2::new(
            GROUND_HALF_EXTENTS_WIDTH,
            GROUND_THICKNESS,
        )));

        // Build a static ground body and add it to the body set.
        let ground_body_handle = physics.bodies.insert(Ground::new());

        // Build the collider.
        let ground_collider = ColliderDesc::new(ground_shape)
            .translation(Vector2::y() * -GROUND_THICKNESS)
            .build(BodyPartHandle(ground_body_handle, 0));

        // Add the collider to the collider set.
        physics.colliders.insert(ground_collider);

        let ball_shape_handle = ShapeHandle::new(Ball::new(BALL_RADIUS));

        let shift = (BALL_RADIUS + ColliderDesc::<f32>::default_margin()) * 2.0;
        let centerx = shift * (BALL_COUNT as f32) / 2.0;
        let centery = shift / 2.0;
        let height = 3.0;

        for i in 0usize..BALL_COUNT {
            for j in 0usize..BALL_COUNT {
                let x = i as f32 * shift - centerx;
                let y = j as f32 * shift + centery + height;

                // Build the rigid body.
                let rigid_body = RigidBodyDesc::new().translation(Vector2::new(x, y)).build();

                // Insert the rigid body to the body set.
                let rigid_body_handle = physics.bodies.insert(rigid_body);

                // Build the collider.
                let ball_collider = ColliderDesc::new(ball_shape_handle.clone())
                    .density(1.0)
                    .build(BodyPartHandle(rigid_body_handle, 0));

                // Insert the collider to the body set.
                physics.colliders.insert(ball_collider);

                world.push((rigid_body_handle, MachineType::Sphere(Sphere::default())));
            }
        }
    }
}

use nphysics2d::math::Isometry;
use skulpin::skia_safe::colors;
use skulpin::skia_safe::Canvas;
use skulpin::skia_safe::Paint;
use skulpin::skia_safe::Point;

pub trait Sprite {
    fn draw(&self, canvas: &mut Canvas, isometry: &Isometry<f32>);
}

pub struct Sphere {
    paint: Paint,
}

impl Default for Sphere {
    fn default() -> Self {
        let paint = Paint::new(colors::GREEN, None);
        Self { paint }
    }
}

impl Sprite for Sphere {
    fn draw(&self, canvas: &mut Canvas, isometry: &Isometry<f32>) {
        let position = isometry.translation;

        canvas.draw_circle(Point::new(position.x, position.y), BALL_RADIUS, &self.paint);
    }
}

pub enum MachineType {
    // Character(Character),
    Sphere(Sphere),
}

// impl Game {
//     pub fn send(keycode: Option<Keycode>) {
//         match keycode.unwrap() {
//             Keycode::Left => controlled_character.state = controlled_character.delta(controlled_character.state, CharacterInput::Left),
//             Keycode::Up => controlled_character.state = controlled_character.delta(controlled_character.state, CharacterInput::Up),
//             Keycode::Right => controlled_character.state = controlled_character.delta(controlled_character.state, CharacterInput::Right),
//             Keycode::Down => controlled_character.state = controlled_character.delta(controlled_character.state, CharacterInput::Down),
//             _ => {},
//         }
//     }

//     pub fn interrupt(keycode: Option<Keycode>) {
//         println!("{:?}", keycode);
//         controlled_character.state = controlled_character.delta(controlled_character.state, CharacterInput::Interrupt);
//     }
// }

// use std::collections::HashMap;
// use std::hash::Hash;

// struct Rect {
//     pub x: u32,
//     pub y: u32,
//     pub w: u32,
//     pub h: u32,
// }

// impl Rect {
//     pub fn new(x: u32, y: u32, w: u32, h: u32) -> Self {
//         Self {
//             x, y, w, h
//         }
//     }
// }

// use nier::*;
// use nier_macros::*;

// use nalgebra as na;

// // Used for physics
// use na::Vector2;

// #[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, State)]
// enum CharacterState {
//     Idle,
//     Running(Vector2),
// }

// #[derive(Debug, Copy, Clone, Alphabet)]
// enum CharacterInput {
//     Left,
//     Up,
//     Right,
//     Down,
//     Interrupt,
// }

// #[derive(Automaton)]
// #[nier(state = "CharacterState")]
// struct Character {
//     sprite_sheet: SpriteSheet,
//     pub state: CharacterState,
// }

// impl Deterministic<CharacterState, CharacterInput> for Character {
//     fn initial() -> CharacterState {
//         CharacterState::Idle
//     }

//     fn delta(state: &CharacterState, input: CharacterInput) -> Result<CharacterState, Reject<CharacterState, CharacterInput>> {
//         match (state, input) {
//             (_, CharacterInput::Left) => Ok(CharacterState::Running(-Vector2::x())),
//             (_, CharacterInput::Up) => Ok(CharacterState::Running(Vector2::y())),
//             (_, CharacterInput::Right) => Ok(CharacterState::Running(Vector2::x())),
//             (_, CharacterInput::Down) => Ok(CharacterState::Running(-Vector2::y())),
//             (_, CharacterInput::Interrupt) => Ok(CharacterState::Idle),
//         }
//     }
// }

// trait Animate: Deterministic<S, I> {
//     fn consume(&mut self, input: I);
//     fn update();
//     fn draw();
// }

// impl Animate<CharacterState, CharacterInput> for Character {
//     fn consume(&mut self, input: I) {
//         self.state = self.delta(&self.state, input);
//     }

//     fn update(&self) {

//     }
// }

// struct SpriteSheet {
//     source: image::RgbaImage,
//     sprites: HashMap<dyn State, Rect>,
// }

// impl SpriteSheet {
//     pub fn new(file_name: String) -> Self {
//         let source = image::load(file_name).unwrap();

//         Self {
//             source,
//             sprites: HashMap::new(),
//         }
//     }
// }
