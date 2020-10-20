use legion::Resources;
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
    pub joint_constraints: DefaultJointConstraintSet<f32>,
    pub force_generators: DefaultForceGeneratorSet<f32>,
    pub nsteps: usize,
}

impl Physics {
    pub fn new(resources: &mut Resources) -> Self {
        let geometrical_world = DefaultGeometricalWorld::<f32>::new();
        let mechanical_world = DefaultMechanicalWorld::new(Vector2::y() * GRAVITY);

        let bodies = DefaultBodySet::<f32>::new();
        let colliders = DefaultColliderSet::<f32>::new();
        let joint_constraints = DefaultJointConstraintSet::<f32>::new();
        let force_generators = DefaultForceGeneratorSet::<f32>::new();

        resources.insert(bodies);
        resources.insert(colliders);

        Self {
            geometrical_world,
            mechanical_world,
            joint_constraints,
            force_generators,
            nsteps: 3,
        }
    }

    pub fn step(
        &mut self,
        bodies: &mut DefaultBodySet<f32>,
        colliders: &mut DefaultColliderSet<f32>,
    ) {
        // Run the simulation.
        self.mechanical_world.step(
            &mut self.geometrical_world,
            bodies,
            colliders,
            &mut self.joint_constraints,
            &mut self.force_generators,
        );
    }
}
