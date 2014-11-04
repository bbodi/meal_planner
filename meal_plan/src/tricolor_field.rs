extern crate sdl2;
extern crate sdl2_ttf;

use sdl2::pixels::RGB;
use sdl2::rect::Rect;

use imgui::textfield;

pub struct TriColorFieldBuilder<'a> {
	textfield: textfield::TextFieldBuilder<'a>,
	values: (f32, f32, f32, f32), // protein, ch, fat, sum weight
}

pub fn tricolor_field_str<'a>(tf: textfield::TextFieldBuilder<'a>, values: (f32, f32, f32, f32))-> TriColorFieldBuilder<'a> {
	TriColorFieldBuilder::new(tf.value_color(RGB(0, 0, 0)).bold(true), values)
}

pub fn fill_tri_rect(renderer: &sdl2::render::Renderer, x: i32, y: i32, w: i32, h: i32, values: (f32, f32, f32, f32), hover: bool) {
	let (p, ch, f, weight) = values;
	let w = w as f32;
	let values = [p / weight, ch / weight, f / weight];
	if values[0].is_nan() {
		return;
	}
	let w1 = w.min(w * values[0]);
	let w2 = (w-w1).min(w * values[1]);
	let w3 = (w-w1-w2).min(w * values[2]);
	let w4 = w - (w1 + w2 + w3);
	if w1 < 0f32 || w2 < 0f32 || w3 < 0f32 {
		return;
	}
	let add = if hover {10} else {0};
	let _ = renderer.set_draw_color(RGB(76+add, 166+add, 79+add));
	let _ = renderer.fill_rect(&Rect::new(x, y, w1 as i32, h));
	let _ = renderer.set_draw_color(RGB(237+add, 166+add, 0+add));
	let _ = renderer.fill_rect(&Rect::new(x+w1 as i32, y, w2 as i32, h));
	let _ = renderer.set_draw_color(RGB(210+add, 93+add, 90+add));
	let _ = renderer.fill_rect(&Rect::new(x+(w1+w2) as i32, y, w3 as i32, h));

	let _ = renderer.set_draw_color(RGB(210+add, 210+add, 210+add));
	let _ = renderer.fill_rect(&Rect::new(x+(w1+w2+w3) as i32, y, w4 as i32, h));
}

impl<'a> TriColorFieldBuilder<'a> {
	pub fn new(tf: textfield::TextFieldBuilder<'a>, values: (f32, f32, f32, f32))-> TriColorFieldBuilder<'a> {
		TriColorFieldBuilder {
			textfield: tf,
			values: values,
		}
	}

	pub fn draw(&mut self, renderer: &sdl2::render::Renderer) -> Option<textfield::TextFieldResult> {
		draw(self, renderer)
	}
}

pub fn draw(builder: &mut TriColorFieldBuilder, renderer: &sdl2::render::Renderer) -> Option<textfield::TextFieldResult> {
	let char_w = builder.textfield.layer.char_w;
	let char_h = builder.textfield.layer.char_h;
	let x = builder.textfield.x.in_pixels(char_w);
	let y = builder.textfield.y.in_pixels(char_h);
	let w = builder.textfield.w.in_pixels(char_w);
	let h = char_h;
	let label_width = builder.textfield.label.len() as i32  * char_w;
	let id = builder.textfield.value.get_id();
	let was_active = builder.textfield.layer.is_active_widget(id);
	let hover = builder.textfield.layer.is_mouse_in(x, y, label_width+w, h);
	let clicked_out = builder.textfield.layer.is_mouse_down() && !hover && was_active;
	let active = was_active && !clicked_out;

	fill_tri_rect(renderer, label_width+x, y, w, h, builder.values, hover || active);

	textfield::draw_text(&mut builder.textfield, renderer);
	textfield::draw_border(&builder.textfield, renderer);
	textfield::handle_logic(&mut builder.textfield)
}
