extern crate sdl2;
extern crate sdl2_ttf;

use std::collections::HashMap;

use textfield;
use drawing_surf::DrawingSurface;

const NO_WIDGET_ID: i32 = 0x0FFFFFFF;

#[deriving(PartialEq, Clone, Show)]
pub struct SizeInCharacters(pub i32);

impl SizeInCharacters {
	pub fn in_pixels(&self, one_char_in_pixels: i32) -> i32 {
		let SizeInCharacters(x) = *self;
		x * one_char_in_pixels
	}
}

impl Add<SizeInCharacters, SizeInCharacters> for SizeInCharacters {
	fn add(&self, rhs: &SizeInCharacters) -> SizeInCharacters {
		let SizeInCharacters(s) = *self;
		let SizeInCharacters(rhs) = *rhs;
		SizeInCharacters(s + rhs)
	}
}

impl Sub<SizeInCharacters, SizeInCharacters> for SizeInCharacters {
	fn sub(&self, rhs: &SizeInCharacters) -> SizeInCharacters {
		let SizeInCharacters(s) = *self;
		let SizeInCharacters(rhs) = *rhs;
		SizeInCharacters(s - rhs)
	}
}

pub trait IndexValue {
	fn set(&mut self, value: uint);
	fn get(&self) -> uint;
}

impl IndexValue for i32 {
	fn set(&mut self, value: uint) {
		*self = value as i32;
	}
	fn get(&self) -> uint {
		*self as uint
	}
}

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
	pub tab: Key,
	pub ctrl: Key,
	pub alt: Key,
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
			tab: Key::new(),
			ctrl: Key::new(),
			alt: Key::new(),
		}
	}
}

pub struct Layer {
	pub bottom_surface: DrawingSurface,
	pub top_surface: DrawingSurface,
	pub middle_surface: DrawingSurface,
	active_id: i32,
	hot_id: i32,
	last_active_id: i32,
	mouse_x: i32,
	mouse_y: i32,
	mouse_state: i32,
	prev_mouse_state: i32,
	pub last_mouse_x: i32,
	pub last_mouse_y: i32,
	tick: uint,
	text_input: String,
	textfield_datas: HashMap<i32, textfield::State>,
	pub control_keys: ControlKeys,
	pub char_w: i32,
	pub char_h: i32,
	pub bchar_w: i32,
	pub bchar_h: i32,
	pub last_x: SizeInCharacters,
	pub last_y: SizeInCharacters,
	pub last_w: SizeInCharacters,
	pub last_h: SizeInCharacters,
	
	pub active: bool,
}

impl Layer {

	pub fn new() -> Layer {
		//DejaVuSansMono, Consolas
		let font = match sdl2_ttf::Font::from_file(&Path::new("ttf/DejaVuSansMono.ttf"), 14) {
        	Ok(f) => f,
        	Err(e) => panic!(e),
	    };
	    let font2 = match sdl2_ttf::Font::from_file(&Path::new("ttf/DejaVuSansMono.ttf"), 14) {
        	Ok(f) => f,
        	Err(e) => panic!(e),
	    };
	    let font3 = match sdl2_ttf::Font::from_file(&Path::new("ttf/DejaVuSansMono.ttf"), 14) {
        	Ok(f) => f,
        	Err(e) => panic!(e),
	    };
	    let (char_w, char_h) = match font.size_of_str("_") {
			Ok((w, h)) => (w, h),
			Err(e) => panic!(e),
		};
		let bfont = match sdl2_ttf::Font::from_file(&Path::new("ttf/DejaVuSansMono-Bold.ttf"), 14) {
        	Ok(f) => f,
        	Err(e) => panic!(e),
	    };
	    let bfont2 = match sdl2_ttf::Font::from_file(&Path::new("ttf/DejaVuSansMono-Bold.ttf"), 14) {
        	Ok(f) => f,
        	Err(e) => panic!(e),
	    };
	    let bfont3 = match sdl2_ttf::Font::from_file(&Path::new("ttf/DejaVuSansMono-Bold.ttf"), 14) {
        	Ok(f) => f,
        	Err(e) => panic!(e),
	    };
	    let (bchar_w, bchar_h) = match bfont.size_of_str("_") {
			Ok((w, h)) => (w, h),
			Err(e) => panic!(e),
		};
	    Layer {
	    	bottom_surface: DrawingSurface::new(font, bfont),
	    	top_surface: DrawingSurface::new(font2, bfont2),
	    	middle_surface: DrawingSurface::new(font3, bfont3),
	    	active_id: NO_WIDGET_ID,
	    	hot_id: NO_WIDGET_ID,
	    	last_active_id: NO_WIDGET_ID,
	    	mouse_x: 0,
	    	mouse_y: 0,
	    	mouse_state: 0,
	    	prev_mouse_state: 0,
	    	last_mouse_x: 0,
			last_mouse_y: 0,
	    	textfield_datas: HashMap::new(),
	    	tick: 0,
	    	text_input: "".into_string(),
	    	control_keys: ControlKeys::new(),
	    	char_w: char_w as i32,
	    	char_h: char_h as i32,
	    	bchar_w: bchar_w as i32,
	    	bchar_h: bchar_h as i32,
	    	last_x: SizeInCharacters(0),
	    	last_y: SizeInCharacters(0),
	    	last_w: SizeInCharacters(0),
	    	last_h: SizeInCharacters(0),
	    	active: true,
	    }
	}

	pub fn get_mut_textfield_state(&mut self, id: i32) -> &mut textfield::State {
		self.textfield_datas.get_mut(&id)
	}

	pub fn get_textfield_state(&self, id: i32) -> &textfield::State {
		match self.textfield_datas.find(&id)  {
			Some(d) => d,
			None => panic!(),
		}
	}

	pub fn clear_textfield_states(&mut self) {
		self.textfield_datas.clear();
	}

	pub fn start_group(func: ||) {
		func();
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

	pub fn set_hot_widget(&mut self, id: i32) {
		self.hot_id = id;
	}

	pub fn set_active_widget_temporarily(&mut self, id: i32) {
		self.last_active_id = self.active_id;
		self.active_id = id;
	}

	pub fn is_hot_widget(&self, id: i32) -> bool {
		self.hot_id == id
	}

	pub fn clear_hot_widget(&mut self) {
		self.hot_id = NO_WIDGET_ID;
	}

	pub fn clear_active_widget(&mut self) {
		self.active_id = self.last_active_id;
		self.last_active_id = NO_WIDGET_ID;
	}

	pub fn set_active_widget(&mut self, id: i32) {
		self.active_id = id;
	}

	pub fn is_active_widget(&self, id: i32) -> bool {
		self.active_id == id
	}

	pub fn is_there_active_widget(&self) -> bool {
		self.active_id != NO_WIDGET_ID
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

	pub fn handle_event(&mut self, sdl_event: &sdl2::event::Event) {
		self.last_x = SizeInCharacters(0);
		self.last_y = SizeInCharacters(0);
		self.last_w = SizeInCharacters(0);
		self.last_h = SizeInCharacters(0);
		self.last_mouse_x = self.mouse_x;
		self.last_mouse_y = self.mouse_y;
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
		Layer::update_key(keys[sdl2::scancode::TabScanCode], &mut self.control_keys.tab);
		Layer::update_key(keys[sdl2::scancode::LCtrlScanCode], &mut self.control_keys.ctrl);
		Layer::update_key(keys[sdl2::scancode::LAltScanCode], &mut self.control_keys.alt);

    	match sdl_event {
    		&sdl2::event::WindowEvent(_, _, win_event_id, _, _) => {
    			match win_event_id {
                	sdl2::event::ResizedWindowEventId | sdl2::event::SizeChangedWindowEventId => {
                		self.bottom_surface.window_resized();
                		self.middle_surface.window_resized();
                		self.top_surface.window_resized();
                	}
                	sdl2::event::FocusLostWindowEventId => self.active = false,
                	sdl2::event::FocusGainedWindowEventId => self.active = true,
                	_=> {},
            	}
            }
			// (timestamp, window, which, [MouseState], x, y, xrel, yrel)
            &sdl2::event::MouseMotionEvent(_, _, _, _, x, y, _, _) => {
            	self.mouse_x = x as i32;
            	self.mouse_y = y as i32;
            },
            /// (timestamp, window, which, MouseBtn, x, y)
    		&sdl2::event::MouseButtonDownEvent(_, _, _, _, x, y) => {
    			self.mouse_x = x as i32;
            	self.mouse_y = y as i32;
            	self.mouse_state = 1;
    		},
    		&sdl2::event::MouseButtonUpEvent(_, _, _, _, x, y) => {
    			self.mouse_x = x as i32;
            	self.mouse_y = y as i32;
            	self.mouse_state = 0;
    		},
    		&sdl2::event::TextInputEvent(_, _, ref text) => {
    			self.text_input = text.clone();
    		}
            _ => {}
        };
    }

    pub fn draw(&mut self, renderer: &sdl2::render::Renderer) {
    	self.bottom_surface.draw(renderer);
    	self.middle_surface.draw(renderer);
    	self.top_surface.draw(renderer);
    	self.bottom_surface.clear();
    	self.middle_surface.clear();
    	self.top_surface.clear();
    }

	pub fn add_textfield_state(&mut self, id: i32, state: ::textfield::State) {
		if !self.textfield_datas.contains_key(&id) {
			self.textfield_datas.insert(id, state);
		}
	}
}

pub fn center_text(text: &str, char_w: i32, area_width: i32) -> i32 {
	area_width/2 - (text.len() as i32)/2 * char_w
}

pub fn center_text_in_chars(text: &str, area_width: i32) -> SizeInCharacters {
	SizeInCharacters(area_width/2 - (text.len() as i32)/2)
}

pub fn text_len(text: &str) -> uint {
	text.graphemes(true).count()
}
