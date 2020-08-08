use crate::window::WindowWrapper;

pub trait Plugin<'a> {
    fn update(&mut self, window: &'a mut WindowWrapper) -> i32;
    fn draw(&mut self, window: &'a mut WindowWrapper) -> i32;
}

use skulpin::sdl2::event::Event;

pub trait SdlEventHandler<'a> {
    fn handle(&mut self, window: &'a mut WindowWrapper, event: &Event) -> i32;
}

pub mod snake;
