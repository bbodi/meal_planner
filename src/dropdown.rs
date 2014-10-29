// TODO 2 pixel széles border

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
use imgui::IndexValue;
use imgui::SizeInCharacters;

pub struct DropdownBuilder<'a> {
	disabled: bool,
	x: SizeInCharacters,
	y: SizeInCharacters,
	labels: &'a [&'a str],
	layer: &'a mut imgui::Layer,
	value: &'a mut IndexValue + 'a
}

impl<'a> DropdownBuilder<'a> {
	pub fn new(layer: &'a mut imgui::Layer, labels: &'a [&str], value: &'a mut IndexValue, x: SizeInCharacters, y: SizeInCharacters) -> DropdownBuilder<'a> {
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
	pub fn x(mut self, v: SizeInCharacters) -> DropdownBuilder<'a> {self.x = v; self}
	pub fn y(mut self, v: SizeInCharacters) -> DropdownBuilder<'a> {self.y = v; self}
	pub fn right(mut self, x: SizeInCharacters) -> DropdownBuilder<'a> {
		self.x = self.layer.last_x + self.layer.last_w + x;
		self
	}

	pub fn down(mut self, y: SizeInCharacters) -> DropdownBuilder<'a> {
		self.y = self.layer.last_y + self.layer.last_h + y;
		self
	}
	

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
	let char_w = builder.layer.char_w;
	let char_h = builder.layer.char_h;
	let x = builder.x.in_pixels(char_w);
	let y = builder.y.in_pixels(char_h);
	let h = builder.layer.char_h;
	let longest_word_len = get_longest_word_len(builder.labels);
	let label_w = char_w * longest_word_len;
	let down_arrow_w = char_w*2;
	let all_w = label_w + down_arrow_w;

	builder.layer.last_x = builder.x;
	builder.layer.last_y = builder.y;
	builder.layer.last_w = SizeInCharacters(longest_word_len + 2);
	builder.layer.last_h = SizeInCharacters(1);

	let was_hot = builder.layer.is_hot_widget(x, y);
	let was_active = builder.layer.is_active_widget(x, y);
	let hover = builder.layer.is_mouse_in(x, y, all_w, h);
	let click = builder.layer.is_mouse_released() && hover;
	let just_clicked = click && !was_active;
	let click_while_active = click && was_active;
	let clicked_out = builder.layer.is_mouse_released() && !hover && was_active;
	let active = was_active && !clicked_out;

	if just_clicked {
		builder.layer.set_active_widget(x, y);
	} else if click_while_active {
		builder.layer.clear_active_widget();
	} else if clicked_out {
		if builder.layer.is_mouse_in(x, y + char_h, all_w, builder.labels.len() as i32 * char_h) {
			builder.value.set( ((builder.layer.mouse_y() - (y + char_h)) / char_h) as uint );
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
	renderer.draw_rect(&Rect::new(x, y, label_w, h));
	imgui::draw_rect_gradient(renderer, x+1, y+1, label_w-2, h-2, top_color, bottom_color);
	if builder.labels[builder.value.get()].len() > 0 {
		imgui::draw_text(x+1, y+1, renderer, &builder.layer.font, builder.labels[builder.value.get()], RGB(221, 221, 221));
	}

	renderer.set_draw_color(sdl2::pixels::RGB(0, 0, 0));
	renderer.draw_rect(&Rect::new(x+label_w, y, down_arrow_w, h));
	imgui::draw_rect_gradient(renderer, x+label_w+1, y+1, down_arrow_w-2, h-2, top_color, bottom_color);
	let arrow_char = if active {"▲"} else {"▼"};
	imgui::draw_text(x + label_w+char_w/3, y, renderer, &builder.layer.font, arrow_char, RGB(221, 221, 221));

	if active {
		for (i, label) in builder.labels.iter().enumerate() {
			let i = i as i32;
			renderer.set_draw_color(sdl2::pixels::RGB(0, 0, 0));
			renderer.draw_rect(&Rect::new(x, y + (1+i)*char_h, all_w, h));

			let color = match builder.layer.is_mouse_in(x, y + (1+i)*char_h, all_w, h) {
				true => sdl2::pixels::RGB(82, 82, 90),
				false => sdl2::pixels::RGB(51, 51, 51),
			};
			renderer.set_draw_color(color);
			renderer.fill_rect(&Rect::new(x+1, y +1+ (1+i)*char_h, all_w-2, h-2));
			if label.len() == 0 {
				continue;
			}
			imgui::draw_text(x+1, y +1+ (1+i)*char_h, renderer, &builder.layer.font, *label, RGB(198, 198, 198));
		}
	}
	false
}