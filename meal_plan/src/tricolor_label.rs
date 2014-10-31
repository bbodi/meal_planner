extern crate sdl2;
extern crate sdl2_ttf;

use sdl2::pixels::RGB;
use imgui::base;
use imgui::label;
use imgui::base::SizeInCharacters;

use tricolor_field;

pub struct TriColorLabelBuilder<'a> {
    label: label::LabelBuilder<'a>,
    values: &'a [f32],
}

pub fn tricolor_label<'a>(label: label::LabelBuilder<'a>, values: &'a [f32]) -> TriColorLabelBuilder<'a> {
    TriColorLabelBuilder::new(label, values)
}


impl<'a> TriColorLabelBuilder<'a> {
    pub fn new(label: label::LabelBuilder<'a>, values: &'a [f32])-> TriColorLabelBuilder<'a> {
        TriColorLabelBuilder {
            values: values,
            label: label,
        }
    }

    pub fn draw(&mut self, renderer: &sdl2::render::Renderer)  {
        draw(self, renderer);
    }
}

pub fn draw(builder: &mut TriColorLabelBuilder, renderer: &sdl2::render::Renderer) {
    let char_w = builder.label.layer.char_w;
    let char_h = builder.label.layer.char_h;
    let x = builder.label.x.in_pixels(char_w);
    let y = builder.label.y.in_pixels(char_h);

    let label_width = builder.label.label.len() as i32 * char_w;
    tricolor_field::fill_tri_rect(renderer, x, y, label_width, char_h, builder.values, false);
    builder.label.draw(renderer);
}
