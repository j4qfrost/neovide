use super::components::sprite::{ClipOrientation, Sprite};
use super::entities::{Character, MachineType, MovementInput};
use super::physics::Physics;
use legion::{Entity, World};
use nalgebra::Vector2;
use ncollide2d::shape::{Ball, Cuboid, ShapeHandle};
use nphysics2d::object::{BodyPartHandle, ColliderDesc, Ground, RigidBodyDesc};

pub struct Level {
    name: String,
}

pub const GROUND_THICKNESS: f32 = 0.2;
pub const GROUND_HALF_EXTENTS_WIDTH: f32 = 3.0;
pub const BALL_RADIUS: f32 = 0.5;
pub const BALL_COUNT: usize = 5;

impl Level {
    pub fn new() -> Self {
        Self {
            name: "test".to_string(),
        }
    }

    pub fn init(&self, world: &mut World, physics: &mut Physics) -> Entity {
        println!("Loading level {:?}", self.name);
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
            }
        }

        let _character = Character::default();
        let sprite = Sprite::default();
        let source = &sprite.source;
        // Build the rigid body.
        let rigid_body = RigidBodyDesc::new().translation(Vector2::y()).build();

        // Insert the rigid body to the body set.
        let rigid_body_handle = physics.bodies.insert(rigid_body);

        let _character_image = &source.get_image("idle", 0, ClipOrientation::Original);

        let box_shape_handle =
            ShapeHandle::new(Cuboid::new(Vector2::new(BALL_RADIUS, BALL_RADIUS)));

        // Build the collider.
        let box_collider = ColliderDesc::new(box_shape_handle)
            .density(1.0)
            .build(BodyPartHandle(rigid_body_handle, 0));

        // Insert the collider to the body set.
        physics.colliders.insert(box_collider);

        world.push((
            rigid_body_handle,
            MachineType::Character(Character::default()),
            sprite,
            MovementInput::default(),
        ))
    }
}
