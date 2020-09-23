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
use nphysics2d::world::{DefaultGeometricalWorld, DefaultMechanicalWorld};

pub const GROUND_THICKNESS: f32 = 0.2;
pub const GROUND_HALF_EXTENTS_WIDTH: f32 = 3.0;
pub const BALL_RADIUS: f32 = 0.2;
pub const GRAVITY: f32 = -9.81;
pub const BALL_COUNT: usize = 5;

// Will contain all the physics simulation state
pub struct Physics {
    geometrical_world: DefaultGeometricalWorld<f32>,
    mechanical_world: DefaultMechanicalWorld<f32>,

    pub bodies: DefaultBodySet<f32>,
    colliders: DefaultColliderSet<f32>,
    joint_constraints: DefaultJointConstraintSet<f32>,
    force_generators: DefaultForceGeneratorSet<f32>,

    pub circle_body_handles: Vec<DefaultBodyHandle>,
}

impl Physics {
    pub fn new() -> Self {
        let geometrical_world = DefaultGeometricalWorld::<f32>::new();
        let mechanical_world = DefaultMechanicalWorld::new(Vector2::y() * GRAVITY);

        let mut bodies = DefaultBodySet::<f32>::new();
        let mut colliders = DefaultColliderSet::new();
        let joint_constraints = DefaultJointConstraintSet::<f32>::new();
        let force_generators = DefaultForceGeneratorSet::<f32>::new();

        // A rectangle that the balls will fall on
        let ground_shape = ShapeHandle::new(Cuboid::new(Vector2::new(
            GROUND_HALF_EXTENTS_WIDTH,
            GROUND_THICKNESS,
        )));

        // Build a static ground body and add it to the body set.
        let ground_body_handle = bodies.insert(Ground::new());

        // Build the collider.
        let ground_collider = ColliderDesc::new(ground_shape)
            .translation(Vector2::y() * -GROUND_THICKNESS)
            .build(BodyPartHandle(ground_body_handle, 0));

        // Add the collider to the collider set.
        colliders.insert(ground_collider);

        let ball_shape_handle = ShapeHandle::new(Ball::new(BALL_RADIUS));

        let shift = (BALL_RADIUS + ColliderDesc::<f32>::default_margin()) * 2.0;
        let centerx = shift * (BALL_COUNT as f32) / 2.0;
        let centery = shift / 2.0;
        let height = 3.0;

        let mut circle_body_handles = vec![];

        for i in 0usize..BALL_COUNT {
            for j in 0usize..BALL_COUNT {
                let x = i as f32 * shift - centerx;
                let y = j as f32 * shift + centery + height;

                // Build the rigid body.
                let rigid_body = RigidBodyDesc::new().translation(Vector2::new(x, y)).build();

                // Insert the rigid body to the body set.
                let rigid_body_handle = bodies.insert(rigid_body);

                // Build the collider.
                let ball_collider = ColliderDesc::new(ball_shape_handle.clone())
                    .density(1.0)
                    .build(BodyPartHandle(rigid_body_handle, 0));

                // Insert the collider to the body set.
                colliders.insert(ball_collider);

                circle_body_handles.push(rigid_body_handle);
            }
        }

        Self {
            geometrical_world,
            mechanical_world,
            bodies,
            colliders,
            joint_constraints,
            force_generators,
            circle_body_handles,
        }
    }

    pub fn step(&mut self) {
        // Run the simulation.
        self.mechanical_world.step(
            &mut self.geometrical_world,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.joint_constraints,
            &mut self.force_generators,
        );
    }
}
