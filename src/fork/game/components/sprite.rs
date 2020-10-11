use super::super::entities::CharacterState;
use image::DynamicImage;
use image::GenericImageView;
use nphysics2d::math::Isometry;
use num_traits::FromPrimitive;
use skulpin::skia_safe::{
    colors, AlphaType, Canvas, ColorInfo, ColorSpace, ColorType, Data, IRect, ISize, Image,
    ImageInfo, Paint, Point, Rect,
};
use std::collections::HashMap;
use std::path::PathBuf;

fn draw_character(
    canvas: &mut Canvas,
    isometry: &Isometry<f32>,
    source: &SpriteSheet,
    state: u32,
    ticks: u32,
) {
    let clip = match CharacterState::from_u32(state).unwrap() {
        CharacterState::Idle => source.get_image("idle", ticks as usize, ClipOrientation::Original),
        CharacterState::RunningLeft => {
            source.get_image("running", ticks as usize, ClipOrientation::Flipped)
        }
        CharacterState::RunningRight => {
            source.get_image("running", ticks as usize, ClipOrientation::Original)
        }
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
        let cropped = source.crop_imm(
            rect.x() as u32,
            rect.y() as u32,
            rect.width() as u32,
            rect.height() as u32,
        );
        let original = cropped.flipv();
        let flipped = if is_flipped {
            Some(original.fliph())
        } else {
            None
        };
        Self { original, flipped }
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
    Flipped,
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

pub fn source_character(source_path: String) -> SpriteSheet {
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
        Clip::new(
            &img,
            &IRect::from_xywh(clip_w, clip_h, clip_w, clip_h),
            true,
        ),
        Clip::new(
            &img,
            &IRect::from_xywh(clip_w * 2, clip_h, clip_w, clip_h),
            true,
        ),
        Clip::new(
            &img,
            &IRect::from_xywh(clip_w * 3, clip_h, clip_w, clip_h),
            true,
        ),
        Clip::new(
            &img,
            &IRect::from_xywh(clip_w * 4, clip_h, clip_w, clip_h),
            true,
        ),
        Clip::new(
            &img,
            &IRect::from_xywh(clip_w * 5, clip_h, clip_w, clip_h),
            true,
        ),
        Clip::new(
            &img,
            &IRect::from_xywh(clip_w * 6, clip_h, clip_w, clip_h),
            true,
        ),
        Clip::new(&img, &IRect::from_xywh(0, clip_h, clip_w * 2, clip_h), true),
        Clip::new(
            &img,
            &IRect::from_xywh(clip_w, clip_h, clip_w * 2, clip_h),
            true,
        ),
    ];
    clips.insert("running".to_string(), running_clips);

    SpriteSheet { clips }
}
