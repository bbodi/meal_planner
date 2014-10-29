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

pub struct ButtonBuilder<'a> {
	disabled: bool,
	x: i32,
	y: i32, 
	label: &'a str,
	allow_multi_click: bool,
	layer: &'a mut imgui::Layer
}

impl<'a> ButtonBuilder<'a> {
	pub fn new(layer: &'a mut imgui::Layer, label: &'a str, x: i32, y: i32) -> ButtonBuilder<'a> {
		ButtonBuilder {
			disabled: false,
			x: x,
			y: y,
			label: label,
			layer: layer,
			allow_multi_click: false
		}
	}

	pub fn disabled(mut self, v: bool) -> ButtonBuilder<'a> {self.disabled = v; self}
	pub fn allow_multi_click(mut self, v: bool) -> ButtonBuilder<'a> {self.allow_multi_click = v; self}
	

	pub fn draw(&mut self, renderer: &sdl2::render::Renderer) -> bool {
		draw(self, renderer)
	}
}

pub fn draw(builder: &mut ButtonBuilder, renderer: &sdl2::render::Renderer) -> bool {
	let char_w = builder.layer.char_w;
	let char_h = builder.layer.char_h;
	let x = builder.x;
	let y = builder.y;
	let border_width = 2i32;
	let borders_size = border_width * 2;
	let text_border_dist = 3;
	let w = char_w*builder.label.len() as i32 + text_border_dist*2;
	let h = char_h;

	let was_hot = builder.layer.is_hot_widget(x, y);
	let was_active = builder.layer.is_active_widget(x, y);
	let hover = builder.layer.is_mouse_in(x, y, w, h);
	let released = builder.layer.is_mouse_released();

	let button_down = was_active && !released;
	let mouse_down = builder.layer.is_mouse_down();

	if mouse_down && hover {
		builder.layer.set_active_widget(x, y);
	} else if was_active && released {
		builder.layer.clear_active_widget();
	}

	if hover && !was_hot {
		builder.layer.set_hot_widget(x, y);
	} else if was_hot && !hover {
		builder.layer.clear_hot_widget();
	}

	renderer.set_draw_color(sdl2::pixels::RGB(32 , 32, 32));
	
	if button_down {
		imgui::draw_rect_gradient(renderer, x, y, w, h, RGB(48, 48, 48), RGB(83, 83, 83));
	} else if hover {
		imgui::draw_rect_gradient(renderer, x, y, w, h, RGB(114, 114, 114), RGB(68, 68, 68));
	} else {
		imgui::draw_rect_gradient(renderer, x, y, w, h, RGB(82, 85, 90), RGB(47, 50, 53));
	}
	imgui::draw_rect(renderer, x, y, w+border_width, h+border_width, 2, RGB(0, 0, 0));
	imgui::draw_text(border_width+text_border_dist+x, y+border_width, renderer, &builder.layer.font, builder.label, RGB(151, 151, 151));
	return (released && hover) || (button_down && hover && builder.allow_multi_click);
}