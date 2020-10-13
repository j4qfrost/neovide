use super::super::components::animate::*;
use super::super::components::sprite::*;
use image::GenericImageView;
use nphysics2d::math::Isometry;
use num_traits::FromPrimitive;
use skulpin::skia_safe::{colors, Canvas, IRect, Paint, Rect};

use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
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
            _ => None,
        }
    }

    fn from_i64(n: i64) -> Option<Self> {
        match n {
            0 => Some(Self::Idle),
            1 => Some(Self::RunningLeft),
            2 => Some(Self::RunningRight),
            _ => None,
        }
    }

    fn from_u64(n: u64) -> Option<Self> {
        match n {
            0 => Some(Self::Idle),
            1 => Some(Self::RunningLeft),
            2 => Some(Self::RunningRight),
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum CharacterInput {
    Left,
    Right,
    Interrupt,
}

impl FromPrimitive for CharacterInput {
    fn from_u32(n: u32) -> Option<Self> {
        match n {
            0 => Some(Self::Left),
            1 => Some(Self::Right),
            2 => Some(Self::Interrupt),
            _ => None,
        }
    }

    fn from_i64(n: i64) -> Option<Self> {
        match n {
            0 => Some(Self::Left),
            1 => Some(Self::Right),
            2 => Some(Self::Interrupt),
            _ => None,
        }
    }

    fn from_u64(n: u64) -> Option<Self> {
        match n {
            0 => Some(Self::Left),
            1 => Some(Self::Right),
            2 => Some(Self::Interrupt),
            _ => None,
        }
    }
}

/*
#[derive(serde)]
struct CharacterDesc {

}
*/

pub fn draw(canvas: &mut Canvas, isometry: &Isometry<f32>, source: &SpriteSheet, anim: &Animate) {
    let clip = match anim.state() {
        CharacterState::Idle => (
            source.get_clip("idle", anim.ticks),
            ClipOrientation::Original,
        ),
        CharacterState::RunningLeft => (
            source.get_clip("running", anim.ticks),
            ClipOrientation::Flipped,
        ),
        CharacterState::RunningRight => (
            source.get_clip("running", anim.ticks),
            ClipOrientation::Original,
        ),
    };
    let dyn_image = clip.0.get(clip.1);

    let img = make_skia_image(dyn_image);

    let position = isometry.translation;
    let paint = Paint::new(colors::RED, None);
    let ratio = clip.0.width_over_height;

    let rect = Rect::from_xywh(position.x - ratio / 2.0, position.y - 0.5, ratio, 1.0);

    #[cfg(feature = "bounds")]
    {
        use skulpin::skia_safe::Point;
        let p1 = Point::new(position.x - ratio / 2.0, position.y - 0.5);
        let p2 = Point::new(position.x - ratio / 2.0, position.y + 0.5);
        let p3 = Point::new(position.x + ratio / 2.0, position.y + 0.5);
        let p4 = Point::new(position.x + ratio / 2.0, position.y - 0.5);
        canvas.draw_line(p1, p2, &paint);
        canvas.draw_line(p2, p3, &paint);
        canvas.draw_line(p3, p4, &paint);
        canvas.draw_line(p4, p1, &paint);
    }

    canvas.draw_image_rect(img, None, rect, &paint);
}

pub fn delta(state: u32, input: u32) -> u32 {
    let state = CharacterState::from_u32(state).unwrap();
    let input = CharacterInput::from_u32(input).unwrap();
    match (state, input) {
        (_, CharacterInput::Left) => CharacterState::RunningLeft as u32,
        (_, CharacterInput::Right) => CharacterState::RunningRight as u32,
        (_, CharacterInput::Interrupt) => CharacterState::Idle as u32,
    }
}

pub fn animate(anim: &mut Animate) {
    let states = match anim.state() {
        CharacterState::Idle => 4,
        CharacterState::RunningLeft | CharacterState::RunningRight => 6,
    };
    anim.ticks = (anim.ticks + 1) % states;
}

pub fn source(source_path: String) -> SpriteSheet {
    let img = image::open(source_path).unwrap();
    let (w, h) = img.dimensions();
    let clip_w = w as i32 / 7;
    let clip_h = h as i32 / 11;

    let mut clips = HashMap::new();
    // Idle
    let idle_clips = vec![
        Clip::new(&img, &IRect::from_xywh(0, 0, clip_w, clip_h), true, true),
        Clip::new(
            &img,
            &IRect::from_xywh(clip_w, 0, clip_w, clip_h),
            true,
            true,
        ),
        Clip::new(
            &img,
            &IRect::from_xywh(clip_w * 2, 0, clip_w, clip_h),
            true,
            true,
        ),
        Clip::new(
            &img,
            &IRect::from_xywh(clip_w * 3, 0, clip_w, clip_h),
            true,
            true,
        ),
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
        Clip::new(
            &img,
            &IRect::from_xywh(clip_w, clip_h, clip_w, clip_h),
            true,
            true,
        ),
        Clip::new(
            &img,
            &IRect::from_xywh(clip_w * 2, clip_h, clip_w, clip_h),
            true,
            true,
        ),
        Clip::new(
            &img,
            &IRect::from_xywh(clip_w * 3, clip_h, clip_w, clip_h),
            true,
            true,
        ),
        Clip::new(
            &img,
            &IRect::from_xywh(clip_w * 4, clip_h, clip_w, clip_h),
            true,
            true,
        ),
        Clip::new(
            &img,
            &IRect::from_xywh(clip_w * 5, clip_h, clip_w, clip_h),
            true,
            true,
        ),
        Clip::new(
            &img,
            &IRect::from_xywh(clip_w * 6, clip_h, clip_w, clip_h),
            true,
            true,
        ),
    ];
    clips.insert("running".to_string(), running_clips);

    SpriteSheet::new(clips)
}
