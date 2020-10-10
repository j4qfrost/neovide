use super::physics::*;
use legion::*;
use skulpin::winit::event::VirtualKeyCode as Keycode;

use image;

// use super::deno::Deno;
use super::python::Python;

use cache_macro::cache;
use image::GenericImageView;
use lru::LruCache;
use nier::*;
use nier_macros::*;
use nphysics2d::math::Isometry;
use skulpin::skia_safe::colors;
use skulpin::skia_safe::AlphaType;
use skulpin::skia_safe::Canvas;
use skulpin::skia_safe::ColorInfo;
use skulpin::skia_safe::ColorSpace;
use skulpin::skia_safe::canvas::SrcRectConstraint;
use skulpin::skia_safe::ColorType;
use skulpin::skia_safe::Data;
use skulpin::skia_safe::ISize;
use skulpin::skia_safe::Image;
use skulpin::skia_safe::ImageInfo;
use skulpin::skia_safe::Rect;
use skulpin::skia_safe::IRect;
use skulpin::skia_safe::Paint;
use skulpin::skia_safe::Point;
use std::collections::HashMap;
use std::hash::Hash;
use std::path::PathBuf;
use image::DynamicImage;
// // Used for physics
use na::Vector2;

pub struct Game {
    pub world: World,
    pub physics: Physics,
    pub nsteps: usize,
    pub python: Python,
    character_handle: Entity,
}

impl Default for Game {
    fn default() -> Self {
        let mut world = World::default();
        let mut physics = Physics::new();
        let mut python = Python::default();
        python.init();

        let level = Level::new();
        let character_handle = level.init(&mut world, &mut physics);

        Self {
            world,
            physics,
            nsteps: 3,
            python,
            character_handle,
        }
    }
}

// This example does a physics demo, because physics is fun :)

use nalgebra as na;

// Used for physics
use ncollide2d::shape::{Ball, Cuboid, ShapeHandle};
use nphysics2d::force_generator::DefaultForceGeneratorSet;
use nphysics2d::joint::DefaultJointConstraintSet;
use nphysics2d::object::{
    BodyPartHandle, ColliderDesc, DefaultBodyHandle, DefaultBodySet, DefaultColliderSet, Ground,
    RigidBodyDesc,
};

use skulpin::winit::event::ElementState;

impl Game {
    fn load_level(&mut self, level: Level) {
        level.init(&mut self.world, &mut self.physics);
    }
}

struct Level {
    name: String,
}

pub const GROUND_THICKNESS: f32 = 0.2;
pub const GROUND_HALF_EXTENTS_WIDTH: f32 = 3.0;
pub const BALL_RADIUS: f32 = 0.5;
pub const BALL_COUNT: usize = 5;

impl Level {
    fn new() -> Self {
        Self {
            name: "test".to_string(),
        }
    }

    fn init(&self, world: &mut World, physics: &mut Physics) -> Entity {
        // A rectangle that the balls will fall on
        let ground_shape = ShapeHandle::new(Cuboid::new(Vector2::new(
            GROUND_HALF_EXTENTS_WIDTH,
            GROUND_THICKNESS,
        )));

        // Build a static ground body and add it to the body set.
        let ground_body_handle = physics.bodies.insert(Ground::new());

        // Build the collider.
        let ground_collider = ColliderDesc::new(ground_shape)
            .translation(Vector2::y() * -GROUND_THICKNESS)
            .build(BodyPartHandle(ground_body_handle, 0));

        // Add the collider to the collider set.
        physics.colliders.insert(ground_collider);

        let ball_shape_handle = ShapeHandle::new(Ball::new(BALL_RADIUS));

        let shift = (BALL_RADIUS + ColliderDesc::<f32>::default_margin()) * 2.0;
        let centerx = shift * (BALL_COUNT as f32) / 2.0;
        let centery = shift / 2.0;
        let height = 3.0;

        for i in 0usize..BALL_COUNT {
            for j in 0usize..BALL_COUNT {
                let x = i as f32 * shift - centerx;
                let y = j as f32 * shift + centery + height;

                // Build the rigid body.
                let rigid_body = RigidBodyDesc::new().translation(Vector2::new(x, y)).build();

                // Insert the rigid body to the body set.
                let rigid_body_handle = physics.bodies.insert(rigid_body);

                // Build the collider.
                let ball_collider = ColliderDesc::new(ball_shape_handle.clone())
                    .density(1.0)
                    .build(BodyPartHandle(rigid_body_handle, 0));

                // Insert the collider to the body set.
                physics.colliders.insert(ball_collider);

                world.push((rigid_body_handle, MachineType::Sphere(Sphere::default())));
            }
        }

        let character = Character::default();
        let clips = character.source().unwrap().clips;
        // Build the rigid body.
        let rigid_body = RigidBodyDesc::new().translation(Vector2::y()).build();

        // Insert the rigid body to the body set.
        let rigid_body_handle = physics.bodies.insert(rigid_body);

        let character_image = &clips.get("idle").unwrap()[0].get(ClipOrientation::Original);

        let box_shape_handle = ShapeHandle::new(Cuboid::new(Vector2::new(
            BALL_RADIUS, BALL_RADIUS
        )));

        // Build the collider.
        let box_collider = ColliderDesc::new(box_shape_handle.clone())
            .density(1.0)
            .build(BodyPartHandle(rigid_body_handle, 0));

        // Insert the collider to the body set.
        physics.colliders.insert(box_collider);

        world.push((
            rigid_body_handle,
            MachineType::Character(Character::default()),
            MovementInput::default(),
        ))
    }
}

pub trait Sprite {
    fn source(&self) -> Option<SpriteSheet> {
        None
    }

    fn draw(&self, canvas: &mut Canvas, isometry: &Isometry<f32>);
}

pub trait Animate {
    fn animate(&mut self);
}

pub struct Sphere {
    paint: Paint,
}

impl Default for Sphere {
    fn default() -> Self {
        let paint = Paint::new(colors::GREEN, None);
        Self { paint }
    }
}

impl Sprite for Sphere {
    fn draw(&self, canvas: &mut Canvas, isometry: &Isometry<f32>) {
        let position = isometry.translation;

        canvas.draw_circle(Point::new(position.x, position.y), BALL_RADIUS, &self.paint);
    }
}

pub enum MachineType {
    Character(Character),
    Sphere(Sphere),
}

#[derive(Default)]
pub struct MovementInput {}

impl MovementInput {
    pub fn process(keycode: Option<Keycode>, controlled_character: &mut Character) {
        match keycode.unwrap() {
            Keycode::Left => {
                controlled_character.state =
                    Character::delta(&controlled_character.state, CharacterInput::Left).unwrap()
            }
            Keycode::Right => {
                controlled_character.state =
                    Character::delta(&controlled_character.state, CharacterInput::Right).unwrap()
            }
            _ => {}
        }
    }
    pub fn interrupt(keycode: Option<Keycode>, controlled_character: &mut Character) {
        println!("{:?}", keycode);
        controlled_character.state =
            Character::delta(&controlled_character.state, CharacterInput::Interrupt).unwrap();
        controlled_character.ticks = 0;
    }
}

impl Game {
    pub fn send(&mut self, keycode: Option<Keycode>, key_state: ElementState) {
        // construct a query from a "view tuple"
        let mut query = <(&MovementInput, &mut MachineType)>::query();
        if let Ok((_, machine_type)) = query.get_mut(&mut self.world, self.character_handle) {
            match machine_type {
                MachineType::Character(machine) => {
                    if key_state == ElementState::Pressed {
                        MovementInput::process(keycode, machine);
                    } else {
                        MovementInput::interrupt(keycode, machine);
                    }
                }
                _ => {}
            }
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub enum CharacterState {
    Idle,
    RunningLeft,
    RunningRight,
}

#[derive(Debug, Copy, Clone)]
pub enum CharacterInput {
    Left,
    Right,
    Interrupt,
}

#[derive(Automaton)]
#[nier(state = "CharacterState")]
pub struct Character {
    source_path: String,
    pub ticks: u32,
    pub state: CharacterState,
}

impl Default for Character {
    fn default() -> Self {
        let mut display_root = PathBuf::new();
        display_root.push(env!("CARGO_MANIFEST_DIR"));
        display_root.push("src/fork/res/adventurer-Sheet.png");
        let source_path = display_root.to_str().unwrap().to_string();
        Self {
            source_path,
            ticks: 0,
            state: CharacterState::Idle,
        }
    }
}

impl Deterministic<CharacterState, CharacterInput> for Character {
    fn initial() -> CharacterState {
        CharacterState::Idle
    }

    fn delta(
        state: &CharacterState,
        input: CharacterInput,
    ) -> Result<CharacterState, Reject<CharacterState, CharacterInput>> {
        match (state, input) {
            (_, CharacterInput::Left) => Ok(CharacterState::RunningLeft),
            (_, CharacterInput::Right) => Ok(CharacterState::RunningRight),
            (_, CharacterInput::Interrupt) => Ok(CharacterState::Idle),
        }
    }
}

fn make_skia_image(img: &DynamicImage) -> Image {
    let (w, h) = img.dimensions();
    let bytes = img.to_bytes();
    let data = unsafe { Data::new_bytes(&bytes) };

    let color_info = ColorInfo::new(
        ColorType::RGBA8888,
        AlphaType::Opaque,
        ColorSpace::new_srgb(),
    );
    let size = ISize::new(w as i32, h as i32);
    let img_info = ImageInfo::from_color_info(size, color_info);
    Image::from_raster_data(&img_info, data, w as usize * img_info.bytes_per_pixel()).unwrap()
}

#[cache(LruCache : LruCache::new(1))]
fn source_character(source_path: String) -> SpriteSheet {
    let img = image::open(source_path).unwrap();
    let (w, h) = img.dimensions();
    let clip_w = w as i32 / 7;
    let clip_h = h as i32 / 11;

    let mut clips = HashMap::new();
    // Idle
    let idle_clips = vec![
        Clip::new(&img, &IRect::from_xywh(0, 0, clip_w, clip_h), true),
        Clip::new(&img, &IRect::from_xywh(clip_w, 0, clip_w, clip_h), true),
        Clip::new(&img, &IRect::from_xywh(clip_w * 2, 0, clip_w, clip_h), true),
        Clip::new(&img, &IRect::from_xywh(clip_w * 3, 0, clip_w, clip_h), true),
    ];
    clips.insert("idle".to_string(), idle_clips);

    // Crouch
    // let crouch_clips = vec![
    //     Clip::new(&img, &IRect::from_xywh(clip_w * 4, 0, clip_w, clip_h), true),
    //     Clip::new(&img, &IRect::from_xywh(clip_w * 5, 0, clip_w, clip_h), true),
    //     Clip::new(&img, &IRect::from_xywh(clip_w * 6, 0, clip_w, clip_h), true),
    //     Clip::new(&img, &IRect::from_xywh(0, clip_h, clip_w, clip_h), true),
    // ];
    // clips.insert("crouch".to_string(), crouch_clips);

    // Running
    let running_clips = vec![
        Clip::new(&img, &IRect::from_xywh(clip_w, clip_h, clip_w, clip_h), true),
        Clip::new(&img, &IRect::from_xywh(clip_w * 2, clip_h, clip_w, clip_h), true),
        Clip::new(&img, &IRect::from_xywh(clip_w * 3, clip_h, clip_w, clip_h), true),
        Clip::new(&img, &IRect::from_xywh(clip_w * 4, clip_h, clip_w, clip_h), true),
        Clip::new(&img, &IRect::from_xywh(clip_w * 5, clip_h, clip_w, clip_h), true),
        Clip::new(&img, &IRect::from_xywh(clip_w * 6, clip_h, clip_w, clip_h), true),
        Clip::new(&img, &IRect::from_xywh(0, clip_h, clip_w * 2, clip_h), true),
        Clip::new(&img, &IRect::from_xywh(clip_w, clip_h, clip_w * 2, clip_h), true),
    ];
    clips.insert("running".to_string(), running_clips);

    SpriteSheet {
        clips,
    }
}

impl Sprite for Character {
    fn source(&self) -> Option<SpriteSheet> {
        Some(source_character(self.source_path.clone()))
    }

    fn draw(&self, canvas: &mut Canvas, isometry: &Isometry<f32>) {
        let source = source_character(self.source_path.clone());
        let clip = match self.state {
            CharacterState::Idle => source.get_image("idle", self.ticks as usize, ClipOrientation::Original),
            CharacterState::RunningLeft => source.get_image("running", self.ticks as usize, ClipOrientation::Flipped),
            CharacterState::RunningRight => source.get_image("running", self.ticks as usize, ClipOrientation::Original),
        };

        let img = make_skia_image(clip);

        let position = isometry.translation;
        let paint = Paint::new(colors::RED, None);

        let rect = Rect::from_xywh(position.x - 0.5, position.y - 0.5, 1.0, 1.0);
        
        // Debug
        {
            let p1 = Point::new(position.x - 0.5, position.y - 0.5);
            let p2 = Point::new(position.x - 0.5, position.y + 0.5);
            let p3 = Point::new(position.x + 0.5, position.y + 0.5);
            let p4 = Point::new(position.x + 0.5, position.y - 0.5);
            canvas.draw_line(p1, p2, &paint);
            canvas.draw_line(p2, p3, &paint);
            canvas.draw_line(p3, p4, &paint);
            canvas.draw_line(p4, p1, &paint);
        }

        canvas.draw_image_rect(img, None, rect, &paint);
    }
}

impl Animate for Character {
    fn animate(&mut self) {
        let states = match self.state {
            CharacterState::Idle => 4,
            CharacterState::RunningLeft | CharacterState::RunningRight => 12,
        };
        self.ticks = (self.ticks + 1) % states;
    }
}

#[derive(Clone)]
pub struct SpriteSheet {
    clips: HashMap<String, Vec<Clip>>,
}

impl SpriteSheet {
    pub fn get_image(&self, key: &str, it: usize, orientation: ClipOrientation) -> &DynamicImage {
        self.clips.get(key).unwrap()[it].get(orientation)
    }
}

#[derive(Clone)]
pub struct Clip {
    original: DynamicImage,
    flipped: Option<DynamicImage>,
}

impl Clip {
    pub fn new(source: &DynamicImage, rect: &IRect, is_flipped: bool) -> Self {
        let cropped = source.crop_imm(rect.x() as u32, rect.y() as u32, rect.width() as u32, rect.height() as u32);
        let original = cropped.flipv();
        let flipped = if is_flipped {
            Some(original.fliph())
        } else {
            None
        };
        Self {
            original,
            flipped,
        }
    }

    pub fn get(&self, orientation: ClipOrientation) -> &DynamicImage {
        match orientation {
            ClipOrientation::Original => &self.original,
            ClipOrientation::Flipped => self.flipped.as_ref().unwrap(),
        }
    }
}

pub enum ClipOrientation {
    Original,
    Flipped
}