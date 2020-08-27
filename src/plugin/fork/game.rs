use rand::Rng;
use std::collections::LinkedList;

use skulpin::skia_safe::gpu::SurfaceOrigin;
use skulpin::skia_safe::{colors, Budgeted, Canvas, Color, IPoint, Paint, Rect, Surface};
use skulpin::CoordinateSystemHelper;

use specs::{Builder, World, WorldExt, Entity, Dispatcher};
use specs_physics::{
    colliders::Shape,
    nalgebra::{Isometry3, Vector3},
    nphysics::{algebra::Velocity3, object::BodyStatus},
    physics_dispatcher,
    PhysicsBodyBuilder,
    PhysicsColliderBuilder,
    SimplePosition,
};

pub struct Game<'a> {
    surface: Option<Surface>,
    pub world: World,
    pub entity: Entity,
    pub dispatcher: Dispatcher<'a, 'a>,
}

impl Game<'_> {
    pub fn new() -> Self {
        // initialise the Specs world; this will contain our Resources and Entities
        let mut world = World::new();

        // create the dispatcher containing all relevant Systems; alternatively to using
        // the convenience function you can add all required Systems by hand
        let mut dispatcher = physics_dispatcher::<f32, SimplePosition<f32>>();
        dispatcher.setup(&mut world);

        // create an Entity with a dynamic PhysicsBody component and a velocity
        let entity = world
            .create_entity()
            .with(SimplePosition::<f32>(Isometry3::<f32>::translation(
                1.0, 67.0, 1.0,
            )))
            .with(
                PhysicsBodyBuilder::<f32>::from(BodyStatus::Dynamic)
                    .velocity(Velocity3::linear(1.0, 0.0, 0.0))
                    .build(),
            )
            .with(
                PhysicsColliderBuilder::<f32>::from(Shape::Cuboid {
                    half_extents: Vector3::new(1.9, 2.0, 1.0),
                })
                .margin(0.1)
                .build(),
            )
            .build();

        // create an Entity with a static PhysicsBody component right next to the first
        // one
        world
            .create_entity()
            .with(SimplePosition::<f32>(Isometry3::<f32>::translation(
                5.0, 1.0, 1.0,
            )))
            .with(PhysicsBodyBuilder::<f32>::from(BodyStatus::Static).build())
            .with(
                PhysicsColliderBuilder::<f32>::from(Shape::Cuboid {
                    half_extents: Vector3::new(1.9, 2.0, 1.0),
                })
                .margin(0.1)
                .build(),
            )
            .build();

        // execute the dispatcher
        dispatcher.dispatch(&world);

        Self { surface: None, world, entity, dispatcher }
    }

    pub fn update(&mut self) {
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

        let pos_storage = self.world.read_storage::<SimplePosition<f32>>();
        let pos = pos_storage.get(self.entity).unwrap();
        let black_paint = Paint::new(colors::BLACK, None);
        let white_paint = Paint::new(colors::WHITE, None);

        let vector = pos.0.translation.vector;
        let rect = Rect::new(
            vector.x,
            vector.y,
            vector.x + 10.0,
            vector.y + 10.0
        );

        canvas.draw_rect(rect, &white_paint);

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
