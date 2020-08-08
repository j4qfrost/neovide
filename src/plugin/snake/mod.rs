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

pub struct SnakePlugin {
    snake: Snake,
}

impl<'a> Plugin<'a> for SnakePlugin {
    fn update(&mut self, window: &'a mut WindowWrapper) -> i32 {
        self.snake.move_all();
        let scale_factor = (1.0 / Sdl2Window::new(&window.window).scale_factor()) as i32;
        if !self.snake.collide(scale_factor) {
            window.vimming = true;
        }

        0
    }

    fn draw(&mut self, window: &'a mut WindowWrapper) -> i32 {
        let snake = &mut self.snake;
        let sdl_window_wrapper = Sdl2Window::new(&window.window);
        let tail = snake.pop_tail();
        let error = window
            .skulpin_renderer
            .draw(
                &sdl_window_wrapper,
                |canvas: &mut Canvas, coordinate_system_helper: CoordinateSystemHelper| {
                    snake.draw(canvas, &coordinate_system_helper, tail);
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

impl<'a> SdlEventHandler<'a> for SnakePlugin {
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
                    // REDRAW_SCHEDULER.queue_next_frame();
                    window
                        .window
                        .set_size(window.cached_size.0, window.cached_size.1)
                        .unwrap();
                    return 1;
                }
                Some(Keycode::Up) | Some(Keycode::W) => {
                    self.snake.set_direction(0);
                }
                Some(Keycode::Left) | Some(Keycode::A) => {
                    self.snake.set_direction(1);
                }
                Some(Keycode::Down) | Some(Keycode::S) => {
                    self.snake.set_direction(2);
                }
                Some(Keycode::Right) | Some(Keycode::D) => {
                    self.snake.set_direction(3);
                }
                _ => {}
            },
            _ => {}
        }

        0
    }
}

impl<'a> SnakePlugin {
    pub fn new() -> Self {
        Self {
            snake: Snake::new(),
        }
    }

    pub fn process_events(&mut self, window: &'a mut WindowWrapper, event_pump: &mut EventPump) {
        for event in event_pump.poll_iter() {
            if self.handle(window, &event) != 0 {
                break;
            }
        }
    }
}
