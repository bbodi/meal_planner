use sdl2::pixels::RGB;

use std::cmp::max;
use std::cmp::min;
use std::iter::AdditiveIterator;

use imgui::base;
use imgui::base::SizeInCharacters;

use imgui::textfield::textfield_f32;
use imgui::label::label;
use imgui::panel::panel;
use imgui::button::button;
use imgui::header::header;
use imgui::dropdown::dropdown;

pub struct TimelineWindow {
	pos: u32,
	zoom_level: f32,
	w: SizeInCharacters,
	h: SizeInCharacters,
	x: SizeInCharacters,
	y: SizeInCharacters,
	last_mouse_x: i32,
	last_mouse_y: i32,
	data: Vec<f32>,
	avg_data: Vec<f32>,
	smoothing_constant: f32,
}

impl TimelineWindow {
	pub fn new(x: SizeInCharacters, y: SizeInCharacters, w: SizeInCharacters, h: SizeInCharacters) -> TimelineWindow {
		let mut last: f32 = 30f32;
	    let mut data = vec![];
	    /*for i in range(0, 20i32) {
	    	data.push(78f32 + i as f32 * 0.1f32);
	    }*/
	    for _ in range(0, 100000i32) {
	        last = last + ::std::rand::random::<f32>() * 2f32 - 1f32;
	        if last < 0f32 {
	        	last = 30f32;
	        } else if last > 60f32 {
	        	last = 30f32;
	        }
	        data.push(last);
	    }

		let mut t = TimelineWindow {
			pos: 10000,
			zoom_level: 100f32,
			w: w,
			h: h,
			x: x,
			y: y,
			last_mouse_x: 0,
			last_mouse_y: 0,
			data: data,
			smoothing_constant: 0.9f32,
			avg_data: vec![],
		};
		t.calc_ema();
		return t;
	}	

	fn calc_ema(&mut self) {
		self.avg_data.clear();
		let mut last_data = self.data[0];
		println!("{}", last_data);
	    for (i, v) in self.data.slice_from(1).iter().enumerate() {
	    	let smoothing_percentage = 1f32 - self.smoothing_constant;
	    	let curr_data = last_data + smoothing_percentage * (*v - last_data);
	    	self.avg_data.push( curr_data );
	    	last_data = curr_data;
	    }
	}

	pub fn do_logic(&mut self, layer: &mut base::Layer) -> bool {
		let char_w = layer.char_w;
		let char_h = layer.char_h;
		let widget_x = self.x.in_pixels(char_w);
		let widget_y = self.y.in_pixels(char_h);
		let widget_w = self.w.in_pixels(char_w);
		let widget_h = self.h.in_pixels(char_h);
		let bottom = (widget_y+widget_h) - (widget_h as f32*0.1f32) as i32;

	    header(layer, "Timeline", self.w, self.h)
	        .x(self.x)
	        .y(self.y)
	        .draw();

	    panel(layer, SizeInCharacters(30), SizeInCharacters(20))
			.x(SizeInCharacters(10))
	        .y(SizeInCharacters(3))
	        .draw();
	    let result = textfield_f32(layer, &mut self.smoothing_constant, SizeInCharacters(20))
	     	.inner_down(SizeInCharacters(1))
	     	.inner_right(SizeInCharacters(1))
	        .default_text("Smoothing constant...")
	        .draw();
	    if result.is_some() && result.unwrap() == ::imgui::textfield::Enter {
	    	self.calc_ema();
	    }
	    let first_column = layer.last_x + SizeInCharacters(1);
	    let a = (self.zoom_level / 3f32.sqrt()) as u32;
	    let x1 = self.pos - a;
	    let x2 = self.pos + a;
	    let range_w = (x2 - x1) as i32;
	    let y1 = 300i32;
	    let y2 = 1500i32;
	    let range_h = (y2 - y1) as i32;
	    let screen_step_w = widget_w as f32 / (2f32*a as f32);
	    let screen_step_h = widget_h as f32 / (range_h as f32);

	    label(layer, format!("screen_step_w: {}", screen_step_w).as_slice())
	     	.down(SizeInCharacters(1))
	        .draw();

	    let mut last_text_end_pos = 0;
	    let real_value_rect_size = 5;
	    // TODO: limit_to(9), limit_from(1)
	    // or limit_to_max(9), limit_to_min(1)
	    let trend_value_rect_size = screen_step_w.min(7f32).max(1f32) as i32;
	    for (i, num) in range(x1, x2).enumerate() {
	    	let str = format!("{}",num);
	    	let str_len = base::text_len(str.as_slice()) as i32;
	    	let x = (widget_x as f32 + i as f32 * screen_step_w) as i32;
	    	let text_x = x - (str_len / 2) * char_w;
	    	if text_x > last_text_end_pos {
	    		layer.bottom_surface.draw_text(text_x, bottom, str.as_slice(), RGB(255, 255, 255));
	    		last_text_end_pos = text_x + str_len * char_w + char_w;
	    	}
	    	layer.bottom_surface.draw_line(x, bottom, x, bottom - 10, RGB(0, 0, 255));

	    	let data_indx = x1 as uint + i;
	    	if data_indx >= self.data.len() {
	    		continue;
	    	}
	    	let real_value = (self.data[data_indx]*10f32) as i32;
	    	let trend = (self.avg_data[data_indx]*10f32) as i32;
	    	let color = if real_value > trend {RGB(255, 0, 0)} else {RGB(0, 255, 0)};

	    	let real_y = bottom - 10 - real_value;
	    	let trend_y = bottom - 10 - trend;
	    	layer.bottom_surface.fill_rect(x-trend_value_rect_size/2, trend_y-trend_value_rect_size/2, trend_value_rect_size, trend_value_rect_size, RGB(255, 0, 255));
	    	if trend != real_value {
	    		layer.bottom_surface.fill_rect(x-real_value_rect_size/2, real_y-real_value_rect_size/2, real_value_rect_size, real_value_rect_size, color);
	    		layer.bottom_surface.draw_line(x, min(real_y+real_value_rect_size/2, trend_y+trend_value_rect_size/2), x, max(real_y-real_value_rect_size/2, trend_y-trend_value_rect_size/2), color);
	    	}

	    	let mouse_index = (layer.mouse_x() as f32 / screen_step_w) as uint;
			if layer.is_mouse_in(widget_x, widget_y, widget_w, widget_h) && mouse_index == i {
				let txt = format!("{}, {}", real_value, trend);
				layer.bottom_surface.fill_rect(x-10, real_y-10, char_w*10, char_h, RGB(255, 0, 0));
				layer.bottom_surface.draw_text(x-10, real_y-10, txt.as_slice(), RGB(255, 255, 255));
			}
		}
		// ***************************
		// Calculating best fit trends
		// ***************************
		let n = (x2-x1) as f32;
		let index_sum = range(0, n as uint).sum() as f32;
		let day_sum = self.avg_data.slice_or_fail(&(x1 as uint), &(x2 as uint)).iter()
			.map(|x| *x)
			.sum();
		let day_index_sum = self.avg_data.slice_or_fail(&(x1 as uint), &(x2 as uint)).iter().enumerate()
			.map(|(i, x)| *x *i as f32)
			.sum();
		let a = n*day_index_sum - index_sum*day_sum;

		let index_sum_square = range(0, x2-x1).map(|x| x * x).sum() as f32;
		let denominator = n*index_sum_square-index_sum*index_sum;
		let m = a / denominator;

		let b_upper = day_sum * index_sum_square - index_sum*day_index_sum;
		let b = b_upper / denominator;
		for i in range(0, n as i32) {
			let x = (widget_x as f32 + i as f32 * screen_step_w) as i32;
			layer.bottom_surface.fill_rect(x-real_value_rect_size/2, bottom - 10 - ((i as f32 * m + b)*10f32) as i32, real_value_rect_size, real_value_rect_size, RGB(0, 255, 255));
		}
		
		label(layer, format!("m: {:.2f}", m).as_slice())
     		.down(SizeInCharacters(1))
        	.draw();
        label(layer, format!("avg weight change / week: {:.2f}", 7f32*m).as_slice())
     		.down(SizeInCharacters(1))
        	.draw();
        label(layer, format!("average daily calorie: {:.2f}", 7700f32*m).as_slice())
     		.down(SizeInCharacters(1))
        	.draw();

		for (i, y_axis) in ::std::iter::range_step(0, 60i32, 1).enumerate() {
			let right = widget_x + (widget_w as f32*0.05f32) as i32;
			let txt = format!("{}", y_axis);
			let y_pos = bottom - 10 - ((y_axis as f32) * 10f32) as i32;
			layer.bottom_surface.draw_text(right, y_pos, txt.as_slice(), RGB(255, 255, 255));
		}

		//let mut points = vec![];
		for (i, v) in self.data.iter().enumerate() {
			//let p = sdl2::rect::Point::new(x + i as i32, y + (h - *v ) );
			//points.push(p);
			
		}

		let multiplier = if layer.control_keys.ctrl.down {100} else {1};
		if layer.control_keys.left.down {
			self.pos = self.pos - 1 * multiplier;
		} else if layer.control_keys.right.down {
			self.pos = self.pos + 1 * multiplier;
		} else if layer.control_keys.down.down && self.zoom_level > 10f32 {
			self.zoom_level = self.zoom_level - 1f32 * multiplier as f32;
			println!("Zoom level: {}", self.zoom_level);
		} else if layer.control_keys.up.down {
			self.zoom_level = self.zoom_level + 1f32 * multiplier as f32;
			println!("Zoom level: {}", self.zoom_level);
		}

	    return false;
	}
}

