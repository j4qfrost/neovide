#[cfg_attr(feature = "sdl2", path = "sdl2.rs")]
#[cfg_attr(feature = "winit", path = "winit.rs")]
pub mod window_wrapper;
pub use window_wrapper::*;

#[cfg(feature = "winit")]
pub mod manager;
#[cfg(feature = "winit")]
pub use manager::*;
