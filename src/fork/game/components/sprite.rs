use image::DynamicImage;
use image::GenericImageView;
use nphysics2d::math::Isometry;

use skulpin::skia_safe::{
    AlphaType, Canvas, ColorInfo, ColorSpace, ColorType, Data, IRect, ISize, Image, ImageInfo,
};
use std::collections::HashMap;

type DrawFunction = fn(&mut Canvas, &Isometry<f32>, &SpriteSheet, u32, u32) -> ();

pub struct Sprite {
    pub draw_fn: DrawFunction,
    pub source: SpriteSheet,
}

impl Sprite {
    pub fn new(draw_fn: DrawFunction, source: SpriteSheet) -> Self {
        Self { draw_fn, source }
    }
}

pub struct SpriteSheet {
    clips: HashMap<String, Vec<Clip>>,
}

impl SpriteSheet {
    pub fn new(clips: HashMap<String, Vec<Clip>>) -> Self {
        Self { clips }
    }
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

pub fn make_skia_image(img: &DynamicImage) -> Image {
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
