extern crate sdl2;
extern crate sdl2_ttf;

use sdl2::pixels::RGB;
use sdl2::rect::Rect;
use imgui::SizeInCharacters;

use imgui;

pub struct CheckboxBuilder<'a> {
	disabled: bool,
	x: SizeInCharacters,
	y: SizeInCharacters, 
	label: &'a str,
	layer: &'a mut imgui::Layer,
	value: &'a mut bool,
}

pub fn checkbox<'a>(layer: &'a mut imgui::Layer, label: &'a str, value: &'a mut bool) -> CheckboxBuilder<'a> {
	CheckboxBuilder::new(layer, label, value)
}

impl<'a> CheckboxBuilder<'a> {
	pub fn new(layer: &'a mut imgui::Layer, label: &'a str, value: &'a mut bool, ) -> CheckboxBuilder<'a> {
		CheckboxBuilder {
			disabled: false,
			x: layer.last_x,
			y: layer.last_y,
			label: label,
			layer: layer,
			value: value,
		}
	}

	pub fn disabled(mut self, v: bool) -> CheckboxBuilder<'a> {self.disabled = v; self}
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

	pub fn draw(&mut self, renderer: &sdl2::render::Renderer) -> bool {
		draw(self, renderer)
	}
}

pub fn draw(builder: &mut CheckboxBuilder, renderer: &sdl2::render::Renderer) -> bool {
	let char_w = builder.layer.char_w;
	let char_h = builder.layer.char_h;
	let x = builder.x.in_pixels(char_w);
	let y = builder.y.in_pixels(char_h);
	let w = char_h + char_w*builder.label.len() as i32;
	let hover = builder.layer.is_mouse_in(x, y, w, char_h);
	let mousebtn_just_released = builder.layer.is_mouse_released() && hover;
	
	
	if mousebtn_just_released {
		*builder.value = !*builder.value;
	}

	if hover {
		imgui::draw_rect_gradient(renderer, x, y, char_h, char_h, RGB(99, 103, 113), RGB(62, 65, 73));
	} else {
		imgui::draw_rect_gradient(renderer, x, y, char_h, char_h, RGB(82, 86, 90), RGB(49, 52, 55));
	}

	if *builder.value {
		let _ = renderer.set_draw_color(sdl2::pixels::RGB(51 , 200, 51));		
	} else {
		let _ = renderer.set_draw_color(sdl2::pixels::RGB(51 , 51, 51));
	}
	let _ = renderer.fill_rect(&Rect::new(x + char_h/3, y + char_h/3, char_h/3, char_h/3));
	imgui::draw_text(x + char_h, y, renderer, &builder.layer.font, builder.label, RGB(151, 151, 151));

	let _ = renderer.set_draw_color(sdl2::pixels::RGB(0, 0, 0));
	let _ = renderer.draw_rect(&Rect::new(x, y, char_h, char_h));

	return mousebtn_just_released;
}