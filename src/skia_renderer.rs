use pango::*;
use skia_safe::{
    Surface, 
    Color as SkColor,
    Paint as SkPaint, 
    TextBlob, 
    Font as SkFont, 
    Typeface, 
    EncodedImageFormat, 
    Data,
};
use glib::{ 
    IsA, 
    GString,    
};

pub struct Canvas {
    surface: Surface,
}

impl Canvas {
    pub fn new(width: i32, height: i32) -> Canvas {
        let mut surface = Surface::new_raster_n32_premul((width, height)).expect("no surface!");
        surface.canvas().clear(SkColor::WHITE);
        Canvas { surface }
    }

    pub fn canvas(&mut self) -> &mut skia_safe::Canvas {
        self.surface.canvas()
    }

    pub fn data(&mut self) -> Data {
        let image = self.surface.image_snapshot();
        image.encode_to_data(EncodedImageFormat::PNG).unwrap()
    }
}

pub struct SkiaRenderer {
    pub canvas: Box<Canvas>,
}


impl SkiaRenderer {

}

impl RendererExt for SkiaRenderer {
    fn activate(&self) {}

    fn deactivate(&self) {}

    fn draw_error_underline(&self, x: i32, y: i32, width: i32, height: i32) {}

    fn draw_glyph<P: IsA<Font>>(&self, font: &P, glyph: Glyph, x: f64, y: f64) {}

    fn draw_glyph_item(&self, text: Option<&str>, glyph_item: &mut GlyphItem, x: i32, y: i32) {}

    fn draw_glyphs<P: IsA<Font>>(&self, font: &P, glyphs: &mut GlyphString, x: i32, y: i32) {}

    fn draw_layout(&self, layout: &Layout, x: i32, y: i32) {
        let paint2 = SkPaint::default();
        let t = layout.get_text().unwrap();

        let text = TextBlob::from_str(
            t.as_str(),
            &SkFont::from_typeface(&Typeface::default(), 18.0),
        ).unwrap();
        let mut surf = self.canvas.surface.clone();
        let can = surf.canvas();
        can.draw_text_blob(&text, (50, 25), &paint2);
    }

    fn draw_layout_line(&self, line: &LayoutLine, x: i32, y: i32) {}

    fn draw_rectangle(&self, part: RenderPart, x: i32, y: i32, width: i32, height: i32) {}

    fn draw_trapezoid(
        &self,
        part: RenderPart,
        y1_: f64,
        x11: f64,
        x21: f64,
        y2: f64,
        x12: f64,
        x22: f64,
    ) {}

    fn get_color(&self, part: RenderPart) -> Option<Color> { None }

    fn get_layout(&self) -> Option<Layout> { None }

    fn get_layout_line(&self) -> Option<LayoutLine> { None }

    fn get_matrix(&self) -> Option<Matrix> { None }

    fn part_changed(&self, part: RenderPart) { }

    fn set_color(&self, part: RenderPart, color: Option<&Color>) { }

    fn set_matrix(&self, matrix: Option<&Matrix>) { }
}
