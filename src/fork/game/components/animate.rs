use nphysics2d::object::{DefaultBodyHandle, DefaultBodySet};
use num_traits::FromPrimitive;

type AnimationFunction = fn(&mut Animate, &DefaultBodyHandle, &mut DefaultBodySet<f32>) -> ();

pub struct Animate {
    delta_fn: fn(u32, u32) -> u32,
    current: u32,
    animate_fn: AnimationFunction,
    pub ticks: usize,
}

impl Animate {
    pub fn new(current: u32, delta_fn: fn(u32, u32) -> u32, animate_fn: AnimationFunction) -> Self {
        Self {
            current,
            delta_fn,
            animate_fn,
            ticks: 0,
        }
    }

    pub fn delta(&mut self, input: u32) {
        self.current = (self.delta_fn)(self.current, input);
    }

    pub fn state<T: FromPrimitive>(&self) -> T {
        T::from_u32(self.current).unwrap()
    }

    pub fn animate(&mut self, body_handle: &DefaultBodyHandle, bodies: &mut DefaultBodySet<f32>) {
        (self.animate_fn)(self, body_handle, bodies);
    }
}
