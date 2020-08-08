use log::{debug, error, info, trace};
use crate::window::WindowWrapper;
use skulpin::sdl2::keyboard::Keycode;

pub trait Plugin<'a> {
    fn update(&mut self, window: &'a mut WindowWrapper) -> i32;
    fn draw(&mut self, window: &'a mut WindowWrapper) -> i32;
}

use skulpin::sdl2::event::{Event, WindowEvent};
use skulpin::sdl2::EventPump;
pub trait SdlEventHandler<'a> {
    fn handle(&mut self, window: &'a mut WindowWrapper, event: &Event) -> i32;
}

use skulpin::{
    CoordinateSystem, CoordinateSystemHelper, LogicalSize, PhysicalSize, PresentMode,
    Renderer as SkulpinRenderer, RendererBuilder, Sdl2Window, Window,
};

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
        return 0;
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
                    window.window
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
        return 0;
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

use rand::Rng;
use std::collections::LinkedList;

use skulpin::skia_safe::gpu::SurfaceOrigin;
use skulpin::skia_safe::{colors, Budgeted, Canvas, Color, IPoint, Paint, Rect, Surface};
// use skulpin::CoordinateSystemHelper;

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

    pub fn collide(&mut self, dpi_scale: i32) -> bool {
        let loc = (
            self.head.x() as i32 * dpi_scale,
            self.head.y() as i32 * dpi_scale,
        );
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
