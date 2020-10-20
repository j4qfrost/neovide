use log::info;
use neovide_plugin::*;
use skulpin::winit::event::WindowEvent;
use skulpin::winit::event_loop::{
    ControlFlow, EventLoopClosed, EventLoopProxy, EventLoopWindowTarget,
};
use skulpin::winit::window::{Icon, Window, WindowBuilder, WindowId};
use skulpin::{
    CoordinateSystem, PresentMode, Renderer as SkulpinRenderer, RendererBuilder, WinitWindow,
};
use std::collections::HashMap;

pub struct WindowManager {
    pub windows: HashMap<WindowId, Box<dyn WindowHandle>>,
    renderer: Option<SkulpinRenderer>,
    proxy: EventLoopProxy<NeovideEvent>,
}

impl WindowManager {
    pub fn new(proxy: EventLoopProxy<NeovideEvent>) -> Self {
        Self {
            windows: HashMap::new(),
            renderer: None,
            proxy,
        }
    }

    pub fn noop(&self) -> Result<(), EventLoopClosed<NeovideEvent>> {
        self.proxy.send_event(NeovideEvent::noop())
    }

    pub fn handle_event(&mut self, id: WindowId, event: WindowEvent) -> Option<ControlFlow> {
        if let Some(handle) = self.windows.get_mut(&id) {
            handle.process_event(event, &self.proxy)
        } else {
            None
        }
    }

    fn initialize_renderer(&mut self, window: &Window) {
        let renderer = {
            let winit_window_wrapper = WinitWindow::new(window);
            RendererBuilder::new()
                .prefer_integrated_gpu()
                .use_vulkan_debug_layer(false)
                .present_mode_priority(vec![PresentMode::Immediate])
                .coordinate_system(CoordinateSystem::Logical)
                .build(&winit_window_wrapper)
                .expect("Failed to create renderer")
        };
        self.renderer = Some(renderer);
    }

    pub fn create_window<U: 'static + WindowHandle + Default>(
        &mut self,
        title: &str,
        window_target: &EventLoopWindowTarget<NeovideEvent>,
        icon: Option<Icon>,
    ) {
        let mut handle = Box::new(U::default());
        let logical_size = handle.logical_size();

        let window = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(logical_size)
            .with_window_icon(icon)
            .build(window_target)
            .expect("Failed to create window");
        info!("window created");
        if self.renderer.is_none() {
            self.initialize_renderer(&window);
        }
        let window_id = window.id();
        handle.set_window(window);
        self.windows.insert(window_id, handle);
    }

    pub fn update_all(&mut self) -> bool {
        for handle in self.windows.values_mut() {
            if !handle.update() {
                return false;
            }
        }
        true
    }

    pub fn render_all(&mut self) -> bool {
        let mut renderer = self.renderer.as_mut().unwrap();
        for handle in self.windows.values_mut() {
            if !handle.draw(&mut renderer) {
                return false;
            }
        }
        true
    }
}
