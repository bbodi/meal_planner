extern crate sdl2;
extern crate sdl2_ttf;

use sdl2::pixels::RGB;
use imgui::label;
use imgui::base::SizeInCharacters;

use tricolor_field;

pub struct TriColorLabelBuilder<'a> {
    label: label::LabelBuilder<'a>,
    values: (f32, f32, f32, f32),
    width: SizeInCharacters
}

pub fn tricolor_label<'a>(label: label::LabelBuilder<'a>, values: (f32, f32, f32, f32), width: SizeInCharacters) -> TriColorLabelBuilder<'a> {
    TriColorLabelBuilder::new(label.color(RGB(0, 0, 0)).bold(true), values, width)
}

impl<'a> TriColorLabelBuilder<'a> {
    pub fn new(label: label::LabelBuilder<'a>, values: (f32, f32, f32, f32), width: SizeInCharacters)-> TriColorLabelBuilder<'a> {
        TriColorLabelBuilder {
            values: values,
            label: label,
            width: width
        }
    }

    pub fn draw(&mut self)  {
        draw(self);
    }
}

pub fn draw(builder: &mut TriColorLabelBuilder) {
    let char_w = builder.label.layer.char_w;
    let char_h = builder.label.layer.char_h;
    let x = builder.label.x.in_pixels(char_w);
    let y = builder.label.y.in_pixels(char_h);

    tricolor_field::fill_tri_rect(builder.label.layer, x, y, builder.width.in_pixels(char_w), char_h, builder.values, false);
    builder.label.draw();
    builder.label.layer.last_w = builder.width;
}
