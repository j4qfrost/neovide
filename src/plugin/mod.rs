use crate::window::WindowWrapper;

#[cfg(feature = "sdl2")]
pub trait SDLPlugin<'a> {
    fn update(&mut self, window: &'a mut WindowWrapper) -> i32;
    fn draw(&mut self, window: &'a mut WindowWrapper) -> i32;
}

#[cfg(feature = "sdl2")]
use skulpin::sdl2::event::Event;

#[cfg(feature = "winit")]
use skulpin::winit::event::Event;

#[cfg(feature = "sdl2")]
pub trait SDLEventHandler<'a> {
    fn handle(&mut self, window: &'a mut WindowWrapper, event: &Event) -> i32;
}

// pub mod snake;
pub mod fork;
