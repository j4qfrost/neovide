use num_traits::FromPrimitive;

pub struct Animate {
    delta_fn: fn(u32, u32) -> u32,
    current: u32,
    // animate_fn: fn(&mut Self) -> (),
    pub ticks: u32,
}

impl Animate {
    pub fn new(
        current: u32,
        delta_fn: fn(u32, u32) -> u32,
        animate_fn: fn(&mut Self) -> (),
    ) -> Self {
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
}
