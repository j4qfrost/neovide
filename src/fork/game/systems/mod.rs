use super::components::animate::Animate;
use super::Physics;
use legion::system;
use nphysics2d::object::{DefaultBodyHandle, DefaultBodySet, DefaultColliderSet};
use std::time::{Duration, Instant};

pub struct DeltaTime {
    instant: Instant,
}

impl Default for DeltaTime {
    fn default() -> Self {
        Self {
            instant: Instant::now(),
        }
    }
}

#[system(for_each)]
pub fn animate_entities(
    anim: &mut Animate,
    body_handle: &DefaultBodyHandle,
    #[resource] time: &mut DeltaTime,
    #[resource] bodies: &mut DefaultBodySet<f32>,
) {
    let refresh_rate = 7.5;
    let frame_length = Duration::from_secs_f32(1.0 / refresh_rate);

    if time.instant.elapsed() >= frame_length {
        anim.animate(body_handle, bodies);
        time.instant = Instant::now();
    }
}

#[system]
pub fn physics(
    #[resource] bodies: &mut DefaultBodySet<f32>,
    #[resource] colliders: &mut DefaultColliderSet<f32>,
    #[resource] physics: &mut Physics,
) {
    for _ in 1..physics.nsteps {
        physics.step(bodies, colliders);
    }
}
