use rand::Rng;
use std::collections::LinkedList;

use skulpin::skia_safe::gpu::SurfaceOrigin;
use skulpin::skia_safe::{colors, Budgeted, Canvas, Color, IPoint, Paint, Rect, Surface};
use skulpin::CoordinateSystemHelper;

pub struct Game {
    surface: Option<Surface>,
}

impl Game {
    pub fn new() -> Self {
        Self { surface: None }
    }

    pub fn draw(
        &mut self,
        gpu_canvas: &mut Canvas,
        coordinate_system_helper: &CoordinateSystemHelper,
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

        if let Some(rattle) = tail {
            canvas.draw_rect(rattle, &black_paint);
        }
        canvas.draw_rect(self.head, &green_paint);
        canvas.draw_rect(self.food.location, &white_paint);

        let image = surface.image_snapshot();
        let window_size = coordinate_system_helper.window_logical_size();
        let image_destination = Rect::new(
            0.0,
            0.0,
            window_size.width as f32,
            window_size.height as f32,
        );

        gpu_canvas.draw_image_rect(image, None, &image_destination, &black_paint);
        self.surface = Some(surface);

        true
    }
}
