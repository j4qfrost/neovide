use crate::window::manager::{NeovideEvent, NeovideEventProcessor, WindowHandle};
use log::error;

use skulpin::winit::event::{
    ElementState, ModifiersState, MouseButton, MouseScrollDelta, WindowEvent,
};
use skulpin::winit::event_loop::{ControlFlow, EventLoopProxy};
use skulpin::winit::window::Window;
use skulpin::{winit::dpi::LogicalSize, Renderer as SkulpinRenderer, WinitWindow};

mod renderer;
use renderer::*;
pub mod game;
use game::*;
// pub mod deno;
pub mod python;

#[derive(Default)]
pub struct Fork {
    window: Option<Window>,
    saved_handle: Option<Box<dyn WindowHandle>>,
    game: Game,
    renderer: Renderer,
    modifiers: ModifiersState,
}

impl Fork {
    pub fn save_handle(&mut self, handle: Box<dyn WindowHandle>) {
        self.saved_handle = Some(handle);
    }
}

impl NeovideEventProcessor for Fork {
    fn process_event(
        &mut self,
        e: WindowEvent,
        _proxy: &EventLoopProxy<NeovideEvent>,
    ) -> Option<ControlFlow> {
        match e {
            WindowEvent::CloseRequested => {
                return Some(ControlFlow::Exit);
            }
            // WindowEvent::DroppedFile(path) => {}
            WindowEvent::KeyboardInput { input, .. } => {
                self.game.send(input.virtual_keycode, input.state);
            }
            WindowEvent::ModifiersChanged(m) => {
                self.modifiers.set(m, true);
            }
            // WindowEvent::CursorMoved { position, .. } => {}
            WindowEvent::MouseWheel {
                delta: MouseScrollDelta::LineDelta(_x, _y),
                ..
            } => {}
            WindowEvent::MouseInput {
                button: MouseButton::Left,
                state,
                ..
            } => {
                if state == ElementState::Pressed {
                } else {
                    unimplemented!();
                }
            }
            // WindowEvent::Focused(focus) => {}
            // WindowEvent::Resized(size) => {
            //     let scale_factor = self.window.as_ref().unwrap().scale_factor();
            //     self.renderer.logical_size = size.to_logical(scale_factor);
            // }
            _ => {}
        }
        None
    }
}

impl WindowHandle for Fork {
    fn window(&mut self) -> Window {
        self.window.take().unwrap()
    }

    fn set_window(&mut self, window: Window) {
        self.window = Some(window);
    }

    fn logical_size(&self) -> LogicalSize<u32> {
        self.renderer.logical_size
    }

    fn update(&mut self) -> bool {
        self.game
            .schedule
            .execute(&mut self.game.world, &mut self.game.resources);
        true
    }

    fn should_draw(&self) -> bool {
        true
    }

    fn draw(&mut self, skulpin_renderer: &mut SkulpinRenderer) -> bool {
        if self.should_draw() {
            let renderer = &mut self.renderer;
            let game = &self.game;
            let window = WinitWindow::new(&self.window.as_ref().unwrap());
            let error = skulpin_renderer
                .draw(&window, |canvas, coordinate_system_helper| {
                    for _ in 0..3 {
                        renderer.draw(canvas, &coordinate_system_helper, game);
                    }
                })
                .is_err();
            if error {
                error!("Render failed. Closing");
                return false;
            }
        }
        true
    }
}
