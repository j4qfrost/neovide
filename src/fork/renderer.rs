use super::game::*;
use super::physics::*;
use legion::IntoQuery;
use nphysics2d::object::{Body, BodySet, DefaultBodyHandle};
use skulpin::skia_safe::{colors, matrix, paint, Canvas, Color, Color4f, Paint, Point, Rect};
use skulpin::winit::dpi::LogicalSize;
use skulpin::CoordinateSystemHelper;

use super::game::MachineType;

pub struct Renderer {
    pub logical_size: LogicalSize<u32>,
}

impl Default for Renderer {
    fn default() -> Self {
        Self {
            logical_size: LogicalSize::new(640, 480),
        }
    }
}

impl Renderer {
    pub fn draw(
        &mut self,
        canvas: &mut Canvas,
        coordinate_system_helper: &CoordinateSystemHelper,
        game: &Game,
    ) -> bool {
        let x_half_extents = GROUND_HALF_EXTENTS_WIDTH * 1.5;
        let y_half_extents = x_half_extents
            / (coordinate_system_helper.surface_extents().width as f32
                / coordinate_system_helper.surface_extents().height as f32);

        coordinate_system_helper
            .use_visible_range(
                canvas,
                Rect {
                    left: -x_half_extents,
                    right: x_half_extents,
                    top: y_half_extents + 1.0,
                    bottom: -y_half_extents + 1.0,
                },
                matrix::ScaleToFit::Center,
            )
            .unwrap();

        // Generally would want to clear data every time we draw
        canvas.clear(Color::from_argb(0, 0, 0, 255));

        // Make a color to draw with
        let mut paint = Paint::new(Color4f::new(0.0, 1.0, 0.0, 1.0), None);
        paint.set_anti_alias(true);
        paint.set_style(paint::Style::Stroke);
        paint.set_stroke_width(0.02);

        canvas.draw_rect(
            Rect {
                left: -GROUND_HALF_EXTENTS_WIDTH,
                top: 0.0,
                right: GROUND_HALF_EXTENTS_WIDTH,
                bottom: -GROUND_THICKNESS,
            },
            &paint,
        );

        // construct a query from a "view tuple"
        let mut query = <(&DefaultBodyHandle, &MachineType)>::query();

        // this time we have &Velocity and &mut Position
        for (handle, machine_type) in query.iter(&game.world) {
            let body = game.physics.bodies.rigid_body(*handle).unwrap();
            match machine_type {
                MachineType::Sphere(machine) => machine.draw(canvas, body.position()),
                MachineType::Character(machine) => machine.draw(canvas, body.position()),
            }
        }

        coordinate_system_helper.use_logical_coordinates(canvas);

        true
    }
}
