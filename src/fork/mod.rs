use crate::settings::SETTINGS;
use crate::window::manager::{NeovideEvent, NeovideEventProcessor, WindowHandle};
use crate::window::window_wrapper::WindowSettings;
use log::{error, info, trace};
use skulpin::winit::event::VirtualKeyCode as Keycode;
use skulpin::winit::event::{
    ElementState, Event, ModifiersState, MouseButton, MouseScrollDelta, StartCause, WindowEvent,
};
use skulpin::winit::event_loop::{ControlFlow, EventLoop, EventLoopProxy};
use skulpin::winit::window::{Icon, Window};
use skulpin::{
    winit::dpi::{LogicalPosition, LogicalSize, PhysicalPosition},
    Renderer as SkulpinRenderer, Window as OtherWindow, WinitWindow,
};

mod renderer;
use renderer::*;
pub mod game;
use game::*;

#[derive(Default)]
pub struct Fork {
    window: Option<Window>,
    saved_handle: Option<Box<dyn WindowHandle>>,
    game: Game,
    renderer: Renderer,
    keycode: Option<Keycode>,
    modifiers: ModifiersState,
    ignore_input_this_frame: bool,
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
        proxy: &EventLoopProxy<NeovideEvent>,
    ) -> Option<ControlFlow> {
        match e {
            WindowEvent::CloseRequested => {
                return Some(ControlFlow::Exit);
            }
            WindowEvent::DroppedFile(path) => {}
            WindowEvent::KeyboardInput { input, .. } => {
                if input.state == ElementState::Pressed {
                    if !self.ignore_input_this_frame {
                        self.keycode = input.virtual_keycode;
                    } else {
                        self.ignore_input_this_frame = false;
                    }
                }
            }
            WindowEvent::ModifiersChanged(m) => {
                self.modifiers.set(m, true);
            }
            WindowEvent::CursorMoved { position, .. } => {}
            WindowEvent::MouseWheel {
                delta: MouseScrollDelta::LineDelta(x, y),
                ..
            } => {}
            WindowEvent::MouseInput {
                button: MouseButton::Left,
                state,
                ..
            } => {
                if state == ElementState::Pressed {
                } else {
                }
            }
            WindowEvent::Focused(focus) => {}
            WindowEvent::Resized(size) => {
                let scale_factor = self.window.as_ref().unwrap().scale_factor();
                self.renderer.logical_size = size.to_logical(scale_factor);
            }
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
        true
    }

    fn should_draw(&self) -> bool {
        true
    }

    fn draw(&mut self, skulpin_renderer: &mut SkulpinRenderer) -> bool {
        if self.should_draw() {
            let renderer = &mut self.renderer;
            let world = &self.game.world;
            let window = WinitWindow::new(&self.window.as_ref().unwrap());
            let error = skulpin_renderer
                .draw(&window, |canvas, coordinate_system_helper| {
                    let dt = 1.0 / (SETTINGS.get::<WindowSettings>().refresh_rate as f32);
                    renderer.draw(canvas, &coordinate_system_helper, dt, world);
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
