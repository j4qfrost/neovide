pub mod physics;
use physics::*;
mod level;
use level::*;

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

impl Game {
    fn load_level(&mut self, level: Level) {
        level.init(&mut self.world, &mut self.physics);
    }
}

pub fn draw_character(canvas: &mut Canvas, isometry: &Isometry<f32>, source: &SpriteSheet, state: u32, ticks: u32) {
    let clip = match CharacterState::from_u32(state).unwrap() {
        CharacterState::Idle => source.get_image("idle", ticks as usize, ClipOrientation::Original),
        CharacterState::RunningLeft => source.get_image("running", ticks as usize, ClipOrientation::Flipped),
        CharacterState::RunningRight => source.get_image("running", ticks as usize, ClipOrientation::Original),
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

pub struct Sprite {
    pub draw_fn: fn(&mut Canvas, &Isometry<f32>, &SpriteSheet, u32, u32) -> (),
    pub source: SpriteSheet,
}

impl Default for Sprite {
    fn default() -> Self {
        let mut display_root = PathBuf::new();
        display_root.push(env!("CARGO_MANIFEST_DIR"));
        display_root.push("src/fork/res/adventurer-Sheet.png");
        let source_path = display_root.to_str().unwrap().to_string();
        let source = source_character(source_path);
        Self {
            draw_fn: draw_character,
            source,
        }
    }
}

pub trait Animate {
    fn animate(&mut self);
}

pub enum MachineType {
    Character(Character),
}

use skulpin::winit::event::ElementState;

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

use num_traits::FromPrimitive;
use num_traits::AsPrimitive;

#[derive(Debug, Copy, Clone, State)]
pub enum CharacterState {
    Idle = 0,
    RunningLeft = 1,
    RunningRight = 2,
}

impl FromPrimitive for CharacterState {
    fn from_u32(n: u32) -> Option<Self> {
        match n {
            0 => Some(Self::Idle),
            1 => Some(Self::RunningLeft),
            2 => Some(Self::RunningRight),
            _ => None
        }
    }

    fn from_i64(n: i64) -> Option<Self> {
        match n {
            0 => Some(Self::Idle),
            1 => Some(Self::RunningLeft),
            2 => Some(Self::RunningRight),
            _ => None
        }
    }

    fn from_u64(n: u64) -> Option<Self> {
        match n {
            0 => Some(Self::Idle),
            1 => Some(Self::RunningLeft),
            2 => Some(Self::RunningRight),
            _ => None
        }
    }
}

#[derive(Debug, Alphabet)]
pub enum CharacterInput {
    Left,
    Right,
    Interrupt,
}

#[derive(Automaton)]
#[nier(state = "CharacterState")]
pub struct Character {
    pub ticks: u32,
    pub state: CharacterState,
}

impl Default for Character {
    fn default() -> Self {
        Self {
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

impl Animate for Character {
    fn animate(&mut self) {
        let states = match self.state {
            CharacterState::Idle => 4,
            CharacterState::RunningLeft | CharacterState::RunningRight => 8,
        };
        self.ticks = (self.ticks + 1) % states;
    }
}

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