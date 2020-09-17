use rand::Rng;
use std::collections::LinkedList;

use super::game::Position;
use legion::*;
use skulpin::skia_safe::gpu::SurfaceOrigin;
use skulpin::skia_safe::{colors, Budgeted, Canvas, Color, IPoint, Paint, Rect, Surface};
use skulpin::winit::dpi::LogicalSize;
use skulpin::CoordinateSystemHelper;

pub struct Renderer {
    surface: Option<Surface>,
    pub logical_size: LogicalSize<u32>,
}

impl Default for Renderer {
    fn default() -> Self {
        Self {
            surface: None,
            logical_size: LogicalSize::new(640, 480),
        }
    }
}

impl Renderer {
    pub fn draw(
        &mut self,
        gpu_canvas: &mut Canvas,
        coordinate_system_helper: &CoordinateSystemHelper,
        dt: f32,
        world: &World,
    ) -> bool {
        let mut surface = self.surface.take().unwrap_or_else(|| {
            let mut context = gpu_canvas.gpu_context().unwrap();
            let budgeted = Budgeted::Yes;
            let image_info = gpu_canvas.image_info();
            let surface_origin = SurfaceOrigin::TopLeft;
            Surface::new_render_target(
                &mut context,
                budgeted,
                &image_info,
                None,
                surface_origin,
                None,
                None,
            )
            .expect("Could not create surface")
        });

        let mut canvas = surface.canvas();
        coordinate_system_helper.use_logical_coordinates(&mut canvas);

        let green_paint = Paint::new(colors::GREEN, None);
        let mut query = <&Position>::query();

        for position in query.iter(world) {
            let rect = Rect {
                left: position.x,
                top: position.y,
                right: position.x + 10.0,
                bottom: position.y + 10.0,
            };
            canvas.draw_rect(rect, &green_paint);
            println!("{:?}", position);
        }

        let image = surface.image_snapshot();
        let window_size = coordinate_system_helper.window_logical_size();
        let image_destination = Rect::new(
            0.0,
            0.0,
            window_size.width as f32,
            window_size.height as f32,
        );

        let black_paint = Paint::new(colors::BLACK, None);
        gpu_canvas.draw_image_rect(image, None, &image_destination, &black_paint);
        self.surface = Some(surface);

        true
    }
}
