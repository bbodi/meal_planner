// TODO függőleges vonalak a chartba, pl. hetek, hónapok jelölésére

extern crate sdl2;
extern crate sdl2_ttf;

use sdl2::pixels::RGB;
use sdl2::rect::Rect;
use base;
use base::SizeInCharacters;

pub struct LineChartBuilder<'a> {
	x: SizeInCharacters,
	y: SizeInCharacters,
	w: SizeInCharacters,
	h: SizeInCharacters,
	label: &'a str,
	maybe_data: Option<&'a [i32]>,
	top_color: sdl2::pixels::Color,
	bottom_color: sdl2::pixels::Color,
	surface_color: Option<sdl2::pixels::Color>,

	layer: &'a mut base::Layer
}

pub fn line_chart<'a>(layer: &'a mut base::Layer, label: &'a str, w: SizeInCharacters, h: SizeInCharacters) -> LineChartBuilder<'a> {
	LineChartBuilder::new(layer, label, w, h)
}

impl<'a> LineChartBuilder<'a> {
	pub fn new(layer: &'a mut base::Layer, label: &'a str, w: SizeInCharacters, h: SizeInCharacters) -> LineChartBuilder<'a> {
		LineChartBuilder {
			x: layer.last_x,
			y: layer.last_y,
			w: w,
			h: h,
			label: label,
			layer: layer,
			maybe_data: None,
			top_color: sdl2::pixels::RGB(200, 200, 200),
			bottom_color: sdl2::pixels::RGB(42, 42, 42),
			surface_color: None
		}
	}

	pub fn x(mut self, v: SizeInCharacters) -> LineChartBuilder<'a> {self.x = v; self}
	pub fn y(mut self, v: SizeInCharacters) -> LineChartBuilder<'a> {self.y = v; self}

	pub fn right(mut self, x: SizeInCharacters) -> LineChartBuilder<'a> {
		self.x = self.layer.last_x + self.layer.last_w + x;
		self
	}

	pub fn inner_right(mut self, x: SizeInCharacters) -> LineChartBuilder<'a> {
		self.x = self.layer.last_x + x;
		self
	}

	pub fn down(mut self, y: SizeInCharacters) -> LineChartBuilder<'a> {
		self.y = self.layer.last_y + self.layer.last_h + y;
		self
	}

	pub fn up(mut self, y: SizeInCharacters) -> LineChartBuilder<'a> {
		self.y = self.layer.last_y - y;
		self
	}

	pub fn inner_down(mut self, y: SizeInCharacters) -> LineChartBuilder<'a> {
		self.y = self.layer.last_y + y;
		self
	}
	pub fn data(mut self, data: &'a [i32]) -> LineChartBuilder { self.maybe_data = Some(data); self}

	pub fn top_color(mut self, data: sdl2::pixels::Color) -> LineChartBuilder<'a> { self.top_color = data; self}
	pub fn bottom_color(mut self, data: sdl2::pixels::Color) -> LineChartBuilder<'a> { self.bottom_color = data; self}
	pub fn surface_color(mut self, data: Option<sdl2::pixels::Color>) -> LineChartBuilder<'a> { self.surface_color = data; self}


	pub fn draw(&mut self, renderer: &sdl2::render::Renderer) {
		draw(self, renderer);
	}
}

pub fn draw(builder: &mut LineChartBuilder, renderer: &sdl2::render::Renderer) {
	let char_w = builder.layer.char_w;
	let char_h = builder.layer.char_h;
	let x = builder.x.in_pixels(char_w);
	let y = builder.y.in_pixels(char_h);
	let w = builder.w.in_pixels(char_w);
	let h = builder.h.in_pixels(char_h);

	let selected_column = if builder.layer.is_mouse_in(x, y, w, h) {
		Some(builder.layer.mouse_x() - x)
	} else {
		None
	};

	let _ = renderer.set_draw_color(sdl2::pixels::RGB(32 , 32, 32));
	let _ = renderer.fill_rect(&sdl2::rect::Rect::new(x, y, w, h));
	if builder.maybe_data.is_none() {
		return;
	}
	if w <= 0 || h <= 0 {
		return;
	}
	let data = builder.maybe_data.unwrap();
	let horizontal_pixels_between_values = w / ::std::cmp::min(w, data.len() as i32 ) ;
	let scaled_data = create_scaled_chart_data(data.as_slice(), horizontal_pixels_between_values);
	draw_horizontal_lines(builder, renderer, scaled_data.as_slice());

	//
	if builder.surface_color.is_some() {
		let mut points = vec![];
		for (i, v) in scaled_data.iter().enumerate() {
			let p = sdl2::rect::Point::new(x + i as i32, y + (h - *v ) );
			points.push(p);
		}
		let _ = renderer.set_draw_color(builder.surface_color.unwrap());
		let _ = renderer.draw_points(points.as_slice());
	}
	//
	// a négyzetek a vonalak végén
	if horizontal_pixels_between_values > 2 {
		let mut rects = vec![];
		for (i, v) in data.iter().enumerate() {
			let px = i * horizontal_pixels_between_values as uint - 1;
			let py = (h - *v - 1) ;
			let p = sdl2::rect::Rect::new(px as i32 + x , y + py, 2, 2);
			rects.push(p);
		}
		let _ = renderer.set_draw_color(sdl2::pixels::RGB(255, 150, 150));
		let _ = renderer.fill_rects(rects.as_slice());
	}
	//
	if selected_column.is_some() {
		let index = selected_column.unwrap();
		if index < scaled_data.len() as i32 {
			let real_index = selected_column.unwrap() / horizontal_pixels_between_values;
			let index = real_index * horizontal_pixels_between_values;
			draw_vertical_line(builder, renderer, index, scaled_data[index as uint], RGB(112, 42, 42), sdl2::pixels::RGB(255, 200, 200));
			let x = x + index - 13;
			let y = y + (h - scaled_data[index as uint] - 30) ;
			let w = 26i32;
			let h = 15i32;
			let v = scaled_data[index as uint];
			builder.layer.bottom_surface.draw_rect_gradient(x, y, w, h, RGB(174, 67, 75), RGB(166, 38, 51));
			builder.layer.bottom_surface.draw_text(x, y, format!("{}", v).as_slice(), RGB(255, 255, 255));
		}
	}
}

fn create_scaled_chart_data(data: &[i32], horizontal_pixels_between_values: i32) -> Vec<i32> {
	let mut scaled_data = vec![];
	let mut last_value = data[0];
	scaled_data.push(data[0]);
	for value in data.slice(1, data.len()).iter() {
		for i in ::std::iter::range_inclusive(1, horizontal_pixels_between_values) {
			let diff = *value - last_value;
			let k = (diff as f32 * (i as f32/horizontal_pixels_between_values as f32)) ;
			scaled_data.push(last_value + k as i32);
		}
		last_value = *value;
	}
	return scaled_data;
}

fn draw_horizontal_lines(builder: &LineChartBuilder, dst: &sdl2::render::Renderer, data: &[i32]) {
	// TODO: find the longest horizontal continous line and draw it!
	let char_w = builder.layer.char_w;
	let char_h = builder.layer.char_h;
	let x = builder.x.in_pixels(char_w);
	let y = builder.y.in_pixels(char_h);
	let h = builder.h.in_pixels(char_h);
	let mut points = vec![];
	for py in range(0, h) {
		let p = py as f32 / h as f32;
		let sp = 1f32 - p;
		let (start_r, start_g, start_b) = builder.bottom_color.get_rgb();
		let (end_r, end_g, end_b) = builder.top_color.get_rgb();
		let r = start_r as f32 * sp + end_r as f32 * p;
		let g = start_g as f32 * sp + end_g as f32 * p;
		let b = start_b as f32 * sp + end_b as f32 * p;
		for (i, value) in data.iter().enumerate() {
			if py > *value {
				continue;
			}
			let p = sdl2::rect::Point::new(x  + i as i32, y + (h-py));
			points.push(p);
		}
		let _ = dst.set_draw_color(sdl2::pixels::RGB(r as u8, g as u8, b as u8));
		let _ = dst.draw_points(points.as_slice());
		points.clear();
	}
}

fn draw_vertical_line(builder: &LineChartBuilder, renderer: &sdl2::render::Renderer, index: i32, value: i32, start_color: sdl2::pixels::Color, end_color: sdl2::pixels::Color) {
	let char_w = builder.layer.char_w;
	let char_h = builder.layer.char_h;
	let x = builder.x.in_pixels(char_w);
	let y = builder.y.in_pixels(char_h);
	let h = builder.h.in_pixels(char_h);
	for py in range(0, value) {
		let p = py as f32 / h as f32;
		let sp = 1f32 - p;
		let (start_r, start_g, start_b) = start_color.get_rgb();
		let (end_r, end_g, end_b) = end_color.get_rgb();
		let r = start_r as f32 * sp + end_r as f32 * p;
		let g = start_g as f32 * sp + end_g as f32 * p;
		let b = start_b as f32 * sp + end_b as f32 * p;

		let p = sdl2::rect::Point::new(x + index, (y + h-py) );
		let _ = renderer.set_draw_color(sdl2::pixels::RGB(r as u8, g as u8, b as u8));
		let _ = renderer.draw_point(p);
	}
}
