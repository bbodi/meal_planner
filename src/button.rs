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
	x: u32,
	y: u32, 
	w: u32, 
	h: u32,
	label: &'a str,
	allow_multi_click: bool,
	layer: &'a mut imgui::Layer
}

impl<'a> ButtonBuilder<'a> {
	pub fn new(layer: &'a mut imgui::Layer, label: &'a str, x: u32, y: u32, w: u32, h: u32) -> ButtonBuilder<'a> {
		ButtonBuilder {
			disabled: false,
			x: x,
			y: y,
			w: w,
			h: h,
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
	let x = builder.x;
	let y = builder.y;
	let w = builder.w;
	let h = builder.h;
	let was_hot = builder.layer.is_hot_widget(x, y);
	let was_active = builder.layer.is_active_widget(x, y);
	let hover = builder.layer.is_mouse_in(x, y, w, h);
	let click = builder.layer.is_mouse_down() && hover;
	let just_clicked = click && !was_active;
	
	
	if was_active && !click {
		builder.layer.clear_active_widget();
	} else if just_clicked {
		builder.layer.set_active_widget(x, y);
	}

	if hover && !was_hot {
		builder.layer.set_hot_widget(x, y);
	} else if was_hot && !hover {
		builder.layer.clear_hot_widget();
	}

	renderer.set_draw_color(sdl2::pixels::RGB(32 , 32, 32));
	
	if click {
		imgui::draw_rect_gradient(renderer, x, y, w, h, RGB(48, 48, 48), RGB(83, 83, 83));
	} else if hover {
		imgui::draw_rect_gradient(renderer, x, y, w, h, RGB(114, 114, 114), RGB(68, 68, 68));
	} else {
		imgui::draw_rect_gradient(renderer, x, y, w, h, RGB(93, 93, 93), RGB(44, 44, 44));
	}
	let texure = imgui::create_text_texture(renderer, &builder.layer.font, builder.label, RGB(151, 151, 151));
	renderer.copy(&texure, None, Some(Rect::new(x as i32, y as i32, w as i32, h as i32)));
	return just_clicked || (click && builder.allow_multi_click);
}