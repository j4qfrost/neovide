use rand::Rng;
use std::collections::LinkedList;

use skulpin::skia_safe::gpu::SurfaceOrigin;
use skulpin::skia_safe::{colors, Budgeted, Canvas, Color, IPoint, Paint, Rect, Surface};
use skulpin::CoordinateSystemHelper;

struct Food {
    pub location: Rect,
}

impl Food {
    fn new(boundaries: (f32, f32), scale: f32) -> Self {
        Self {
            location: Food::drop_food(boundaries, scale),
        }
    }

    fn drop_food(boundaries: (f32, f32), scale: f32) -> Rect {
        let mut rng = rand::thread_rng();

        let n1: u32 = rng.gen_range(1, (boundaries.0 / scale) as u32) * scale as u32 - scale as u32;
        let n2: u32 = rng.gen_range(1, (boundaries.1 / scale) as u32) * scale as u32 - scale as u32;
        let f1 = n1 as f32;
        let f2 = n2 as f32;
        Rect::new(f1, f2, f1 + scale, f2 + scale)
    }
}

pub struct Snake {
    head: Rect,
    tail: LinkedList<Rect>,
    direction: i32,
    step: f32,
    scale: f32,
    color: Color,
    grow: bool,
    play_area: (f32, f32),
    food: Food,
    surface: Option<Surface>,
}

impl Snake {
    pub fn new() -> Self {
        let scale = 10.0;
        let head = Rect::new(scale, 0.0, 2.0 * scale, scale);
        let mut tail = LinkedList::new();
        tail.push_back(Rect::new(-scale, 0.0, 0.0, scale));
        tail.push_back(Rect::new(0.0, 0.0, scale, scale));
        let play_area = (640.0, 480.0);
        Snake {
            head,
            tail,
            direction: 3,
            step: 10.0,
            scale,
            color: Color::GREEN,
            grow: false,
            play_area,
            food: Food::new(play_area, scale),
            surface: None,
        }
    }

    // 0 - up, 1 - left, 2 - down, 3 - right
    pub fn set_direction(&mut self, direction: i32) {
        let switch = (self.direction + direction) % 2;
        if switch == 1 {
            self.direction = direction;
        }
    }

    pub fn collide(&mut self) -> bool {
        let loc = (self.head.x() as i32 * 2, self.head.y() as i32 * 2);
        if let Some(mut surface) = self.surface.take() {
            let image = surface.image_snapshot();
            let image = image.new_non_texture_image().unwrap();
            if let Some(pm) = image.peek_pixels() {
                let color = pm.get_color(IPoint::from(loc));
                self.surface = Some(surface);
                if color == self.color {
                    return false;
                } else if color == Color::WHITE {
                    self.grow = true;
                    self.food.location = Food::drop_food(self.play_area, self.scale);
                }
            }
        }
        true
    }

    pub fn pop_tail(&mut self) -> Option<Rect> {
        if !self.grow {
            return self.tail.pop_front();
        }
        self.grow = false;
        None
    }

    fn next_move(direction: i32, step: f32, boundaries: (f32, f32), s: &Rect) -> (f32, f32) {
        match direction {
            0 => (s.x(), (s.y() - step + boundaries.1) % boundaries.1),
            1 => ((s.x() - step + boundaries.0) % boundaries.0, s.y()),
            2 => (s.x(), (s.y() + step) % boundaries.1),
            3 => ((s.x() + step) % boundaries.0, s.y()),
            _ => (s.x(), s.y()),
        }
    }

    pub fn move_all(&mut self) {
        self.tail.push_back(Rect::new(
            self.head.left(),
            self.head.top(),
            self.head.right(),
            self.head.bottom(),
        ));

        let next_move = Snake::next_move(self.direction, self.step, self.play_area, &self.head);
        self.head.set_xywh(
            next_move.0,
            next_move.1,
            self.head.width(),
            self.head.height(),
        );
    }

    pub fn draw(
        &mut self,
        gpu_canvas: &mut Canvas,
        coordinate_system_helper: &CoordinateSystemHelper,
        tail: Option<Rect>,
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

        // draw and logic
        let green_paint = Paint::new(colors::GREEN, None);
        let black_paint = Paint::new(colors::BLACK, None);
        let white_paint = Paint::new(colors::WHITE, None);

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
