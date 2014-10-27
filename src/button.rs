extern crate sdl2;
extern crate sdl2_ttf;

use std::collections::RingBuf;
use std::collections::Deque;
use std::cmp::min;
use std::cmp::max;

use sdl2::pixels::RGB;
use sdl2::rect::Rect;
use sdl2::rect::Point;

use widget;
use widget::WidgetImpl;
use widget::EventHandlingResult;
use widget::WidgetEvent;
use widget::WidgetPos;


pub struct Button {
	pub label: String,
	font: sdl2_ttf::Font,
	pub hover: bool,
	pub down: bool,
}

impl WidgetImpl for Button {

    fn handle_event(&mut self, event: WidgetEvent) -> EventHandlingResult {
    	match event {
    		widget::WidgetMouseMoveEvent(pos) => {
    			let WidgetPos(x, _) = pos;
    			self.hover = true;
    			return widget::HANDLED | widget::NEED_REDRAW;
    		},
    		widget::WidgetMouseClickEvent(pos) => {
    			let WidgetPos(x, _) = pos;
    			self.down = true;
    			return widget::HANDLED | widget::NEED_REDRAW;
    		},
    	}
    	return widget::NONE;
    }

    fn draw(&self, renderer: &sdl2::render::Renderer, w: u32, h: u32) {
    	renderer.set_draw_color(sdl2::pixels::RGB(32 , 32, 32));
		//renderer.fill_rect(&sdl2::rect::Rect::new(0, 0, self.width as i32, self.height as i32));
		if self.hover {
			widget::draw_rect_gradient(renderer, 0, 0, w, h, RGB(114, 114, 114), RGB(68, 68, 68));
		} else {
			widget::draw_rect_gradient(renderer, 0, 0, w, h, RGB(93, 93, 93), RGB(44, 44, 44));
		}
		let texure = widget::create_text_texture(renderer, &self.font, self.label.as_slice(), RGB(151, 151, 151));
		renderer.copy(&texure, None, None);
		renderer.set_draw_color(sdl2::pixels::RGB(33, 33, 33));
		renderer.draw_line(Point::new(0, 2), Point::new(0, (h-2) as i32 ) );
		renderer.draw_line(Point::new((w-1) as i32, 2), Point::new((w-1) as i32, (h-2) as i32) );

		renderer.set_draw_color(sdl2::pixels::RGB(33, 33, 33));
		renderer.draw_line(Point::new(2, 0), Point::new((w-3) as i32, 0) );
		renderer.draw_line(Point::new(0, 1), Point::new((w-1) as i32, 1) );

		renderer.draw_line(Point::new(0, (h-2) as i32 ), Point::new((w-1) as i32, (h-2) as i32) );
		renderer.draw_line(Point::new(2, (h-1) as i32 ) , Point::new((w-3) as i32, (h-1) as i32) );
    }
}

impl Button {
	pub fn new(label: &str) -> Button {
		let font = match sdl2_ttf::Font::from_file(&Path::new("DejaVuSansMono.ttf"), 12) {
        	Ok(f) => f,
        	Err(e) => fail!(e),	
	    };
	    
		Button {
			label: label.into_string(),
			font: font,
			hover: false,
			down: false,
		}
	}
}
