extern crate sdl2;
extern crate sdl2_ttf;

use sdl2::pixels::RGB;
use base;
use base::SizeInCharacters;


#[deriving(PartialEq, Clone, Show)]
pub enum Type {
	Vertical,
	Horizontal
}

pub struct SliderBuilder<'a> {
	x: SizeInCharacters,
	y: SizeInCharacters,
	w: SizeInCharacters,
	h: SizeInCharacters,
	min_value: SizeInCharacters,
	max_value: SizeInCharacters,
	value: &'a mut SizeInCharacters,
	layer: &'a mut base::Layer,
	typ: Type,
}


pub fn slider<'a>( layer: &'a mut base::Layer, value: &'a mut SizeInCharacters, typ: Type, w: SizeInCharacters, h: SizeInCharacters) -> SliderBuilder<'a> {
	SliderBuilder::new(layer, value, typ, w, h)
}

impl<'a> SliderBuilder<'a> {
	pub fn new(layer: &'a mut base::Layer, value: &'a mut SizeInCharacters, typ: Type, w: SizeInCharacters, h: SizeInCharacters) -> SliderBuilder<'a> {
		SliderBuilder {
			x: layer.last_x,
			y: layer.last_y,
			w: w,
			h: h,
			layer: layer,
			value: value,
			typ: typ,
			min_value: SizeInCharacters(0),
			max_value: if typ == Vertical {w} else {h},
		}
	}

	pub fn x(mut self, v: SizeInCharacters) -> SliderBuilder<'a> {self.x = v; self}
	pub fn y(mut self, v: SizeInCharacters) -> SliderBuilder<'a> {self.y = v; self}
	pub fn min_value(mut self, v: SizeInCharacters) -> SliderBuilder<'a> {self.min_value = v; self}
	pub fn max_value(mut self, v: SizeInCharacters) -> SliderBuilder<'a> {self.max_value = v; self}
	pub fn right(mut self, x: SizeInCharacters) -> SliderBuilder<'a> {
		self.x = self.layer.last_x + self.layer.last_w + x;
		self
	}

	pub fn inner_right(mut self, x: SizeInCharacters) -> SliderBuilder<'a> {
		self.x = self.layer.last_x + x;
		self
	}

	pub fn down(mut self, y: SizeInCharacters) -> SliderBuilder<'a> {
		self.y = self.layer.last_y + self.layer.last_h + y;
		self
	}

	pub fn inner_down(mut self, y: SizeInCharacters) -> SliderBuilder<'a> {
		self.y = self.layer.last_y + y;
		self
	}


	pub fn draw(&mut self) -> bool {
		draw(self)
	}
}

pub fn draw(builder: &mut SliderBuilder) -> bool {
	let char_w = builder.layer.char_w;
	let char_h = builder.layer.char_h;
	let region_x = (builder.x).in_pixels(char_w);
	let region_y = (builder.y).in_pixels(char_h);
	let slider_x: i32;
	let slider_y: i32;
	let slider_w: i32;
	let slider_h: i32;

	if builder.typ == Vertical {
		slider_x = (builder.x + *builder.value).in_pixels(char_w);
		slider_y = builder.y.in_pixels(char_h);
		slider_w = char_w;
		slider_h = builder.h.in_pixels(char_h);
	} else {
		slider_x = builder.x.in_pixels(char_w);
		slider_y = (builder.y + *builder.value).in_pixels(char_h);
		slider_w = builder.w.in_pixels(char_w);
		slider_h = char_h;
	}

	builder.layer.last_x = builder.x;
	builder.layer.last_y = builder.y;
	builder.layer.last_w = builder.w;
	builder.layer.last_h = builder.h;
	

	let hover = builder.layer.is_mouse_in(slider_x, slider_y, slider_w, slider_h);

	builder.layer.bottom_surface.fill_rect(slider_x, slider_y, slider_w, slider_h, RGB(114, 114, 114));
	if hover {
		if builder.typ == Vertical {
			let center_h = (slider_h as f32 * 0.1f32) as i32; 
			builder.layer.bottom_surface.fill_rect(slider_x, slider_y + slider_h/2 - center_h/2, slider_w, center_h, RGB(140, 140, 140));
		} else {
			let center_w = (slider_w as f32 * 0.1f32) as i32; 
			builder.layer.bottom_surface.fill_rect(slider_x + slider_w/2 - center_w/2, slider_y, center_w, slider_h, RGB(140, 140, 140));
		}
	}

	let id = builder.value as *mut SizeInCharacters as i32;
	let was_active = builder.layer.is_active_widget(id);

	if was_active {
		if builder.layer.is_mouse_down() {
			if builder.typ == Vertical {
				let x_in_region = builder.layer.mouse_x() - region_x;
				let nth_column = SizeInCharacters(x_in_region / char_w);
				if nth_column > builder.min_value && nth_column < builder.max_value {
					*builder.value = nth_column;
				}
			} else {
				let y_in_region = builder.layer.mouse_y() - region_y;
				let nth_row = SizeInCharacters(y_in_region / char_h);
				if nth_row > builder.min_value && nth_row < builder.max_value {
					*builder.value = nth_row;
				}
			}
			return true;
		} else {
			builder.layer.clear_active_widget();	
		}
	} else {
		let click = hover && builder.layer.is_mouse_down();
		if click {
			builder.layer.set_active_widget(id);
		}
	}

	false
}
