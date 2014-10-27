extern crate sdl2;

use std::collections::RingBuf;
use std::collections::Deque;
use std::cmp::min;
use std::cmp::max;

use sdl2::pixels::RGB;

use widget;
use widget::WidgetImpl;
use widget::EventHandlingResult;
use widget::WidgetEvent;
use widget::WidgetPos;

pub struct Chart {
	pub data: Vec<i32>,
	pub height: u32,
	pub width: u32,
	pub selected_column: Option<u32>,
}

impl WidgetImpl for Chart {

    fn handle_event(&mut self, event: WidgetEvent) -> EventHandlingResult {
    	match event {
    		widget::WidgetMouseMoveEvent(pos) => {
    			let WidgetPos(x, _) = pos;
    			self.selected_column = Some(x);
    			return widget::HANDLED | widget::NEED_REDRAW;
    		},
    		//widget::MouseLeaveEvent => {
    		//	self.selected_column = None;	
    		//},
            _ => {}
    	}
    	return widget::NONE;
    }

    fn draw(&self, dst: &sdl2::render::Renderer) {
    	self.draw(dst);
    }
}

impl Chart {
	pub fn new(width: u32, height: u32) -> Chart {
		Chart {
			width: width,
			height: height,
			data: vec![],
			selected_column: None
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

	fn draw_rect_gradient(&self, dst: &sdl2::render::Renderer, x: u32, y: u32, w: u32, h: u32, start_color: sdl2::pixels::Color, end_color: sdl2::pixels::Color) {
		for i in range(0, h) {
			let p = i as f32 / h as f32;
			let sp = 1f32 - p;
			let (start_r, start_g, start_b) = start_color.get_rgb();
			let (end_r, end_g, end_b) = end_color.get_rgb();
			let mut r = start_r as f32 * sp + end_r as f32 * p;
			let mut g = start_g as f32 * sp + end_g as f32 * p;
			let mut b = start_b as f32 * sp + end_b as f32 * p;
			let start = sdl2::rect::Point::new((x as u32) as i32, (y+i) as i32);
			let end = sdl2::rect::Point::new((x+w as u32) as i32, (y+i) as i32);
			dst.set_draw_color(sdl2::pixels::RGB(r as u8, g as u8, b as u8));
			dst.draw_line(start, end);
		}
	}

	fn draw_horizontal_lines(&self, dst: &sdl2::render::Renderer, data: &[i32], start_color: sdl2::pixels::Color, end_color: sdl2::pixels::Color) {
		let mut points = vec![];
		for y in range(0, self.height) {
			let p = y as f32 / self.height as f32;
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
				let p = sdl2::rect::Point::new((i as u32) as i32, (self.height-y) as i32);
				points.push(p);
			}
			dst.set_draw_color(sdl2::pixels::RGB(r as u8, g as u8, b as u8));
			dst.draw_points(points.as_slice());
			points.clear();
		}
	}

	fn draw_vertical_line(&self, renderer: &sdl2::render::Renderer, x: u32, value: i32, start_color: sdl2::pixels::Color, end_color: sdl2::pixels::Color) {
		for y in range(0, value) {
			let p = y as f32 / self.height as f32;
			let sp = 1f32 - p;
			let (start_r, start_g, start_b) = start_color.get_rgb();
			let (end_r, end_g, end_b) = end_color.get_rgb();
			let mut r = start_r as f32 * sp + end_r as f32 * p;
			let g = start_g as f32 * sp + end_g as f32 * p;
			let b = start_b as f32 * sp + end_b as f32 * p;

			let p = sdl2::rect::Point::new((x) as i32, (self.height-y as u32) as i32);
			renderer.set_draw_color(sdl2::pixels::RGB(r as u8, g as u8, b as u8));
			renderer.draw_point(p);
		}
	}

	pub fn draw(&self, renderer: &sdl2::render::Renderer) {
		renderer.set_draw_color(sdl2::pixels::RGB(32 , 32, 32));
		renderer.fill_rect(&sdl2::rect::Rect::new(0, 0, self.width as i32, self.height as i32));
		let horizontal_pixels_between_values = self.width / ::std::cmp::min(self.width, self.data.len() as u32) ;
		let scaled_data = Chart::create_scaled_chart_data(self.data.as_slice(), horizontal_pixels_between_values);
		self.draw_horizontal_lines(renderer, scaled_data.as_slice(), sdl2::pixels::RGB(42, 42, 42), sdl2::pixels::RGB(200, 200, 200));
		//
		let mut points = vec![];
		for (i, v) in scaled_data.iter().enumerate() {
			let p = sdl2::rect::Point::new((i as u32) as i32, (self.height - *v as u32) as i32);
			points.push(p);
		}
		renderer.set_draw_color(sdl2::pixels::RGB(255, 255, 255));
		renderer.draw_points(points.as_slice());
		//
		if self.selected_column.is_some() {
			let index = self.selected_column.unwrap();
			if index < scaled_data.len() as u32 {
				self.draw_vertical_line(renderer, index, scaled_data[index as uint], sdl2::pixels::RGB(112, 42, 42), sdl2::pixels::RGB(255, 200, 200));
				self.draw_rect_gradient(renderer, index - 13, (self.height as i32 - scaled_data[index as uint] - 30) as u32, 26, 15, RGB(174, 67, 75), RGB(166, 38, 51));
			}
		}
	}
}
