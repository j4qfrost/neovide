use crate::window::WindowWrapper;
use skulpin::sdl2::event::Event;
use skulpin::sdl2::keyboard::Keycode;
use skulpin::sdl2::EventPump;
use skulpin::skia_safe::Canvas;
use skulpin::{CoordinateSystemHelper, Sdl2Window, Window};

mod game;
use game::*;

use log::error;

use super::{Plugin, SdlEventHandler};

pub struct ForkPlugin {
    game: Game,
}

impl<'a> Plugin<'a> for ForkPlugin {
    fn update(&mut self, window: &'a mut WindowWrapper) -> i32 {
        let scale_factor = (1.0 / Sdl2Window::new(&window.window).scale_factor()) as i32;

        0
    }

    fn draw(&mut self, window: &'a mut WindowWrapper) -> i32 {
        let game = &mut self.game;
        let sdl_window_wrapper = Sdl2Window::new(&window.window);
        let error = window
            .skulpin_renderer
            .draw(
                &sdl_window_wrapper,
                |canvas: &mut Canvas, coordinate_system_helper: CoordinateSystemHelper| {
                    game.draw(canvas, &coordinate_system_helper);
                },
            )
            .is_err();
        if error {
            error!("Render failed. Closing");
            return -1;
        }

        0
    }
}

impl<'a> SdlEventHandler<'a> for GamePlugin {
    fn handle(&mut self, window: &'a mut WindowWrapper, event: &Event) -> i32 {
        match event {
            Event::Quit { .. } => {
                window.handle_quit();
                window.vimming = true;
            }
            Event::KeyDown {
                keycode: received_keycode,
                ..
            } => match received_keycode {
                // 0 - up, 1 - left, 2 - down, 3 - right
                Some(Keycode::RGui) => {
                    window.vimming = true;
                    window.renderer.surface = window.snapshot.clone();
                    window.snapshot = None;
                    window
                        .window
                        .set_size(window.cached_size.0, window.cached_size.1)
                        .unwrap();
                    return 1;
                }
                Some(Keycode::W) => {
                    self.snake.set_direction(0);
                }
                Some(Keycode::A) => {
                    self.snake.set_direction(1);
                }
                Some(Keycode::S) => {
                    self.snake.set_direction(2);
                }
                Some(Keycode::D) => {
                    self.snake.set_direction(3);
                }
                _ => {}
            },
            _ => {}
        }

        0
    }
}

impl<'a> GamePlugin {
    pub fn new() -> Self {
        Self { game: Game::new() }
    }

    pub fn process_events(&mut self, window: &'a mut WindowWrapper, event_pump: &mut EventPump) {
        for event in event_pump.poll_iter() {
            if self.handle(window, &event) != 0 {
                break;
            }
        }
    }
}
