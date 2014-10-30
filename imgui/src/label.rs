extern crate sdl2;
extern crate sdl2_ttf;

use sdl2::pixels::RGB;

use imgui;
use imgui::SizeInCharacters;

pub struct LabelBuilder<'a> {
	x: SizeInCharacters,
	y: SizeInCharacters, 
	label: &'a str,
	layer: &'a mut imgui::Layer,
}

pub fn label<'a>(layer: &'a mut imgui::Layer, label: &'a str) -> LabelBuilder<'a> {
	LabelBuilder::new(layer, label)
}


impl<'a> LabelBuilder<'a> {
	pub fn new(layer: &'a mut imgui::Layer, label: &'a str)-> LabelBuilder<'a> {
		LabelBuilder {
			x: layer.last_x,
			y: layer.last_y,
			layer: layer,
			label: label,
		}
	}

	pub fn label(mut self, v: &'a str) -> LabelBuilder<'a> {self.label = v; self}
	pub fn x(mut self, v: SizeInCharacters) -> LabelBuilder<'a> {self.x = v; self}
	pub fn y(mut self, v: SizeInCharacters) -> LabelBuilder<'a> {self.y = v; self}
	pub fn right(mut self, x: SizeInCharacters) -> LabelBuilder<'a> {
		self.x = self.layer.last_x + self.layer.last_w + x;
		self
	}

	pub fn inner_right(mut self, x: SizeInCharacters) -> LabelBuilder<'a> {
		self.x = self.layer.last_x + x;
		self
	}

	pub fn down(mut self, y: SizeInCharacters) -> LabelBuilder<'a> {
		self.y = self.layer.last_y + self.layer.last_h + y;
		self
	}

	pub fn inner_down(mut self, y: SizeInCharacters) -> LabelBuilder<'a> {
		self.y = self.layer.last_y + y;
		self
	}

	pub fn draw(&mut self, renderer: &sdl2::render::Renderer)  {
		draw(self, renderer);
	}
}

pub fn draw(builder: &mut LabelBuilder, renderer: &sdl2::render::Renderer) {
	let char_w = builder.layer.char_w;
	let char_h = builder.layer.char_h;
	let x = builder.x.in_pixels(char_w);
	let y = builder.y.in_pixels(char_h);

	builder.layer.last_x = builder.x;
	builder.layer.last_y = builder.y;
	builder.layer.last_w = SizeInCharacters(builder.label.len() as i32);
	builder.layer.last_h = SizeInCharacters(1);
	


	let _ = renderer.set_draw_color(sdl2::pixels::RGB(32 , 32, 32));


	if builder.label.len() > 0 {
		imgui::draw_text(x, y, renderer, &builder.layer.font, builder.label, RGB(221, 221, 221));
	}
}