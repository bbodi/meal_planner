extern crate sdl2;
extern crate sdl2_ttf;

use sdl2::pixels::RGB;
use sdl2::rect::Rect;

use imgui::textfield;

pub struct TriColorFieldBuilder<'a> {
	textfield: textfield::TextFieldBuilder<'a>,
	values: &'a [f32],
}



pub fn tricolor_field_str<'a>(tf: textfield::TextFieldBuilder<'a>, values: &'a [f32])-> TriColorFieldBuilder<'a> {
	TriColorFieldBuilder::new(tf.value_color(RGB(0, 0, 0)).bold(true), values)
}

pub fn fill_tri_rect(renderer: &sdl2::render::Renderer, x: i32, y: i32, w: i32, h: i32, values: &[f32], hover: bool) {
	let w1 = (w as f32 * values[0]) as i32;
	let w2 = (w as f32 * values[1]) as i32;
	let w3 = (w as f32 * values[2]) as i32;
	if w1 < 0 || w2 < 0 || w3 < 0 {
		return;
	}
	let add = if hover {10} else {0};
	let _ = renderer.set_draw_color(RGB(76+add, 166+add, 79+add));
	let _ = renderer.fill_rect(&Rect::new(x, y, w1, h));
	let _ = renderer.set_draw_color(RGB(237+add, 166+add, 0+add));
	let _ = renderer.fill_rect(&Rect::new(x+w1, y, w2, h));
	let _ = renderer.set_draw_color(RGB(210+add, 93+add, 90+add));
	let _ = renderer.fill_rect(&Rect::new(x+w1+w2, y, w3, h));
}

impl<'a> TriColorFieldBuilder<'a> {
	pub fn new(tf: textfield::TextFieldBuilder<'a>, values: &'a [f32])-> TriColorFieldBuilder<'a> {
		TriColorFieldBuilder {
			textfield: tf,
			values: values,
		}
	}
	
	pub fn draw(&mut self, renderer: &sdl2::render::Renderer) -> bool {
		draw(self, renderer)
	}
}

pub fn draw(builder: &mut TriColorFieldBuilder, renderer: &sdl2::render::Renderer) -> bool {
	let char_w = builder.textfield.layer.char_w;
	let char_h = builder.textfield.layer.char_h;
	let x = builder.textfield.x.in_pixels(char_w);
	let y = builder.textfield.y.in_pixels(char_h);
	let w = builder.textfield.w.in_pixels(char_w);
	let h = char_h;
	let label_width = builder.textfield.label.len() as i32  * char_w;	
	let was_active = builder.textfield.layer.is_active_widget(x, y);
	let hover = builder.textfield.layer.is_mouse_in(x, y, label_width+w, h);
	let clicked_out = builder.textfield.layer.is_mouse_down() && !hover && was_active;
	let active = was_active && !clicked_out;

	fill_tri_rect(renderer, label_width+x, y, w, h, builder.values, hover || active);
	
	textfield::draw_text(&mut builder.textfield, renderer);
	textfield::draw_border(&builder.textfield, renderer);
	textfield::handle_logic(&mut builder.textfield)
}