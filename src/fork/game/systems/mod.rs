use super::components::animate::Animate;
use legion::system;

#[system(for_each)]
pub fn animate_entities(anim: &mut Animate, #[resource] _: &usize) {
    anim.animate();
}
