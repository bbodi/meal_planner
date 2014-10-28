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

pub struct TextFieldBuilder<'a> {
	disabled: bool,
	x: u32,
	y: u32, 
	w: u32, 
	h: u32,
	text: &'a mut String,
	default_text: &'a str,
	layer: &'a mut imgui::Layer
}

pub struct State {
	cursor_pos: uint,
	cursor_visible: bool,
	cursor_visibility_change_tick: uint
}

impl State {
	pub fn new(text: &str) -> State {
		State {
			cursor_pos: text.len(),
			cursor_visibility_change_tick: 0,
			cursor_visible: false,
		}
	}
}

impl<'a> TextFieldBuilder<'a> {
	pub fn new(layer: &'a mut imgui::Layer, text: &'a mut String, x: u32, y: u32, w: u32, h: u32) -> TextFieldBuilder<'a> {
		TextFieldBuilder {
			disabled: false,
			x: x,
			y: y,
			w: w,
			h: h,
			text: text,
			layer: layer,
			default_text: ""
		}
	}

	pub fn disabled(mut self, v: bool) -> TextFieldBuilder<'a> {self.disabled = v; self}
	pub fn default_text(mut self, v: &'a str) -> TextFieldBuilder<'a> {self.default_text = v; self}
	

	pub fn draw(&mut self, renderer: &sdl2::render::Renderer) -> bool {
		draw(self, renderer)
	}
}

pub fn draw(builder: &mut TextFieldBuilder, renderer: &sdl2::render::Renderer) -> bool {
	let x = builder.x;
	let y = builder.y;
	let w = builder.w;
	let h = builder.h;
	let was_hot = builder.layer.is_hot_widget(x, y);
	let active = builder.layer.is_active_widget(x, y);
	let hover = builder.layer.is_mouse_in(x, y, w, h);
	let just_clicked = builder.layer.is_mouse_down() && hover && !active;

	
	if active {
		let maybe_char = builder.layer.input_char();
		let text_len = builder.text.as_slice().chars().count();
		let control_keys = builder.layer.control_keys;
		let state = builder.layer.get_mut_textfield_state(builder.x, builder.y);
		state.cursor_pos = ::std::cmp::min(state.cursor_pos, text_len);
		
		if state.cursor_pos > 0 && control_keys.backspace.just_pressed {
			let idx: uint = builder.text.as_slice().graphemes(true).take(state.cursor_pos-1).map(|g| g.len()).sum();
			builder.text.remove(idx);
			state.cursor_pos = state.cursor_pos-1;
        } else if state.cursor_pos > 0 && control_keys.left.just_pressed { 
        	state.cursor_pos = state.cursor_pos-1;
        } else if state.cursor_pos < text_len && control_keys.right.just_pressed { 
        	state.cursor_pos = state.cursor_pos+1;
        } else if state.cursor_pos < text_len && control_keys.del.just_pressed { 
        	let idx: uint = builder.text.as_slice().graphemes(true).take(state.cursor_pos).map(|g| g.len()).sum();            
			builder.text.remove(idx);
        } else {
			if maybe_char.is_some() {
				let ch = maybe_char.unwrap();
				//
				let idx: uint = builder.text.as_slice().graphemes(true).take(state.cursor_pos).map(|g| g.len()).sum();
				builder.text.insert(idx, ch);
				state.cursor_pos = state.cursor_pos+1;
			}
		}
	}


	if just_clicked {
		builder.layer.set_active_widget(x, y);
	}

	if hover && !was_hot {
		builder.layer.set_hot_widget(x, y);
	} else if was_hot && !hover {
		builder.layer.clear_hot_widget();
	}

	renderer.set_draw_color(sdl2::pixels::RGB(32 , 32, 32));
	
	if hover || active {
		//imgui::draw_rect_gradient(renderer, x, y, w, h, RGB(114, 114, 114), RGB(68, 68, 68));
		imgui::draw_rect_gradient(renderer, x, y, w, h, RGB(51, 51, 51), RGB(61, 61, 61));
	} else {
		//imgui::draw_rect_gradient(renderer, x, y, w, h, RGB(93, 93, 93), RGB(44, 44, 44));
		imgui::draw_rect_gradient(renderer, x, y, w, h, RGB(51, 51, 51), RGB(51, 51, 51));
	}
	
	if builder.text.len() > 0 {
		imgui::draw_text(x, y, renderer, &builder.layer.font, builder.text.as_slice(), RGB(204, 204, 204));
	} else if builder.default_text != "" && !active {
		imgui::draw_text(x, y, renderer, &builder.layer.font, builder.default_text.as_slice(), RGB(113, 113, 113));
	}

	if active {
		{
			let state = builder.layer.get_textfield_state(builder.x, builder.y);
			if state.cursor_visible {
				let (text_w, text_h) = match builder.layer.font.size_of_str("_") {
					Ok((w, h)) => (w, h),
					Err(e) => fail!("e"),
				};
				let texure = imgui::create_text_texture(renderer, &builder.layer.font, "_", RGB(204, 204, 204));
				renderer.copy(&texure, None, Some(Rect::new((x as int + text_w*state.cursor_pos as int) as i32, y as i32, text_w as i32, text_h as i32)));		
			}
		}
	
		let tick = builder.layer.tick();
		let mut state = builder.layer.get_mut_textfield_state(builder.x, builder.y);
		if state.cursor_visibility_change_tick < tick {
			state.cursor_visible = !state.cursor_visible;
			state.cursor_visibility_change_tick = tick + 500;
		}
	}
	return builder.layer.control_keys.enter.just_pressed;
}