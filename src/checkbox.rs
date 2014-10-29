extern crate sdl2;
extern crate sdl2_ttf;

use std::collections::RingBuf;
use std::collections::Deque;
use std::cmp::min;
use std::cmp::max;

use sdl2::pixels::RGB;
use sdl2::rect::Rect;
use sdl2::rect::Point;

use imgui;

pub struct CheckboxBuilder<'a> {
	disabled: bool,
	x: i32,
	y: i32, 
	label: &'a str,
	layer: &'a mut imgui::Layer,
	value: &'a mut bool,
}

impl<'a> CheckboxBuilder<'a> {
	pub fn new(layer: &'a mut imgui::Layer, label: &'a str, value: &'a mut bool, x: i32, y: i32) -> CheckboxBuilder<'a> {
		CheckboxBuilder {
			disabled: false,
			x: x,
			y: y,
			label: label,
			layer: layer,
			value: value,
		}
	}

	pub fn disabled(mut self, v: bool) -> CheckboxBuilder<'a> {self.disabled = v; self}
	

	pub fn draw(&mut self, renderer: &sdl2::render::Renderer) -> bool {
		draw(self, renderer)
	}
}

pub fn draw(builder: &mut CheckboxBuilder, renderer: &sdl2::render::Renderer) -> bool {
	let char_w = builder.layer.char_w;
	let char_h = builder.layer.char_h;
	let x = builder.x;
	let y = builder.y;
	let w = char_h + char_w*builder.label.len() as i32;
	let was_active = builder.layer.is_active_widget(x, y);
	let hover = builder.layer.is_mouse_in(x, y, w, char_h);
	let mousebtn_just_released = builder.layer.is_mouse_released() && hover;
	
	
	if mousebtn_just_released {
		*builder.value = !*builder.value;
	}

	if hover {
		imgui::draw_rect_gradient(renderer, builder.x, builder.y, char_h, char_h, RGB(99, 103, 113), RGB(62, 65, 73));
	} else {
		imgui::draw_rect_gradient(renderer, builder.x, builder.y, char_h, char_h, RGB(82, 86, 90), RGB(49, 52, 55));
	}

	if *builder.value {
		renderer.set_draw_color(sdl2::pixels::RGB(51 , 200, 51));		
	} else {
		renderer.set_draw_color(sdl2::pixels::RGB(51 , 51, 51));
	}
	renderer.fill_rect(&Rect::new(x + char_h/3, y + char_h/3, char_h/3, char_h/3));
	imgui::draw_text(x + char_h, y, renderer, &builder.layer.font, builder.label, RGB(151, 151, 151));

	renderer.set_draw_color(sdl2::pixels::RGB(0, 0, 0));
	renderer.draw_rect(&Rect::new(builder.x, builder.y, char_h, char_h));

	return mousebtn_just_released;
}