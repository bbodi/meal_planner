extern crate sdl2;
extern crate sdl2_ttf;

use std::cmp::max;

use sdl2::rect::Rect as SdlRect;
use sdl2::rect::Point;
use sdl2::pixels::RGB;
use std::collections::HashMap;
use std::collections::LruCache;

use textfield;

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

pub enum DrawCommand {
	Rect(i32, i32, i32, i32, sdl2::pixels::Color),
	Line(i32, i32, i32, i32, sdl2::pixels::Color),
	// x, y, bold, color
	Text(i32, i32, bool, String, sdl2::pixels::Color),
	GradientRect(i32, i32, i32, i32, sdl2::pixels::Color, sdl2::pixels::Color),
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
		}
	}
}

pub struct Layer {
	draw_commands: Vec<DrawCommand>,
	pub popup_layer: Option<Box<Layer>>,
	pub font: sdl2_ttf::Font,
	pub bfont: sdl2_ttf::Font,
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
	pub group_stack: Vec<(SizeInCharacters, SizeInCharacters, SizeInCharacters, SizeInCharacters)>,
	pub text_cache: LruCache<(u8, u8, u8, bool, u64), sdl2::render::Texture>,
	pub active: bool,

	// w, h, src_color, dst_color
	pub gradient_rect_cache: LruCache<(u32, u32, u8, u8, u8, u8, u8, u8), sdl2::render::Texture>,
}

impl Layer {

	pub fn new() -> Layer {
		let popup_layer = Layer::create_layer(None);
		Layer::create_layer(Some(box popup_layer))
	}

	fn create_layer(popup_layer: Option<Box<Layer>>) -> Layer {
		//DejaVuSansMono, Consolas
		let font = match sdl2_ttf::Font::from_file(&Path::new("ttf/DejaVuSansMono.ttf"), 16) {
        	Ok(f) => f,
        	Err(e) => panic!(e),
	    };
	    let (char_w, char_h) = match font.size_of_str("_") {
			Ok((w, h)) => (w, h),
			Err(e) => panic!(e),
		};
		let bfont = match sdl2_ttf::Font::from_file(&Path::new("ttf/DejaVuSansMono-Bold.ttf"), 16) {
        	Ok(f) => f,
        	Err(e) => panic!(e),
	    };
	    let (bchar_w, bchar_h) = match bfont.size_of_str("_") {
			Ok((w, h)) => (w, h),
			Err(e) => panic!(e),
		};
	    Layer {
	    	draw_commands: vec![],
	    	popup_layer: popup_layer,
	    	font: font,
	    	bfont: bfont,
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
	    	text_cache: LruCache::new(200),
	    	gradient_rect_cache: LruCache::new(200),
	    	group_stack: vec![],
	    	active: true,
	    }
	}

	pub fn clear_caches(&mut self) {
		self.text_cache.clear();
		self.gradient_rect_cache.clear();
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

	pub fn set_hot_widget(&mut self, x: i32, y: i32) {
		let id = x << 8 | y;
		self.hot_id = id;
	}

	pub fn set_active_widget_temporarily(&mut self, x: i32, y: i32) {
		self.last_active_id = self.active_id;
		let id = x << 8 | y;
		self.active_id = id;
	}

	pub fn is_hot_widget(&self, x: i32, y: i32) -> bool {
		let id = x << 8 | y;
		self.hot_id == id
	}

	pub fn clear_hot_widget(&mut self) {
		self.hot_id = NO_WIDGET_ID;
	}

	pub fn clear_active_widget(&mut self) {
		self.active_id = self.last_active_id;
		self.last_active_id = NO_WIDGET_ID;
	}

	pub fn set_active_widget(&mut self, x: i32, y: i32) {
		let id = x << 8 | y;
		self.active_id = id;
	}

	pub fn is_active_widget(&self, x: i32, y: i32) -> bool {
		let id = x << 8 | y;
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

    	match sdl_event {
    		&sdl2::event::WindowEvent(_, _, win_event_id, _, _) => {
    			match win_event_id {
                	sdl2::event::ResizedWindowEventId | sdl2::event::SizeChangedWindowEventId => self.clear_caches(),
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
    	let mut newvec: Vec<DrawCommand> = self.draw_commands.clone();
    	for draw_command in newvec.iter() {
    		match draw_command {
    			&Line(x1, y1, x2, y2, color) => {

    			},
    			&Rect(x, y, w, h, color) => {sdl_fill_rect(renderer, x, y, w, h, color);},
    			&GradientRect(x, y, w, h, color1, color2) => {self.do_draw_rect_gradient(renderer, x, y, w, h, color1, color2);},
    			&Text(x, y, bold, ref text, color) => {self.do_do_draw_text(renderer, x, y, bold, text.as_slice(), color);},
    		}
    	}
    	self.draw_commands.clear();
    }

	pub fn add_textfield_state(&mut self, id: i32, state: ::textfield::State) {
		if !self.textfield_datas.contains_key(&id) {
			self.textfield_datas.insert(id, state);
		}
	}

	pub fn draw_bold_text(&mut self, x: i32, y: i32, text: &str, color: sdl2::pixels::Color) {
		self.do_draw_text(x, y, true, text, color);
	}

	pub fn draw_text(&mut self, x: i32, y: i32, text: &str, color: sdl2::pixels::Color) {
		self.do_draw_text(x, y, false, text, color);
	}

	fn do_draw_text(&mut self, x: i32, y: i32, bold: bool, text: &str, color: sdl2::pixels::Color) {
		self.draw_commands.push(Text(x, y, bold, text.into_string(), color));
	}

	fn do_do_draw_text(&mut self, renderer: &sdl2::render::Renderer, x: i32, y: i32, bold: bool, text: &str, color: sdl2::pixels::Color) {
		let (text_w, text_h) = match self.font.size_of_str(text) {
			Ok((w, h)) => (w, h),
			Err(e) => panic!(e),
		};
		let (r, g, b) = color.get_rgb();
		let key = (r, g, b, bold, ::std::hash::hash(&text.into_string()));
		let has_cached_texture = self.text_cache.get(&key).is_some();
		if has_cached_texture {
			let cached_texture = self.text_cache.get(&key).unwrap();
			let _ = renderer.copy(cached_texture, None, Some(SdlRect::new(x, y, text_w as i32, text_h as i32)));
		} else {
			println!("MISS: {}", text);
			let created_texture = self.create_text_texture(renderer, bold, text, color);
			let _ = renderer.copy(&created_texture, None, Some(SdlRect::new(x, y, text_w as i32, text_h as i32)));
			self.text_cache.put(key, created_texture);
		}
	}

	fn create_text_texture(&self, renderer: &sdl2::render::Renderer, bold: bool, text: &str, color: sdl2::pixels::Color) -> sdl2::render::Texture {
		assert!(text.len() > 0, "create_text_texture was called with empty string!");
		let font = match bold {true => &self.bfont, false => &self.font};
	    let surface = match font.render_str_solid(text, color) {
	        Ok(s) => s,
	        Err(e) => panic!(e),
	    };
		match renderer.create_texture_from_surface(&surface) {
	        Ok(t) => t,
	        Err(e) => panic!(e),
	   	}
	}

	pub fn draw_rect_gradient(&mut self, x: i32, y: i32, w: i32, h: i32, start_color: sdl2::pixels::Color, end_color: sdl2::pixels::Color) {
		self.draw_commands.push(GradientRect(x, y, w, h, start_color, end_color));
	}

	fn do_draw_rect_gradient(&mut self, renderer: &sdl2::render::Renderer, x: i32, y: i32, w: i32, h: i32, start_color: sdl2::pixels::Color, end_color: sdl2::pixels::Color) {
		let (sr, sg, sb) = start_color.get_rgb();
		let (er, eg, eb) = end_color.get_rgb();
		let key = (w as u32, h as u32, sr, sg, sb, er, eg, eb);
		let has_cached_texture = self.gradient_rect_cache.get(&key).is_some();
		if has_cached_texture {
			let cached_texture = self.gradient_rect_cache.get(&key).unwrap();
			let _ = renderer.copy(cached_texture, None, Some(SdlRect::new(x, y, w, h)));
		} else {
			println!("MISS GRADIENT");
			let created_texture = create_gradient_texture(renderer, w, h, start_color, end_color);
			let _ = renderer.copy(&created_texture, None, Some(SdlRect::new(x, y, w, h)));
			self.gradient_rect_cache.put(key, created_texture);
		}
	}

	pub fn draw_rect_gradient1(&mut self, x: i32, y: i32, w: i32, h: i32, start_color: sdl2::pixels::Color) {
		let (sr, sg, sb) = start_color.get_rgb();
		let sr = sr as i32;
		let sg = sg as i32;
		let sb = sb as i32;
		self.draw_rect_gradient(x, y, w, h, start_color, RGB(max(0, sr-40) as u8, max(0, sg-40) as u8, max(0, sb-40) as u8))
	}

	pub fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: sdl2::pixels::Color) {
		self.draw_commands.push(Line(x1, y1, x2, y2, color));
	}

	pub fn fill_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: sdl2::pixels::Color) {
		self.draw_commands.push(Rect(x, y, w, h, color));
	}
}

pub fn draw_rect(renderer: &sdl2::render::Renderer, x: i32, y: i32, w: i32, h: i32, border: i32, color: sdl2::pixels::Color) {
	let _ = renderer.set_draw_color(color);
	for i in range(0, border) {
		let _ = renderer.draw_rect(&SdlRect::new(x+i, y+i, w-2*i, h-2*i));
	}
}

pub fn sdl_fill_rect(renderer: &sdl2::render::Renderer, x: i32, y: i32, w: i32, h: i32, color: sdl2::pixels::Color) {
	let _ = renderer.set_draw_color(color);
	let _ = renderer.fill_rect(&SdlRect::new(x, y, w, h));
}

pub fn draw_line(renderer: &sdl2::render::Renderer, x1: i32, y1: i32, x2: i32, y2: i32, color: sdl2::pixels::Color) {
	let _ = renderer.set_draw_color(color);
	let _ = renderer.draw_line(Point::new(x1, y1), Point::new(x2, y2));
}

fn create_gradient_texture(renderer: &sdl2::render::Renderer, w: i32, h: i32, start_color: sdl2::pixels::Color, end_color: sdl2::pixels::Color) -> sdl2::render::Texture {
	let texture =  match renderer.create_texture(sdl2::pixels::RGBA8888, sdl2::render::AccessTarget, w as int, h as int) {
		Ok(t) => t,
		Err(e) => panic!(e),
	};
	renderer.set_render_target(Some(&texture));
	for i in range(0, h) {
		let p = i as f32 / h as f32;
		let sp = 1f32 - p;
		let (start_r, start_g, start_b) = start_color.get_rgb();
		let (end_r, end_g, end_b) = end_color.get_rgb();
		let r = start_r as f32 * sp + end_r as f32 * p;
		let g = start_g as f32 * sp + end_g as f32 * p;
		let b = start_b as f32 * sp + end_b as f32 * p;
		let start = sdl2::rect::Point::new(0, i);
		let end = sdl2::rect::Point::new(w, i);
		let _ = renderer.set_draw_color(sdl2::pixels::RGB(r as u8, g as u8, b as u8));
		let _ = renderer.draw_line(start, end);
	}
	renderer.set_render_target(None);
	texture
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