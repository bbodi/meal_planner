extern crate sdl2;
extern crate sdl2_ttf;

use std::collections::RingBuf;
use std::collections::Deque;
use std::cmp::min;
use std::cmp::max;

use sdl2::pixels::RGB;
use sdl2::rect::Rect;

use imgui;

pub struct LineChartBuilder<'a> {
	x: u32,
	y: u32, 
	w: u32, 
	h: u32,
	label: &'a str,
	maybe_data: Option<&'a [i32]>,

	layer: &'a mut imgui::Layer
}

impl<'a> LineChartBuilder<'a> {
	pub fn new(layer: &'a mut imgui::Layer, label: &'a str, x: u32, y: u32, w: u32, h: u32) -> LineChartBuilder<'a> {
		LineChartBuilder {
			x: x,
			y: y,
			w: w,
			h: h,
			label: label,
			layer: layer,
			maybe_data: None
		}
	}

	pub fn data(mut self, data: &'a [i32]) -> LineChartBuilder { self.maybe_data = Some(data); self}

	pub fn draw(&mut self, renderer: &sdl2::render::Renderer) {
		draw(self, renderer);
	}
}

pub fn draw(builder: &mut LineChartBuilder, renderer: &sdl2::render::Renderer) {
	let selected_column = if builder.layer.is_mouse_in(builder.x, builder.y, builder.w, builder.h) {
		Some(builder.layer.mouse_x())
	} else {
		None
	};

	renderer.set_draw_color(sdl2::pixels::RGB(32 , 32, 32));
	renderer.fill_rect(&sdl2::rect::Rect::new(0, 0, builder.w as i32, builder.h as i32));
	if builder.maybe_data.is_none() {
		return;
	}
	let data = builder.maybe_data.unwrap();
	let horizontal_pixels_between_values = builder.w / ::std::cmp::min(builder.w, data.len() as u32) ;
	let scaled_data = create_scaled_chart_data(data.as_slice(), horizontal_pixels_between_values);
	draw_horizontal_lines(builder.h, renderer, scaled_data.as_slice(), sdl2::pixels::RGB(42, 42, 42), sdl2::pixels::RGB(200, 200, 200));
	
	//
	let mut points = vec![];
	for (i, v) in scaled_data.iter().enumerate() {
		let p = sdl2::rect::Point::new(i as i32, (builder.h - *v as u32) as i32);
		points.push(p);
	}
	renderer.set_draw_color(sdl2::pixels::RGB(255, 255, 255));
	//renderer.draw_points(points.as_slice());
	//
	//
	if horizontal_pixels_between_values > 2 {
		let mut rects = vec![];
		for (i, v) in data.iter().enumerate() {
			let x = i * horizontal_pixels_between_values as uint - 1;
			let y = (builder.h - *v as u32 - 1) as i32;
			let p = sdl2::rect::Rect::new(x as i32, y, 2, 2);
			rects.push(p);
		}
		renderer.set_draw_color(sdl2::pixels::RGB(255, 150, 150));
		renderer.fill_rects(rects.as_slice());
	}
	//
	if selected_column.is_some() {
		let index = selected_column.unwrap();
		if index < scaled_data.len() as u32 {
			let real_index = selected_column.unwrap() / horizontal_pixels_between_values;
			let index = real_index * horizontal_pixels_between_values;
			draw_vertical_line(builder.h, renderer, index, scaled_data[index as uint], RGB(112, 42, 42), sdl2::pixels::RGB(255, 200, 200));
			let x = index - 13;
			let y = (builder.h as i32 - scaled_data[index as uint] - 30) as u32;
			let w = 26u32;
			let h = 15u32;
			let v = scaled_data[index as uint];
			imgui::draw_rect_gradient(renderer, x, y, w, h, RGB(174, 67, 75), RGB(166, 38, 51));
			let texure = imgui::create_text_texture(renderer, &builder.layer.font, format!("{}", v).as_slice(), RGB(255, 255, 255));
			renderer.copy(&texure, None, Some(Rect::new(x as i32, y as i32, w as i32, h as i32)));
		}
	}
}

fn create_scaled_chart_data(data: &[i32], horizontal_pixels_between_values: u32) -> Vec<i32> {
	let mut scaled_data = vec![];
	let mut last_value = data[0];
	scaled_data.push(data[0]);
	for value in data.slice(1, data.len()).iter() {
		for i in ::std::iter::range_inclusive(1, horizontal_pixels_between_values) {
			let diff = *value - last_value;
			let k = (diff as f32 * (i as f32/horizontal_pixels_between_values as f32)) as i32;
			scaled_data.push(last_value + k);
		}
		last_value = *value;
	}
	return scaled_data;
}

fn draw_horizontal_lines(height: u32, dst: &sdl2::render::Renderer, data: &[i32], start_color: sdl2::pixels::Color, end_color: sdl2::pixels::Color) {
	// TODO: find the longest horizontal continous line and draw it!
	let mut points = vec![];
	for y in range(0, height) {
		let p = y as f32 / height as f32;
		let sp = 1f32 - p;
		let (start_r, start_g, start_b) = start_color.get_rgb();
		let (end_r, end_g, end_b) = end_color.get_rgb();
		let mut r = start_r as f32 * sp + end_r as f32 * p;
		let mut g = start_g as f32 * sp + end_g as f32 * p;
		let mut b = start_b as f32 * sp + end_b as f32 * p;
		for (i, value) in data.iter().enumerate() {
			if y as i32 > *value {
				continue;
			}
			let p = sdl2::rect::Point::new((i as u32) as i32, (height-y) as i32);
			points.push(p);
		}
		dst.set_draw_color(sdl2::pixels::RGB(r as u8, g as u8, b as u8));
		dst.draw_points(points.as_slice());
		points.clear();
	}
}

fn draw_vertical_line(height: u32, renderer: &sdl2::render::Renderer, x: u32, value: i32, start_color: sdl2::pixels::Color, end_color: sdl2::pixels::Color) {
	for y in range(0, value) {
		let p = y as f32 / height as f32;
		let sp = 1f32 - p;
		let (start_r, start_g, start_b) = start_color.get_rgb();
		let (end_r, end_g, end_b) = end_color.get_rgb();
		let mut r = start_r as f32 * sp + end_r as f32 * p;
		let g = start_g as f32 * sp + end_g as f32 * p;
		let b = start_b as f32 * sp + end_b as f32 * p;

		let p = sdl2::rect::Point::new((x) as i32, (height-y as u32) as i32);
		renderer.set_draw_color(sdl2::pixels::RGB(r as u8, g as u8, b as u8));
		renderer.draw_point(p);
	}
}