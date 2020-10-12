use super::components::animate::Animate;
use legion::system;
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
pub fn animate_entities(anim: &mut Animate, #[resource] time: &mut DeltaTime) {
    let refresh_rate = 10.0;
    let frame_length = Duration::from_secs_f32(1.0 / refresh_rate);

    if time.instant.elapsed() >= frame_length {
        anim.animate();
        time.instant = Instant::now();
    }
}
