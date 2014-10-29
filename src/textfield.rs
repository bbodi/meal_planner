extern crate sdl2;
extern crate sdl2_ttf;

use std::iter::AdditiveIterator;
use std::collections::RingBuf;
use std::collections::Deque;
use std::cmp::min;
use std::cmp::max;

use sdl2::pixels::RGB;
use sdl2::rect::Rect;
use sdl2::rect::Point;

use imgui;
use imgui::SizeInCharacters;

pub struct TextFieldBuilder<'a> {
	disabled: bool,
	x: SizeInCharacters,
	y: SizeInCharacters, 
	w: SizeInCharacters, 
	text: &'a mut String,
	default_text: &'a str,
	label: &'a str,
	layer: &'a mut imgui::Layer
}

pub struct State {
	cursor_pos: i32,
	cursor_visible: bool,
	cursor_visibility_change_tick: uint
}

impl State {
	pub fn new(text: &str) -> State {
		State {
			cursor_pos: text.len() as i32 ,
			cursor_visibility_change_tick: 0,
			cursor_visible: false,
		}
	}
}

impl<'a> TextFieldBuilder<'a> {
	pub fn new(layer: &'a mut imgui::Layer, text: &'a mut String, x: SizeInCharacters, y: SizeInCharacters, w: SizeInCharacters)-> TextFieldBuilder<'a> {
		TextFieldBuilder {
			disabled: false,
			x: x,
			y: y,
			w: w,
			text: text,
			layer: layer,
			default_text: "",
			label: "",
		}
	}

	pub fn disabled(mut self, v: bool) -> TextFieldBuilder<'a> {self.disabled = v; self}
	pub fn default_text(mut self, v: &'a str) -> TextFieldBuilder<'a> {self.default_text = v; self}
	pub fn label(mut self, v: &'a str) -> TextFieldBuilder<'a> {self.label = v; self}
	pub fn x(mut self, v: SizeInCharacters) -> TextFieldBuilder<'a> {self.x = v; self}
	pub fn y(mut self, v: SizeInCharacters) -> TextFieldBuilder<'a> {self.y = v; self}
	pub fn right(mut self, x: SizeInCharacters) -> TextFieldBuilder<'a> {
		self.x = self.layer.last_x + self.layer.last_w + x;
		self
	}

	pub fn down(mut self, y: SizeInCharacters) -> TextFieldBuilder<'a> {
		self.y = self.layer.last_y + self.layer.last_h + y;
		self
	}
	

	pub fn draw(&mut self, renderer: &sdl2::render::Renderer) -> bool {
		draw(self, renderer)
	}
}

pub fn draw(builder: &mut TextFieldBuilder, renderer: &sdl2::render::Renderer) -> bool {
	let char_w = builder.layer.char_w;
	let char_h = builder.layer.char_h;
	let x = builder.x.in_pixels(char_w);
	let y = builder.y.in_pixels(char_h);
	let w = builder.w.in_pixels(char_w);
	let h = char_h;
	let label_width = builder.label.len() as i32  * char_w;	
	let was_hot = builder.layer.is_hot_widget(x, y);
	let was_active = builder.layer.is_active_widget(x, y);
	let hover = builder.layer.is_mouse_in(x, y, label_width+w, h);
	let just_clicked = builder.layer.is_mouse_down() && hover && !was_active;
	let clicked_out = builder.layer.is_mouse_down() && !hover && was_active;
	let active = was_active && !clicked_out;

	builder.layer.last_x = builder.x;
	builder.layer.last_y = builder.y;
	builder.layer.last_w = builder.w + SizeInCharacters(builder.label.len() as i32);
	builder.layer.last_h = SizeInCharacters(1);
	
	if active {
		let maybe_char = builder.layer.input_char();
		let text_len = builder.text.as_slice().chars().count() as i32;
		let control_keys = builder.layer.control_keys;
		let state = builder.layer.get_mut_textfield_state(builder.text);
		state.cursor_pos = ::std::cmp::min(state.cursor_pos, text_len);
		
		if state.cursor_pos > 0 && control_keys.backspace.just_pressed {
			let idx: uint = builder.text.as_slice().graphemes(true).take(state.cursor_pos as uint - 1).map(|g| g.len()).sum();
			builder.text.remove(idx);
			state.cursor_pos = state.cursor_pos-1;
        } else if state.cursor_pos > 0 && control_keys.left.just_pressed { 
        	state.cursor_pos = state.cursor_pos-1;
        } else if state.cursor_pos < text_len && control_keys.right.just_pressed { 
        	state.cursor_pos = state.cursor_pos+1;
        } else if state.cursor_pos < text_len && control_keys.del.just_pressed { 
        	let idx: uint = builder.text.as_slice().graphemes(true).take(state.cursor_pos as uint).map(|g| g.len()).sum();            
			builder.text.remove(idx);
        } else {
			if maybe_char.is_some() {
				let ch = maybe_char.unwrap();
				//
				let idx: uint = builder.text.as_slice().graphemes(true).take(state.cursor_pos as uint).map(|g| g.len()).sum();
				builder.text.insert(idx, ch);
				state.cursor_pos = state.cursor_pos+1;
			}
		}
	}


	if just_clicked {
		builder.layer.set_active_widget(x, y);
	} else if clicked_out {
		builder.layer.clear_active_widget();
	}

	if hover && !was_hot {
		builder.layer.set_hot_widget(x, y);
	} else if was_hot && !hover {
		builder.layer.clear_hot_widget();
	}


	renderer.set_draw_color(sdl2::pixels::RGB(32 , 32, 32));

	if hover || active {
		imgui::draw_rect_gradient(renderer, label_width+x, y, w, h, RGB(51, 51, 51), RGB(61, 61, 61));
	} else {
		imgui::draw_rect_gradient(renderer, label_width+x, y, w, h, RGB(51, 51, 51), RGB(51, 51, 51));
	}
	let border_width = 2;
	imgui::draw_rect(renderer, label_width+x, y, w+border_width, h+border_width, 2, RGB(0, 0, 0));
	if builder.text.len() > 0 {
		imgui::draw_text(label_width+x+border_width, y, renderer, &builder.layer.font, builder.text.as_slice(), RGB(204, 204, 204));
	} else if builder.default_text != "" && !active {
		imgui::draw_text(label_width+x+border_width, y, renderer, &builder.layer.font, builder.default_text.as_slice(), RGB(113, 113, 113));
	}
	if label_width > 0 {
		imgui::draw_text(x+border_width, y, renderer, &builder.layer.font, builder.label.as_slice(), RGB(221, 221, 221));
	}

	if active {
		{
			let state = builder.layer.get_textfield_state(builder.text);
			if state.cursor_visible {
				imgui::draw_text(label_width+x + char_w as i32 *state.cursor_pos, y, renderer, &builder.layer.font, "_", RGB(204, 204, 204));
			}
		}
	
		let tick = builder.layer.tick();
		let mut state = builder.layer.get_mut_textfield_state(builder.text);
		if state.cursor_visibility_change_tick < tick {
			state.cursor_visible = !state.cursor_visible;
			state.cursor_visibility_change_tick = tick + 500;
		}
	}
	return builder.layer.control_keys.enter.just_pressed;
}