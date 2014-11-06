extern crate sdl2;
extern crate sdl2_ttf;

use sdl2::pixels::RGB;
use base;
use base::SizeInCharacters;

pub struct HeaderBuilder<'a> {
	disabled: bool,
	x: SizeInCharacters,
	y: SizeInCharacters,
	w: SizeInCharacters,
	h: SizeInCharacters,
	bold: bool,
	label: &'a str,
	layer: &'a mut base::Layer,
	color: Option<sdl2::pixels::Color>,
}


pub fn header<'a>(layer: &'a mut base::Layer, label: &'a str, w: SizeInCharacters, h: SizeInCharacters) -> HeaderBuilder<'a>{
	HeaderBuilder::new(layer, label, w, h)
}

impl<'a> HeaderBuilder<'a> {
	pub fn new(layer: &'a mut base::Layer, label: &'a str, w: SizeInCharacters, h: SizeInCharacters) -> HeaderBuilder<'a> {
		HeaderBuilder {
			disabled: false,
			x: layer.last_x,
			y: layer.last_y,
			w: w,
			h: h,
			bold: false,
			label: label,
			layer: layer,
			color: None,
		}
	}

	pub fn disabled(mut self, v: bool) -> HeaderBuilder<'a> {self.disabled = v; self}
	pub fn x(mut self, v: SizeInCharacters) -> HeaderBuilder<'a> {self.x = v; self}
	pub fn y(mut self, v: SizeInCharacters) -> HeaderBuilder<'a> {self.y = v; self}
	pub fn bold(mut self, v: bool) -> HeaderBuilder<'a> {self.bold = v; self}
	pub fn color(mut self, v: sdl2::pixels::Color) -> HeaderBuilder<'a> {self.color = Some(v); self}

	pub fn right(mut self, x: SizeInCharacters) -> HeaderBuilder<'a> {
		self.x = self.layer.last_x + self.layer.last_w + x;
		self
	}

	pub fn left(mut self, x: SizeInCharacters) -> HeaderBuilder<'a> {
		self.x = self.layer.last_x - x;
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

	pub fn up(mut self, y: SizeInCharacters) -> HeaderBuilder<'a> {
		self.y = self.layer.last_y - y;
		self
	}

	pub fn inner_down(mut self, y: SizeInCharacters) -> HeaderBuilder<'a> {
		self.y = self.layer.last_y + y;
		self
	}

	pub fn draw_with_body(&mut self, body: |&mut base::Layer|) {
		self.layer.last_x = self.x;
		self.layer.last_y = self.y;
		self.layer.last_w = SizeInCharacters(0);
		self.layer.last_h = SizeInCharacters(1);
		draw(self);
		body(self.layer);
		self.layer.last_x = self.x;
		self.layer.last_y = self.y;
		self.layer.last_w = self.w;
		self.layer.last_h = self.h;
	}

	pub fn draw(&mut self) {
		self.layer.last_x = self.x;
		self.layer.last_y = self.y;
		self.layer.last_w = self.w;
		self.layer.last_h = SizeInCharacters(1);
		draw(self);
	}
}


pub fn draw(builder: &mut HeaderBuilder) {
	let char_w = builder.layer.char_w;
	let char_h = builder.layer.char_h;
	let x = builder.x.in_pixels(char_w);
	let y = builder.y.in_pixels(char_h);
	let w = builder.w.in_pixels(char_w);
	let header_h = char_h;
	let h = builder.h.in_pixels(char_h);

	let border_width = 2;
	if builder.color.is_some() {
		builder.layer.draw_rect_gradient1(x, y, w, header_h, builder.color.unwrap());	
	} else {
		builder.layer.draw_rect_gradient(x, y, w, header_h, RGB(40, 120, 182), RGB(22, 83, 144));
	}
	builder.layer.draw_rect(x, y, w+border_width, header_h+border_width, 2, RGB(0, 0, 0));
	let text_x = base::center_text(builder.label, char_w, w);
	if builder.label.len() > 0 {
		if builder.bold {
			builder.layer.draw_bold_text(x + text_x, y, builder.label, RGB(236, 236, 236));
		} else {
			builder.layer.draw_text(x + text_x, y, builder.label, RGB(236, 236, 236));
		}
	}

	builder.layer.draw_rect(x, y, w+border_width, h+border_width, 2, RGB(0, 0, 0));
}
