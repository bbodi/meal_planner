extern crate sdl2;
extern crate sdl2_ttf;

use sdl2::rect::Rect;
use std::collections::HashMap;

use button;
use line_chart;
use textfield;
use checkbox;
use dropdown;

#[deriving(PartialEq, Clone, Show)]
pub struct Key {
	pub down: bool,
	pub just_pressed: bool,
	pub just_released: bool
}

impl Key {
	pub fn new() -> Key {
		Key {
			down: false,
			just_pressed: false,
			just_released: false,
		}
	}
}

#[deriving(PartialEq, Clone, Show)]
pub struct ControlKeys {
	pub left: Key,
	pub right: Key,
	pub up: Key,
	pub down: Key,
	pub backspace: Key,
	pub del: Key,
	pub home: Key,
	pub end: Key,
	pub enter: Key,
}

impl ControlKeys {
	pub fn new() -> ControlKeys {
		ControlKeys {
			left: Key::new(),
			right: Key::new(),
			up: Key::new(),
			down: Key::new(),
			backspace: Key::new(),
			del: Key::new(),
			home: Key::new(),
			end: Key::new(),
			enter: Key::new(),
		}
	}
}

pub struct Layer {
	pub font: sdl2_ttf::Font,
	active_id: i32,
	hot_id: i32,
	mouse_x: i32,
	mouse_y: i32,
	mouse_state: i32,
	prev_mouse_state: i32,
	tick: uint,
	text_input: String,
	textfield_datas: HashMap<i32, textfield::State>,
	pub control_keys: ControlKeys,
	pub char_w: i32,
	pub char_h: i32,
}

impl Layer {

	pub fn new() -> Layer {
		let font = match sdl2_ttf::Font::from_file(&Path::new("DejaVuSansMono.ttf"), 16) {
        	Ok(f) => f,
        	Err(e) => fail!(e),	
	    };
	    let (char_w, char_h) = match font.size_of_str("_") {
			Ok((w, h)) => (w, h),
			Err(e) => fail!("e"),
		};
	    Layer {
	    	font: font,
	    	active_id: 0xFFFFFFFF,
	    	hot_id: 0xFFFFFFFF,
	    	mouse_x: 0,
	    	mouse_y: 0,
	    	mouse_state: 0,
	    	prev_mouse_state: 0,
	    	textfield_datas: HashMap::new(),
	    	tick: 0,
	    	text_input: "".into_string(),
	    	control_keys: ControlKeys::new(),
	    	char_w: char_w as i32,
	    	char_h: char_h as i32,
	    }
	}

	pub fn get_mut_textfield_state(&mut self, x: i32, y: i32) -> &mut textfield::State {
		let id = x << 8 | y;
		self.textfield_datas.get_mut(&id) 
	}

	pub fn get_textfield_state(&self, x: i32, y: i32) -> &textfield::State {
		let id = x << 8 | y;
		match self.textfield_datas.find(&id)  {
			Some(d) => d,
			None => fail!(),
		}
	}

	pub fn is_mouse_in(&self, x: i32, y: i32, w: i32, h: i32) -> bool {
		let mx = self.mouse_x;
		let my = self.mouse_y;
		mx >= x && mx < (x+w) && my >= y && my < (y+h)
	}

	pub fn is_mouse_down(&self) -> bool {
		self.mouse_state == 1
	}

	pub fn is_mouse_pressed(&self) -> bool {
		self.mouse_state == 1 && self.prev_mouse_state == 0
	}

	pub fn is_mouse_released(&self) -> bool {
		self.mouse_state == 0 && self.prev_mouse_state == 1
	}

	pub fn set_hot_widget(&mut self, x: i32, y: i32) {
		unsafe {
			let id = x << 8 | y;
			self.hot_id = id;
		}
	}

	pub fn is_hot_widget(&self, x: i32, y: i32) -> bool {
		unsafe {
			let id = x << 8 | y;
			self.hot_id == id
		}
	}

	pub fn clear_hot_widget(&mut self) {
		self.hot_id = 0xFFFFFFFF;
	}

	pub fn clear_active_widget(&mut self) {
		self.active_id = 0xFFFFFFFF;
	}

	pub fn set_active_widget(&mut self, x: i32, y: i32) {
		unsafe {
			let id = x << 8 | y;
			self.active_id = id;
		}
	}

	pub fn is_active_widget(&self, x: i32, y: i32) -> bool {
		unsafe {
			let id = x << 8 | y;
			self.active_id == id
		}
	}

	pub fn mouse_x(&self) -> i32 {
		self.mouse_x
	}

	pub fn mouse_y(&self) -> i32 {
		self.mouse_y
	}

	pub fn tick(&self) -> uint {
		self.tick
	}

	pub fn input_char(&mut self) -> Option<char> {
		self.text_input.pop()
	}

	fn update_key(down: bool, key: &mut Key) {
		if down && !key.down {
			key.just_pressed = true;
			key.just_released = false;
			key.down = true;
		} else if !down && key.down {
			key.just_released = true;
			key.just_pressed = false;
			key.down = false;
		} else {
			key.just_released = false;
			key.just_pressed = false;
			key.down = down;
		}
	}

	pub fn handle_event(&mut self, sdl_event: sdl2::event::Event) {
		self.text_input = "".into_string();
		self.prev_mouse_state = self.mouse_state;
		self.tick = sdl2::timer::get_ticks();
		let keys = sdl2::keyboard::get_keyboard_state();

		Layer::update_key(keys[sdl2::scancode::BackspaceScanCode], &mut self.control_keys.backspace);
		Layer::update_key(keys[sdl2::scancode::LeftScanCode], &mut self.control_keys.left);
		Layer::update_key(keys[sdl2::scancode::RightScanCode], &mut self.control_keys.right);
		Layer::update_key(keys[sdl2::scancode::DeleteScanCode], &mut self.control_keys.del);
		Layer::update_key(keys[sdl2::scancode::ReturnScanCode], &mut self.control_keys.enter);
		Layer::update_key(keys[sdl2::scancode::HomeScanCode], &mut self.control_keys.home);
		Layer::update_key(keys[sdl2::scancode::EndScanCode], &mut self.control_keys.end);
	    
		
    	match sdl_event {
        	// /// (timestamp, window, winEventId, data1, data2)
			sdl2::event::WindowEvent(_, _, winEventId, data1, data2) => {
				match winEventId {
					sdl2::event::ResizedWindowEventId => {
						//self.set_window_size(data1 as i32, data2 as i32);
					}
					_ => {}
				}
			},
			// (timestamp, window, which, [MouseState], x, y, xrel, yrel)
            sdl2::event::MouseMotionEvent(_, _, _, _, x, y, _, _) => {
            	self.mouse_x = x as i32;
            	self.mouse_y = y as i32;
            },
            /// (timestamp, window, which, MouseBtn, x, y)
    		sdl2::event::MouseButtonDownEvent(_, _, _, _, x, y) => {
    			self.mouse_x = x as i32;
            	self.mouse_y = y as i32;
            	self.mouse_state = 1;	
    		},
    		sdl2::event::MouseButtonUpEvent(_, _, _, _, x, y) => {
    			self.mouse_x = x as i32;
            	self.mouse_y = y as i32;
            	self.mouse_state = 0;	
    		},
    		sdl2::event::TextInputEvent(_, _, text) => {
    			self.text_input = text;
    		}
            _ => {}
        };
    }

    pub fn button<'a>(&'a mut self, label: &'a str, x: i32, y: i32, w: i32, h: i32) -> button::ButtonBuilder<'a> {
		button::ButtonBuilder::new(self, label, x, y, w, h)
	}

	pub fn textfield<'a>(&'a mut self, text: &'a mut String, x: i32, y: i32, w: i32, h: i32) -> textfield::TextFieldBuilder<'a> {
		let id = x << 8 | y;
		if !self.textfield_datas.contains_key(&id) {
			self.textfield_datas.insert(id, textfield::State::new(text.as_slice()));
		}
		
		textfield::TextFieldBuilder::new(self, text, x, y, w, h)
	}

	pub fn line_chart<'a>(&'a mut self, label: &'a str, x: i32, y: i32, w: i32, h: i32) -> line_chart::LineChartBuilder<'a> {
		line_chart::LineChartBuilder::new(self, label, x, y, w, h)
	}

	pub fn checkbox<'a>(&'a mut self, label: &'a str, value: &'a mut bool, x: i32, y: i32) -> checkbox::CheckboxBuilder<'a> {
		checkbox::CheckboxBuilder::new(self, label, value, x, y)
	}

	pub fn dropdown<'a>(&'a mut self, labels: &'a [&'a str], value: &'a mut uint, x: i32, y: i32) -> dropdown::DropdownBuilder<'a> {
		dropdown::DropdownBuilder::new(self, labels, value, x, y)
	}
}

pub fn draw_rect_gradient(renderer: &sdl2::render::Renderer, x: i32, y: i32, w: i32, h: i32, start_color: sdl2::pixels::Color, end_color: sdl2::pixels::Color) {
	for i in range(0, h) {
		let p = i as f32 / h as f32;
		let sp = 1f32 - p;
		let (start_r, start_g, start_b) = start_color.get_rgb();
		let (end_r, end_g, end_b) = end_color.get_rgb();
		let mut r = start_r as f32 * sp + end_r as f32 * p;
		let mut g = start_g as f32 * sp + end_g as f32 * p;
		let mut b = start_b as f32 * sp + end_b as f32 * p;
		let start = sdl2::rect::Point::new(x, y+i);
		let end = sdl2::rect::Point::new(x+w, y+i);
		renderer.set_draw_color(sdl2::pixels::RGBA(r as u8, g as u8, b as u8, 150));
		renderer.draw_line(start, end);
	}
}

pub fn draw_text(x: i32, y: i32, renderer: &sdl2::render::Renderer, font: &sdl2_ttf::Font, text: &str, color: sdl2::pixels::Color) {
	let (text_w, text_h) = match font.size_of_str(text) {
		Ok((w, h)) => (w, h),
		Err(e) => fail!("e"),
	};
	let texure = create_text_texture(renderer, font, text, color);
	renderer.copy(&texure, None, Some(Rect::new(x, y, text_w as i32, text_h as i32)));
}

pub fn create_text_texture(renderer: &sdl2::render::Renderer, font: &sdl2_ttf::Font, text: &str, color: sdl2::pixels::Color) -> sdl2::render::Texture {
	// render a surface, and convert it to a texture bound to the renderer
    let surface = match font.render_str_blended(text, color) {
        Ok(s) => s,
        Err(e) => fail!(e),
    };
	match renderer.create_texture_from_surface(&surface) {
        Ok(t) => t,
        Err(e) => fail!(e),
   	}
}