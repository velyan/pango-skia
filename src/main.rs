mod skia_renderer;

use cairo::*;
use pango::{ Layout as PangoLayout, Underline, Attribute, Direction, EllipsizeMode};
use pangocairo::functions::{show_layout, create_context};
use std::fs::File;
use glib::translate::*;
use pango_sys;
use skia_safe::{Point as SkPoint, Paint as SkPaint, Typeface, Font as SkFont, TextBlobBuilder};
use skia_bindings::{SkScalar};
use std::io::Write;
use crate::skia_renderer::Canvas;

fn pango_pixels(d: i32) -> i32 { d + 512 >> 10 }

fn main() {    
   cairo_pango_skia();
}

fn create_layout() -> pango::Layout {
    let surface = cairo::ImageSurface::create(cairo::Format::ARgb32, 0, 0).unwrap();
    let ctx = cairo::Context::new(&surface);
    pangocairo::functions::create_layout(&ctx).unwrap()
}

fn cairo_pango_skia() {
        let text = String::from("Georgia 30");
        let layout = create_layout();
        let desc = pango::FontDescription::from_string("Georgia 30");
        layout.set_font_description(Some(&desc));
        layout.set_text(text.as_str());

        let mut iterator = layout.get_iter().unwrap();
        let y_offset: SkScalar = pango_pixels(iterator.get_baseline()) as f32;
        let x_offset: SkScalar = 0.0;
  
        let mut canvas = Canvas::new(500, 2000);
        let mut glyph_x = x_offset;

        for run in iterator.get_run_readonly() {

                let pango_item: pango::Item = unsafe { from_glib_none((*run.to_glib_none().0).item) };
                let offset = pango_item.offset();
                println!("{}", offset);

                let glyph_string: pango_sys::PangoGlyphString = unsafe { (*(*run.to_glib_none().0).glyphs) };
                println!("{:?}", glyph_string);
                let glyph_count: usize = unsafe { glyph_string.num_glyphs } as usize;
                println!("{:?}", glyph_count);

                let glyph_infos: &[pango_sys::PangoGlyphInfo] = unsafe { std::slice::from_raw_parts(glyph_string.glyphs, glyph_count) };
                println!("{:?}", glyph_infos);

                let mut glyphs = Vec::<u32>::with_capacity(glyph_count);
                let mut positions = Vec::<SkPoint>::with_capacity(glyph_count);

                for i in 0..glyph_count {
                        let glyph_info = glyph_infos[i];
                        let pos =  SkPoint::new(glyph_x, y_offset); // todo: + offset 
                        glyph_x += pango_pixels(glyph_info.geometry.width) as f32;

                        glyphs.push(glyph_info.glyph);
                        positions.push(pos);
                }

                println!("{}", glyph_x);
                println!("glyphs {:?}", glyphs);
                

                //Skia
                let mut paint = SkPaint::default();
                paint.set_anti_alias(true);
                let typeface = Typeface::new("Georgia", skia_safe::FontStyle::default()).unwrap();
                
                let font = SkFont::from_typeface(typeface, 30.0);
                let mut builder = TextBlobBuilder::new();
                let (skia_glyphs, skia_pos) = builder.alloc_run_pos_h(font, glyph_count, y_offset, None);
                println!("skia_glyphs {:?}", skia_glyphs);
                println!("skia_pos {:?}", skia_pos);

                for (i, glyph) in glyphs.iter().enumerate() {
                        skia_glyphs[i] = *glyph as u16
                }
                println!("skia_glyphs updated {:?}", skia_glyphs);


                for (i, pos) in positions.iter().enumerate() {
                        skia_pos[i] = pos.x
                }
                println!("skia_pos updated {:?}", skia_pos);

                let blob = builder.make().unwrap();
                let skia_canvas = canvas.canvas();
                
                skia_canvas.draw_text_blob(&blob, SkPoint::new(x_offset, y_offset), &paint);
        }

        let d = canvas.data();

        let mut file = File::create("test.png").unwrap();
        let bytes = d.as_bytes();
        file.write_all(bytes).unwrap();    
}

fn attributes_support() {
        // let path = Path::new("./text.svg");
        let surface = ImageSurface::create(Format::ARgb32, 500, 2000).unwrap();//SvgSurface::new(500.0, 2000.0, &path);
        let ctx = Context::new(&surface);

        // background
        ctx.set_source_rgb(1.0, 1.0, 1.0);
        ctx.rectangle(0.0, 0.0, 500.0, 2000.0);
        ctx.fill();

        ctx.set_source_rgb(0.0, 0.0, 0.0);
        ctx.translate(20.0, 30.0);

        let mut layout = pangocairo::functions::create_layout(&ctx).unwrap();

        // Normal
        let mut desc = pango::FontDescription::from_string("Georgia 30");
        layout.set_font_description(Some(&desc));
        layout.set_text("Georgia 30");
        
        show_layout(&ctx, &layout);

        // Bold
        ctx.translate(0.0, 70.0);
        desc = pango::FontDescription::from_string("OpenSans-Regular Bold 20");
        layout.set_font_description(Some(&desc));
        layout.set_text("OpenSans-Regular Bold 20");
        show_layout(&ctx, &layout);

        // Italic
        ctx.translate(0.0, 70.0);
        desc = pango::FontDescription::from_string("Kaya-Medium Italic 15");
        layout.set_font_description(Some(&desc));
        layout.set_text("Kaya-Medium Italic 15");
        show_layout(&ctx, &layout);

        // Underline
        ctx.translate(0.0, 70.0);
        let attr_list = pango::AttrList::new();
        let underline = Attribute::new_underline(Underline::Single).unwrap();
        attr_list.insert(underline);
        layout.set_attributes(Some(&attr_list));
        layout.set_text("Underlined italic size 15");
        show_layout(&ctx, &layout);

        // RTL
        ctx.translate(0.0, 70.0);
        let pango_ctx = create_context(&ctx).unwrap();
        pango_ctx.set_base_dir(Direction::Rtl);
        layout = PangoLayout::new(&pango_ctx);
        layout.set_auto_dir(false);
        layout.set_text("bahrain مصر kuwait");
        show_layout(&ctx, &layout);

        // LTR
        ctx.translate(0.0, 70.0);
        pango_ctx.set_base_dir(Direction::Ltr);
        show_layout(&ctx, &layout);

        // Edge case 
        ctx.translate(0.0, 70.0);
        layout.set_text("Lorem ipsum dolor sit amet, consectetur adipiscing elit.");
        println!("char count {}",layout.get_character_count());
        let mut lr = layout.get_extents().1;
        let mut width = lr.width/pango::SCALE;
        println!("logical rect {:?}",lr);
        println!("width {}", width);

        let attr_list = pango::AttrList::new();
        let tracking = Attribute::new_letter_spacing(-1 * pango::SCALE).unwrap();
        attr_list.insert(tracking);
        layout.set_attributes(Some(&attr_list));

        lr = layout.get_extents().1;
        width = lr.width/pango::SCALE;
        println!("logical rect {:?}",lr);
        println!("width {}", width);

        show_layout(&ctx, &layout);

        // Wrap
        ctx.translate(0.0, 70.0);
        let pango_ctx = create_context(&ctx).unwrap();
        layout = PangoLayout::new(&pango_ctx);
        layout.set_width(300 * pango::SCALE);
        layout.set_height(-1 * pango::SCALE);
        layout.set_ellipsize(EllipsizeMode::End);
        layout.set_text("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Curabitur volutpat, urna nec luctus varius, libero tortor rutrum odio, laoreet condimentum turpis nisi non nisi.

        Aenean malesuada ultrices velit nec lacinia. Aliquam ultricies nequ.");
        show_layout(&ctx, &layout);

        // Indent
        ctx.translate(0.0, 140.0);
        layout.set_indent(20 * pango::SCALE);
        show_layout(&ctx, &layout);

        // Justify
        ctx.translate(0.0, 140.0);
        layout.set_indent(0);
        layout.set_justify(true);
        show_layout(&ctx, &layout);

        // Alignment
        ctx.translate(0.0, 140.0);
        layout.set_justify(false);
        layout.set_alignment(pango::Alignment::Right);
        show_layout(&ctx, &layout);

        // Leading
        ctx.translate(0.0, 140.0);
        layout.set_alignment(pango::Alignment::Left);
        layout.set_spacing(20 * pango::SCALE);
        show_layout(&ctx, &layout);


        // Tracking
        ctx.translate(0.0, 250.0);
        layout.set_spacing(0);
        let attr_list = pango::AttrList::new();
        let tracking = Attribute::new_letter_spacing(10 * pango::SCALE).unwrap();
        attr_list.insert(tracking);
        layout.set_attributes(Some(&attr_list));
        show_layout(&ctx, &layout);

        //write to disk
        //svg
        // surface.finish();

        let mut buffer = File::create("foo.png").unwrap();
        let _ = surface.write_to_png(&mut buffer);
    
}

