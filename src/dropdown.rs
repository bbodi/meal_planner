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

pub struct DropdownBuilder<'a> {
	disabled: bool,
	x: i32,
	y: i32,
	labels: &'a [&'a str],
	layer: &'a mut imgui::Layer,
	value: &'a mut uint
}

impl<'a> DropdownBuilder<'a> {
	pub fn new(layer: &'a mut imgui::Layer, labels: &'a [&str], value: &'a mut uint, x: i32, y: i32) -> DropdownBuilder<'a> {
		DropdownBuilder {
			disabled: false,
			x: x,
			y: y,
			labels: labels,
			layer: layer,
			value: value,
		}
	}

	pub fn disabled(mut self, v: bool) -> DropdownBuilder<'a> {self.disabled = v; self}
	

	pub fn draw(&mut self, renderer: &sdl2::render::Renderer) -> bool {
		draw(self, renderer)
	}
}

fn get_longest_word_len(labels: &[&str]) -> i32 {
	let mut len = 0;
	for label in labels.iter() {
		if len < label.len() {
			len = label.len();
		}
	}
	return len as i32;
}

pub fn draw(builder: &mut DropdownBuilder, renderer: &sdl2::render::Renderer) -> bool {
	let h = builder.layer.char_h as i32;
	let char_w = builder.layer.char_w as i32;
	let char_h = builder.layer.char_h as i32;
	let label_w = char_w * get_longest_word_len(builder.labels);
	let down_arrow_w = char_w*2;
	let all_w = label_w + down_arrow_w;

	let was_hot = builder.layer.is_hot_widget(builder.x, builder.y);
	let was_active = builder.layer.is_active_widget(builder.x, builder.y);
	let hover = builder.layer.is_mouse_in(builder.x, builder.y, all_w, h);
	let click = builder.layer.is_mouse_released() && hover;
	let just_clicked = click && !was_active;
	let click_while_active = click && was_active;
	let clicked_out = builder.layer.is_mouse_released() && !hover && was_active;
	let active = was_active && !clicked_out;

	if just_clicked {
		builder.layer.set_active_widget(builder.x, builder.y);
	} else if click_while_active {
		builder.layer.clear_active_widget();
	} else if clicked_out {
		if builder.layer.is_mouse_in(builder.x, builder.y + char_h, label_w, builder.labels.len() as i32 * char_h) {
			*builder.value = ((builder.layer.mouse_y() - (builder.y + char_h)) / char_h) as uint;
		}
		builder.layer.clear_active_widget();
	}
	
	let top_color = match hover {
		false => RGB(82, 85, 90),
		true => RGB(102, 105, 110),
	};
	let bottom_color = match hover {
		false => RGB(47, 50, 53),
		true => RGB(47, 50, 53),
	};
	renderer.set_draw_color(sdl2::pixels::RGB(0, 0, 0));
	renderer.draw_rect(&Rect::new(builder.x, builder.y, label_w, h));
	imgui::draw_rect_gradient(renderer, builder.x+1, builder.y+1, label_w-2, h-2, top_color, bottom_color);
	if builder.labels[*builder.value].len() > 0 {
		imgui::draw_text(builder.x+1, builder.y+1, renderer, &builder.layer.font, builder.labels[*builder.value], RGB(221, 221, 221));
	}

	renderer.set_draw_color(sdl2::pixels::RGB(0, 0, 0));
	renderer.draw_rect(&Rect::new(builder.x+label_w, builder.y, down_arrow_w, h));
	imgui::draw_rect_gradient(renderer, builder.x+label_w+1, builder.y+1, down_arrow_w-2, h-2, top_color, bottom_color);
	imgui::draw_text(builder.x + label_w+char_w/3, builder.y, renderer, &builder.layer.font, "▼".as_slice(), RGB(221, 221, 221));

	if active {
		for (i, label) in builder.labels.iter().enumerate() {
			renderer.set_draw_color(sdl2::pixels::RGB(0, 0, 0));
			renderer.draw_rect(&Rect::new(builder.x, builder.y + (1+i as i32)*char_h, all_w, h));

			let color = match builder.layer.is_mouse_in(builder.x, builder.y + (1+i as i32)*char_h, all_w, h) {
				true => sdl2::pixels::RGB(82, 82, 90),
				false => sdl2::pixels::RGB(51, 51, 51),
			};
			renderer.set_draw_color(color);
			renderer.fill_rect(&Rect::new(builder.x+1, builder.y +1+ (1+i as i32)*char_h, all_w-2, h-2));
			if label.len() == 0 {
				continue;
			}
			imgui::draw_text(builder.x+1, builder.y +1+ (1+i as i32)*char_h, renderer, &builder.layer.font, *label, RGB(198, 198, 198));
		}
	}
	
	/*let x = builder.x;
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
	return just_clicked || (click && builder.allow_multi_click);*/
	false
}