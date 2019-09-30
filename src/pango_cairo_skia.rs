use pango::{ 
    Underline, 
    Attribute, 
    Direction, 
    EllipsizeMode};
use skia_safe::{
    Surface, 
    Color as SkColor,
    Paint as SkPaint, 
    TextBlob, 
    TextBlobBuilder,
    Font as SkFont, 
    Typeface, 
    EncodedImageFormat, 
    Data,
    Canvas as SkCanvas,
    FontStyle,
    Point as SkPoint,
};
use skia_bindings::{SkFontStyle_Weight};

pub fn create_layout() -> pango::Layout {
    let surface = cairo::ImageSurface::create(cairo::Format::ARgb32, 0, 0).unwrap();
    let ctx = cairo::Context::new(&surface);
    pangocairo::functions::create_layout(&ctx).unwrap()
}
