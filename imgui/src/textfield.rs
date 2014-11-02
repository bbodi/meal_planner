extern crate sdl2;
extern crate sdl2_ttf;

use std::iter::AdditiveIterator;

use sdl2::pixels::RGB;
use base;
use base::SizeInCharacters;

pub enum TextFieldResult {
	Selected,
	Changed,
}

pub struct TextFieldBuilder<'a> {
	pub disabled: bool,
	pub x: SizeInCharacters,
	pub y: SizeInCharacters,
	pub w: SizeInCharacters,
	pub value: Value<'a>,
	pub default_text: &'a str,
	pub label: &'a str,
	pub layer: &'a mut base::Layer,
	pub value_color: sdl2::pixels::Color,
	pub label_color: sdl2::pixels::Color,
	pub bold: bool,
}

pub struct State {
	pub cursor_pos: i32,
	pub cursor_visible: bool,
	pub cursor_visibility_change_tick: uint,
	pub value: String,
}

impl State {
	pub fn new(value: &Value) -> State {
		let value = value.to_string();
		State {
			cursor_pos: value.len() as i32 ,
			cursor_visibility_change_tick: 0,
			cursor_visible: false,
			value: value
		}
	}
}

pub enum Value<'a> {
	Text(&'a mut String),
	F32(&'a mut f32),
	I32(&'a mut i32),
}

pub fn textfield_f32<'a>(layer: &'a mut base::Layer, ptr: &'a mut f32, w: SizeInCharacters)-> TextFieldBuilder<'a> {
	TextFieldBuilder::new(layer, F32(ptr), w)
}

pub fn textfield_i32<'a>(layer: &'a mut base::Layer, ptr: &'a mut i32, w: SizeInCharacters)-> TextFieldBuilder<'a> {
	TextFieldBuilder::new(layer, I32(ptr), w)
}

pub fn textfield_str<'a>(layer: &'a mut base::Layer, ptr: &'a mut String, w: SizeInCharacters)-> TextFieldBuilder<'a> {
	TextFieldBuilder::new(layer, Text(ptr), w)
}

impl <'a> Value<'a> {
	pub fn get_id(&self) -> i32 {
		match self {
			&Text(ref ptr) => *ptr as *mut String as i32,
			&F32(ref ptr) => *ptr as *mut f32 as i32,
			&I32(ref ptr) => *ptr as *mut i32 as i32,
		}
	}

	pub fn to_string(&self) -> String {
		match self {
			&Text(ref ptr) => ptr.to_string(),
			&F32(ref ptr) => format!("{: >.1f}", **ptr),
			&I32(ref ptr) => ptr.to_string(),
		}
	}
}


impl<'a> TextFieldBuilder<'a> {
	pub fn new(layer: &'a mut base::Layer, value: Value<'a>, w: SizeInCharacters)-> TextFieldBuilder<'a> {
		layer.add_textfield_state(value.get_id(), State::new(&value));
		TextFieldBuilder {
			disabled: false,
			x: layer.last_x,
			y: layer.last_y,
			w: w,
			layer: layer,
			default_text: "",
			label: "",
			value: value,
			value_color: RGB(204, 204, 204),
			label_color: RGB(221, 221, 221),
			bold: false,
		}
	}

	pub fn disabled(mut self, v: bool) -> TextFieldBuilder<'a> {self.disabled = v; self}
	pub fn default_text(mut self, v: &'a str) -> TextFieldBuilder<'a> {self.default_text = v; self}
	pub fn label(mut self, v: &'a str) -> TextFieldBuilder<'a> {self.label = v; self}
	pub fn label_color(mut self, v: sdl2::pixels::Color) -> TextFieldBuilder<'a> {self.label_color = v; self}
	pub fn value_color(mut self, v: sdl2::pixels::Color) -> TextFieldBuilder<'a> {self.value_color = v; self}
	pub fn bold(mut self, v: bool) -> TextFieldBuilder<'a> {self.bold = v; self}
	pub fn x(mut self, v: SizeInCharacters) -> TextFieldBuilder<'a> {self.x = v; self}
	pub fn y(mut self, v: SizeInCharacters) -> TextFieldBuilder<'a> {self.y = v; self}

	pub fn right(mut self, x: SizeInCharacters) -> TextFieldBuilder<'a> {
		self.x = self.layer.last_x + self.layer.last_w + x;
		self
	}

	pub fn inner_right(mut self, x: SizeInCharacters) -> TextFieldBuilder<'a> {
		self.x = self.layer.last_x + x;
		self
	}

	pub fn down(mut self, y: SizeInCharacters) -> TextFieldBuilder<'a> {
		self.y = self.layer.last_y + self.layer.last_h + y;
		self
	}

	pub fn up(mut self, y: SizeInCharacters) -> TextFieldBuilder<'a> {
		self.y = self.layer.last_y - y;
		self
	}

	pub fn inner_down(mut self, y: SizeInCharacters) -> TextFieldBuilder<'a> {
		self.y = self.layer.last_y + y;
		self
	}


	pub fn draw(&mut self, renderer: &sdl2::render::Renderer) -> Option<TextFieldResult> {
		draw(self, renderer)
	}
}


pub fn draw_bg(builder: &mut TextFieldBuilder, renderer: &sdl2::render::Renderer) {
	let char_w = builder.layer.char_w;
	let char_h = builder.layer.char_h;
	let label_width = builder.label.len() as i32  * char_w;
	let x = builder.x.in_pixels(char_w);
	let y = builder.y.in_pixels(char_h);
	let w = builder.w.in_pixels(char_w);
	let h = char_h;

	let hover = builder.layer.is_mouse_in(x, y, label_width+w, h);
	let was_active = builder.layer.is_active_widget(x, y);
	let clicked_out = builder.layer.is_mouse_down() && !hover && was_active;
	let active = was_active && !clicked_out;
	if hover || active {
		builder.layer.draw_rect_gradient(label_width+x, y, w, h, RGB(51, 51, 51), RGB(61, 61, 61));
	} else {
		builder.layer.draw_rect_gradient(label_width+x, y, w, h, RGB(51, 51, 51), RGB(51, 51, 51));
	}
}

pub fn draw_text(builder: &mut TextFieldBuilder, renderer: &sdl2::render::Renderer) {
	let char_w = builder.layer.char_w;
	let char_h = builder.layer.char_h;
	let label_width = builder.label.len() as i32  * char_w;
	let x = builder.x.in_pixels(char_w);
	let y = builder.y.in_pixels(char_h);
	let w = builder.w.in_pixels(char_w);
	let h = char_h;

	let hover = builder.layer.is_mouse_in(x, y, label_width+w, h);
	let was_active = builder.layer.is_active_widget(x, y);
	let clicked_out = builder.layer.is_mouse_down() && !hover && was_active;
	let active = was_active && !clicked_out;
	let border_width = 2;

	let output_value = builder.layer.get_textfield_state(builder.value.get_id()).value.clone();
	if output_value.len() > 0 {
		if builder.bold {
			builder.layer.draw_bold_text(label_width+x+border_width, y, output_value.as_slice(), builder.value_color);
		} else {
			builder.layer.draw_text(label_width+x+border_width, y, output_value.as_slice(), builder.value_color);
		}
	} else if builder.default_text != "" && !active {
		builder.layer.draw_text(label_width+x+border_width, y, builder.default_text.as_slice(), RGB(113, 113, 113));
	}

	if label_width > 0 {
		builder.layer.draw_text(x+border_width, y, builder.label.as_slice(), builder.label_color);
	}
	if active {
		let cursor_pos = builder.layer.get_textfield_state(builder.value.get_id()).cursor_pos;
		let cursor_visible = builder.layer.get_textfield_state(builder.value.get_id()).cursor_visible;
		if cursor_visible {
			builder.layer.draw_text(label_width+x + char_w as i32 * cursor_pos, y, "_", RGB(204, 204, 204));
		}
	}
}

pub fn draw_border(builder: &TextFieldBuilder, renderer: &sdl2::render::Renderer) {
	let char_w = builder.layer.char_w;
	let char_h = builder.layer.char_h;
	let label_width = builder.label.len() as i32  * char_w;
	let x = builder.x.in_pixels(char_w);
	let y = builder.y.in_pixels(char_h);
	let w = builder.w.in_pixels(char_w);
	let h = char_h;

	let border_width = 2;
	base::draw_rect(renderer, label_width+x, y, w+border_width, h+border_width, 2, RGB(0, 0, 0));
}

pub fn handle_logic(builder: &mut TextFieldBuilder) -> Option<TextFieldResult> {
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

	let mut edited = false;
	let output_value = if active {
		let maybe_char = builder.layer.input_char();
		let control_keys = builder.layer.control_keys;
		let state = builder.layer.get_mut_textfield_state(builder.value.get_id());
		let text_len = state.value.as_slice().chars().count() as i32;


		state.cursor_pos = ::std::cmp::min(state.cursor_pos, text_len);

		if state.cursor_pos > 0 && control_keys.backspace.just_pressed {
			let idx: uint = state.value.as_slice().graphemes(true).take(state.cursor_pos as uint - 1).map(|g| g.len()).sum();
			state.value.remove(idx);
			state.cursor_pos = state.cursor_pos-1;
			edited = true;
        } else if state.cursor_pos > 0 && control_keys.left.just_pressed {
        	state.cursor_pos = state.cursor_pos-1;
        } else if state.cursor_pos < text_len && control_keys.right.just_pressed {
        	state.cursor_pos = state.cursor_pos+1;
        } else if state.cursor_pos < text_len && control_keys.del.just_pressed {
        	let idx: uint = state.value.as_slice().graphemes(true).take(state.cursor_pos as uint).map(|g| g.len()).sum();
			state.value.remove(idx);
			edited = true;
        } else {
			if maybe_char.is_some() {
				let ch = maybe_char.unwrap();
				//
				let idx: uint = state.value.as_slice().graphemes(true).take(state.cursor_pos as uint).map(|g| g.len()).sum();
				state.value.insert(idx, ch);
				state.cursor_pos = state.cursor_pos+1;
				edited = true;
			}
		}
		state.value.clone()
	} else {
		builder.layer.get_textfield_state(builder.value.get_id()).value.clone()
	};

	if active && builder.layer.control_keys.tab.just_pressed {
		builder.layer.clear_active_widget();
	} else if !builder.layer.is_there_active_widget() {
		builder.layer.set_active_widget(x, y);
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



	if active {
		let tick = builder.layer.tick();
		let state = builder.layer.get_mut_textfield_state(builder.value.get_id());
		if state.cursor_visibility_change_tick < tick {
			state.cursor_visible = !state.cursor_visible;
			state.cursor_visibility_change_tick = tick + 500;
		}
	}
	if !edited {
		return if just_clicked { Some(Selected) } else {None};
	}
	match builder.value {
		Text(ref mut ptr) => {
			ptr.clear();
			ptr.push_str(output_value.as_slice());
			return Some(Changed);
		},
		I32(ref mut ptr) => {
			let maybe = from_str::<i32>(output_value.as_slice());
			if maybe.is_some() {
				**ptr = maybe.unwrap();
				return Some(Changed);
			}
		},
		F32(ref mut ptr) => {
			let maybe = from_str::<f32>(output_value.as_slice());
			if maybe.is_some() {
				**ptr = maybe.unwrap();
				return Some(Changed);
			}
		}
	}
	return None;
}

pub fn draw(builder: &mut TextFieldBuilder, renderer: &sdl2::render::Renderer) -> Option<TextFieldResult> {
	draw_bg(builder, renderer);

	draw_text(builder, renderer);
	draw_border(builder, renderer);

	handle_logic(builder)
}
