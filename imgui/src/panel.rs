extern crate sdl2;
extern crate sdl2_ttf;


use sdl2::pixels::RGB;

use base;
use base::SizeInCharacters;

pub struct PanelBuilder<'a> {
	layer: &'a mut base::Layer,
	x: SizeInCharacters,
	y: SizeInCharacters,
	w: SizeInCharacters,
	h: SizeInCharacters,
	color: sdl2::pixels::Color,
}


pub fn panel<'a>( layer: &'a mut base::Layer, w: SizeInCharacters, h: SizeInCharacters) -> PanelBuilder<'a> {
	PanelBuilder::new(layer, w, h)
}

impl<'a> PanelBuilder<'a> {
	pub fn new(layer: &'a mut base::Layer, w: SizeInCharacters, h: SizeInCharacters) -> PanelBuilder<'a> {
		PanelBuilder {
			x: layer.last_x,
			y: layer.last_y,
			w: w,
			h: h,
			layer: layer,
			color: RGB(76, 78, 78),
		}
	}

	pub fn x(mut self, v: SizeInCharacters) -> PanelBuilder<'a> {self.x = v; self}
	pub fn y(mut self, v: SizeInCharacters) -> PanelBuilder<'a> {self.y = v; self}
	pub fn color(mut self, v: sdl2::pixels::Color) -> PanelBuilder<'a> {self.color = v; self}
	pub fn right(mut self, x: SizeInCharacters) -> PanelBuilder<'a> {
		self.x = self.layer.last_x + self.layer.last_w + x;
		self
	}

	pub fn down(mut self, y: SizeInCharacters) -> PanelBuilder<'a> {
		self.y = self.layer.last_y + self.layer.last_h + y;
		self
	}


	pub fn draw(&mut self, renderer: &sdl2::render::Renderer) {
		draw(self, renderer)
	}
}

pub fn draw(builder: &mut PanelBuilder, renderer: &sdl2::render::Renderer) {
	let char_w = builder.layer.char_w;
	let char_h = builder.layer.char_h;
	let x = builder.x.in_pixels(char_w);
	let y = builder.y.in_pixels(char_h);
	let w = builder.w.in_pixels(char_w);
	let h = builder.h.in_pixels(char_h);

	builder.layer.last_x = builder.x;
	builder.layer.last_y = builder.y;
	builder.layer.last_w = builder.w;
	builder.layer.last_h = builder.h;


	builder.layer.fill_rect(x, y, w, h, builder.color);
}
