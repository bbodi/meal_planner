extern crate sdl2;
extern crate sdl2_ttf;

use sdl2::pixels::RGB;

use imgui;
use imgui::SizeInCharacters;

pub struct ScrollBarBuilder<'a> {
	disabled: bool,
	x: SizeInCharacters,
	y: SizeInCharacters, 
	w: SizeInCharacters,
	min_value: f32,
	max_value: f32,
	value: &'a mut f32,
	layer: &'a mut imgui::Layer,
	color: sdl2::pixels::Color,
}


pub fn scrollbar<'a>( layer: &'a mut imgui::Layer, w: SizeInCharacters, min_value: f32, max_value: f32, value: &'a mut f32) -> ScrollBarBuilder<'a> {
	ScrollBarBuilder::new(layer, w, min_value, max_value, value)
}

impl<'a> ScrollBarBuilder<'a> {
	pub fn new(layer: &'a mut imgui::Layer, w: SizeInCharacters, min_value: f32, max_value: f32, value: &'a mut f32) -> ScrollBarBuilder<'a> {
		ScrollBarBuilder {
			disabled: false,
			x: layer.last_x,
			y: layer.last_y,
			w: w,
			layer: layer,
			value: value,
			min_value: min_value,
			max_value: max_value,
			color: RGB(76, 166, 79),
		}
	}

	pub fn disabled(mut self, v: bool) -> ScrollBarBuilder<'a> {self.disabled = v; self}
	pub fn x(mut self, v: SizeInCharacters) -> ScrollBarBuilder<'a> {self.x = v; self}
	pub fn y(mut self, v: SizeInCharacters) -> ScrollBarBuilder<'a> {self.y = v; self}
	pub fn color(mut self, v: sdl2::pixels::Color) -> ScrollBarBuilder<'a> {self.color = v; self}
	pub fn right(mut self, x: SizeInCharacters) -> ScrollBarBuilder<'a> {
		self.x = self.layer.last_x + self.layer.last_w + x;
		self
	}

	pub fn inner_right(mut self, x: SizeInCharacters) -> ScrollBarBuilder<'a> {
		self.x = self.layer.last_x + x;
		self
	}

	pub fn down(mut self, y: SizeInCharacters) -> ScrollBarBuilder<'a> {
		self.y = self.layer.last_y + self.layer.last_h + y;
		self
	}

	pub fn inner_down(mut self, y: SizeInCharacters) -> ScrollBarBuilder<'a> {
		self.y = self.layer.last_y + y;
		self
	}
	

	pub fn draw(&mut self, renderer: &sdl2::render::Renderer) -> bool {
		draw(self, renderer)
	}
}

pub fn draw(builder: &mut ScrollBarBuilder, renderer: &sdl2::render::Renderer) -> bool {
	let char_w = builder.layer.char_w;
	let char_h = builder.layer.char_h;
	let x = builder.x.in_pixels(char_w);
	let y = builder.y.in_pixels(char_h);
	let w = builder.w.in_pixels(char_w);
	let SizeInCharacters(wc) = builder.w;
	let h = 2*char_h;
	let min_label = format!("{:.1f}", builder.min_value);
	let max_label = format!("{:.1f}", builder.max_value);
	let min_label_w = min_label.len() as i32 * char_w;
	let max_label_w = max_label.len() as i32 * char_w;
	let all_w = min_label_w+char_w + w + char_w + max_label_w;

	builder.layer.last_x = builder.x;
	builder.layer.last_y = builder.y;
	builder.layer.last_w = SizeInCharacters(all_w / char_w);
	builder.layer.last_h = SizeInCharacters(2);

	
	let value_range = builder.max_value - builder.min_value;
	let step = value_range / wc as f32;
	let place_count = (builder.max_value - builder.min_value) / step ;
	let value = builder.min_value.max(*builder.value).min(builder.max_value);


	let value_place = ((value - builder.min_value) / value_range * place_count as f32) as i32;

	imgui::draw_text(x, y, renderer, &builder.layer.font, min_label.as_slice(), RGB(151, 151, 151));
	let range_start_x = min_label_w+char_w + x;
	imgui::fill_rect(renderer, range_start_x, y, w, char_h, builder.color);

	let pointer_x = min_label_w+char_w + x + char_w*value_place;

	let hover = builder.layer.is_mouse_in(x, y, all_w, h);
	if hover {
		imgui::draw_rect_gradient(renderer, pointer_x, y-char_h/2, char_w, char_h*2, RGB(114, 114, 114), RGB(68, 68, 68));
		let str = format!("{:.1f}", *builder.value);
		imgui::fill_rect(renderer, pointer_x, y-char_h, str.len() as i32 * char_w, char_h, RGB(51, 51, 51));
		imgui::draw_text(pointer_x, y-char_h, renderer, &builder.layer.font, str.as_slice(), RGB(221, 221, 221));
	} else {
		imgui::draw_rect_gradient(renderer, pointer_x, y-char_h/2, char_w, char_h*2, RGB(82, 85, 90), RGB(47, 50, 53));
	}
	imgui::draw_text(min_label_w+char_w + x + w + char_w, y, renderer, &builder.layer.font, max_label.as_slice(), RGB(151, 151, 151));

	let click = builder.layer.is_mouse_down() && hover;

	if click {
		let x_in_range = builder.layer.mouse_x() - range_start_x;
		let nth_column = x_in_range / char_w;
		*builder.value = builder.min_value + nth_column as f32 * step;
		return true;
	}


	false
}