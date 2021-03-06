extern crate sdl2;
extern crate sdl2_ttf;

use sdl2::pixels::RGB;
use base;
use base::SizeInCharacters;

pub struct ButtonBuilder<'a> {
	disabled: bool,
	x: SizeInCharacters,
	y: SizeInCharacters,
	label: &'a str,
	allow_multi_click: bool,
	layer: &'a mut base::Layer
}

pub fn button<'a>(layer: &'a mut base::Layer, label: &'a str) -> ButtonBuilder<'a> {
	ButtonBuilder::new(layer, label)
}

impl<'a> ButtonBuilder<'a> {
	pub fn new(layer: &'a mut base::Layer, label: &'a str) -> ButtonBuilder<'a> {
		ButtonBuilder {
			disabled: false,
			x: layer.last_x,
			y: layer.last_y,
			label: label,
			layer: layer,
			allow_multi_click: false
		}
	}

	pub fn disabled(mut self, v: bool) -> ButtonBuilder<'a> {self.disabled = v; self}
	pub fn allow_multi_click(mut self, v: bool) -> ButtonBuilder<'a> {self.allow_multi_click = v; self}
	pub fn x(mut self, v: SizeInCharacters) -> ButtonBuilder<'a> {self.x = v; self}
	pub fn y(mut self, v: SizeInCharacters) -> ButtonBuilder<'a> {self.y = v; self}
	pub fn right(mut self, x: SizeInCharacters) -> ButtonBuilder<'a> {
		self.x = self.layer.last_x + self.layer.last_w + x;
		self
	}

	pub fn inner_right(mut self, x: SizeInCharacters) -> ButtonBuilder<'a> {
		self.x = self.layer.last_x + x;
		self
	}

	pub fn left(mut self, x: SizeInCharacters) -> ButtonBuilder<'a> {
		self.x = self.layer.last_x - x;
		self
	}

	pub fn down(mut self, y: SizeInCharacters) -> ButtonBuilder<'a> {
		self.y = self.layer.last_y + self.layer.last_h + y;
		self
	}

	pub fn up(mut self, y: SizeInCharacters) -> ButtonBuilder<'a> {
		self.y = self.layer.last_y - y;
		self
	}

	pub fn inner_down(mut self, y: SizeInCharacters) -> ButtonBuilder<'a> {
		self.y = self.layer.last_y + y;
		self
	}

	pub fn draw(&mut self) -> bool {
		draw(self)
	}
}

pub fn draw(builder: &mut ButtonBuilder) -> bool {
	builder.layer.last_x = builder.x;
	builder.layer.last_y = builder.y;
	builder.layer.last_w = SizeInCharacters(base::text_len(builder.label) as i32);
	builder.layer.last_h = SizeInCharacters(1);

	let char_w = builder.layer.char_w;
	let char_h = builder.layer.char_h;
	let x = builder.x.in_pixels(char_w);
	let y = builder.y.in_pixels(char_h);
	let border_width = 2i32;
	let text_border_dist = 3;
	let w = char_w*base::text_len(builder.label) as i32 + text_border_dist*2;
	let h = char_h;
	if builder.disabled {
		builder.layer.bottom_surface.fill_rect(x, y, w, h, RGB(50, 50, 50));
		builder.layer.bottom_surface.draw_text(border_width+text_border_dist+x, y+border_width, builder.label, RGB(151, 151, 151));
		return false;
	}

	let was_hot = builder.layer.is_hot_widget(x << 16 | (y & 0xFFFF) );
	let was_active = builder.layer.is_active_widget(x << 16 | (y & 0xFFFF) );
	let hover = builder.layer.is_mouse_in(x, y, w, h);
	let released = builder.layer.is_mouse_released();

	let button_down = was_active && !released;
	let mouse_down = builder.layer.is_mouse_down();

	if mouse_down && hover && !was_active {
		builder.layer.set_active_widget_temporarily(x << 16 | (y & 0xFFFF) );
	} else if was_active && released {
		builder.layer.clear_active_widget();
	}

	if hover && !was_hot {
		builder.layer.set_hot_widget(x << 16 | (y & 0xFFFF) );
	} else if was_hot && !hover {
		builder.layer.clear_hot_widget();
	}

	if button_down {
		builder.layer.bottom_surface.draw_rect_gradient(x, y, w, h, RGB(48, 48, 48), RGB(83, 83, 83));
	} else if hover {
		builder.layer.bottom_surface.draw_rect_gradient(x, y, w, h, RGB(114, 114, 114), RGB(68, 68, 68));
	} else {
		builder.layer.bottom_surface.draw_rect_gradient(x, y, w, h, RGB(82, 85, 90), RGB(47, 50, 53));
	}
	builder.layer.bottom_surface.draw_rect(x, y, w+border_width, h+border_width, 2, RGB(0, 0, 0));
	if builder.label.len() > 0 {
		builder.layer.bottom_surface.draw_text(border_width+text_border_dist+x, y+border_width, builder.label, RGB(151, 151, 151));
	}
	return (released && hover) || (button_down && hover && builder.allow_multi_click);
}
