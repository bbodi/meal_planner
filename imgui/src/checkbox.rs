extern crate sdl2;
extern crate sdl2_ttf;

use sdl2::pixels::RGB;
use sdl2::rect::Rect;
use base;
use base::SizeInCharacters;

pub struct CheckboxBuilder<'a> {
	disabled: bool,
	x: SizeInCharacters,
	y: SizeInCharacters,
	label: &'a str,
	layer: &'a mut base::Layer,
	value: &'a mut bool,
}

pub fn checkbox<'a>(layer: &'a mut base::Layer, value: &'a mut bool) -> CheckboxBuilder<'a> {
	CheckboxBuilder::new(layer, value)
}

impl<'a> CheckboxBuilder<'a> {
	pub fn new(layer: &'a mut base::Layer, value: &'a mut bool, ) -> CheckboxBuilder<'a> {
		CheckboxBuilder {
			disabled: false,
			x: layer.last_x,
			y: layer.last_y,
			label: "",
			layer: layer,
			value: value,
		}
	}

	pub fn disabled(mut self, v: bool) -> CheckboxBuilder<'a> {self.disabled = v; self}
	pub fn label(mut self, v: &'a str) -> CheckboxBuilder<'a> {self.label = v; self}
	pub fn x(mut self, v: SizeInCharacters) -> CheckboxBuilder<'a> {self.x = v; self}
	pub fn y(mut self, v: SizeInCharacters) -> CheckboxBuilder<'a> {self.y = v; self}
	pub fn right(mut self, x: SizeInCharacters) -> CheckboxBuilder<'a> {
		self.x = self.layer.last_x + self.layer.last_w + x;
		self
	}

	pub fn inner_right(mut self, x: SizeInCharacters) -> CheckboxBuilder<'a> {
		self.x = self.layer.last_x + x;
		self
	}

	pub fn down(mut self, y: SizeInCharacters) -> CheckboxBuilder<'a> {
		self.y = self.layer.last_y + self.layer.last_h + y;
		self
	}

	pub fn inner_down(mut self, y: SizeInCharacters) -> CheckboxBuilder<'a> {
		self.y = self.layer.last_y + y;
		self
	}

	pub fn draw(&mut self) -> bool {
		draw(self)
	}
}

pub fn draw(builder: &mut CheckboxBuilder) -> bool {
	let char_w = builder.layer.char_w;
	let char_h = builder.layer.char_h;
	let x = builder.x.in_pixels(char_w);
	let y = builder.y.in_pixels(char_h);
	let w = char_h + char_w*builder.label.len() as i32;
	let hover = builder.layer.is_mouse_in(x, y, w, char_h);
	let mousebtn_just_released = builder.layer.is_mouse_released() && hover;
	let label_len = builder.label.len() as i32;

	builder.layer.last_x = builder.x;
	builder.layer.last_y = builder.y;
	builder.layer.last_w = SizeInCharacters(label_len + 1);
	builder.layer.last_h = SizeInCharacters(1);

	if mousebtn_just_released {
		*builder.value = !*builder.value;
	}

	if hover {
		builder.layer.draw_rect_gradient(x, y, char_h, char_h, RGB(99, 103, 113), RGB(62, 65, 73));
	} else {
		builder.layer.draw_rect_gradient(x, y, char_h, char_h, RGB(82, 86, 90), RGB(49, 52, 55));
	}

	let color = if *builder.value {
		sdl2::pixels::RGB(51 , 200, 51)
	} else {
		sdl2::pixels::RGB(51 , 51, 51)
	};
	builder.layer.fill_rect(x + char_h/3, y + char_h/3, char_h/3, char_h/3, color);
	if builder.label.len() > 0 {
		builder.layer.draw_text(x + char_h, y, builder.label, RGB(151, 151, 151));
	}

	return mousebtn_just_released;
}
