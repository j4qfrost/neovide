use nalgebra::Vector2;
use nphysics2d::force_generator::DefaultForceGeneratorSet;
use nphysics2d::joint::DefaultJointConstraintSet;
use nphysics2d::object::{DefaultBodySet, DefaultColliderSet};
use nphysics2d::world::{DefaultGeometricalWorld, DefaultMechanicalWorld};

pub const GRAVITY: f32 = -9.81;

// Will contain all the physics simulation state
pub struct Physics {
    geometrical_world: DefaultGeometricalWorld<f32>,
    mechanical_world: DefaultMechanicalWorld<f32>,

    pub bodies: DefaultBodySet<f32>,
    pub colliders: DefaultColliderSet<f32>,
    pub joint_constraints: DefaultJointConstraintSet<f32>,
    pub force_generators: DefaultForceGeneratorSet<f32>,
}

impl Physics {
    pub fn new() -> Self {
        let geometrical_world = DefaultGeometricalWorld::<f32>::new();
        let mechanical_world = DefaultMechanicalWorld::new(Vector2::y() * GRAVITY);

        let bodies = DefaultBodySet::<f32>::new();
        let colliders = DefaultColliderSet::new();
        let joint_constraints = DefaultJointConstraintSet::<f32>::new();
        let force_generators = DefaultForceGeneratorSet::<f32>::new();

        Self {
            geometrical_world,
            mechanical_world,
            bodies,
            colliders,
            joint_constraints,
            force_generators,
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
