// TODO 2 pixel széles border

extern crate sdl2;
extern crate sdl2_ttf;



use sdl2::pixels::RGB;

use imgui;
use imgui::SizeInCharacters;

pub struct HeaderBuilder<'a> {
	disabled: bool,
	x: SizeInCharacters,
	y: SizeInCharacters,
	w: SizeInCharacters,
	h: SizeInCharacters,
	label: &'a str,
	layer: &'a mut imgui::Layer,
}


pub fn header<'a>(layer: &'a mut imgui::Layer, label: &'a str, w: SizeInCharacters, h: SizeInCharacters) -> HeaderBuilder<'a>{
	HeaderBuilder::new(layer, label, w, h)
}

impl<'a> HeaderBuilder<'a> {
	pub fn new(layer: &'a mut imgui::Layer, label: &'a str, w: SizeInCharacters, h: SizeInCharacters) -> HeaderBuilder<'a> {
		HeaderBuilder {
			disabled: false,
			x: layer.last_x,
			y: layer.last_y,
			w: w,
			h: h,
			label: label,
			layer: layer,
		}
	}

	pub fn disabled(mut self, v: bool) -> HeaderBuilder<'a> {self.disabled = v; self}
	pub fn x(mut self, v: SizeInCharacters) -> HeaderBuilder<'a> {self.x = v; self}
	pub fn y(mut self, v: SizeInCharacters) -> HeaderBuilder<'a> {self.y = v; self}

	pub fn right(mut self, x: SizeInCharacters) -> HeaderBuilder<'a> {
		self.x = self.layer.last_x + self.layer.last_w + x;
		self
	}

	pub fn inner_right(mut self, x: SizeInCharacters) -> HeaderBuilder<'a> {
		self.x = self.layer.last_x + x;
		self
	}

	pub fn down(mut self, y: SizeInCharacters) -> HeaderBuilder<'a> {
		self.y = self.layer.last_y + self.layer.last_h + y;
		self
	}

	pub fn inner_down(mut self, y: SizeInCharacters) -> HeaderBuilder<'a> {
		self.y = self.layer.last_y + y;
		self
	}

	pub fn draw(&mut self, renderer: &sdl2::render::Renderer) {
		draw(self, renderer);
	}
}


pub fn draw(builder: &mut HeaderBuilder, renderer: &sdl2::render::Renderer) {
	let char_w = builder.layer.char_w;
	let char_h = builder.layer.char_h;
	let x = builder.x.in_pixels(char_w);
	let y = builder.y.in_pixels(char_h);
	let w = builder.w.in_pixels(char_w);
	let header_h = char_h;
	let h = builder.h.in_pixels(char_h);

	builder.layer.last_x = builder.x;
	builder.layer.last_y = builder.y;
	builder.layer.last_w = builder.w;
	builder.layer.last_h = SizeInCharacters(1);

	let border_width = 2;
	builder.layer.draw_rect_gradient(renderer, x, y, w, header_h, RGB(40, 120, 182), RGB(22, 83, 144));
	imgui::draw_rect(renderer, x, y, w+border_width, header_h+border_width, 2, RGB(0, 0, 0));
	let text_x = imgui::center_text(builder.label, char_w, w);
	if builder.label.len() > 0 {
		builder.layer.draw_text(x + text_x, y, renderer, builder.label, RGB(236, 236, 236));
	}

	imgui::draw_rect(renderer, x, y, w+border_width, h+border_width, 2, RGB(0, 0, 0));
}