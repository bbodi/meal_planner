extern crate sdl2;
extern crate sdl2_ttf;

use sdl2::rect::Rect;

struct WindowPos(u32, u32);
struct LayerPos(u32, u32);
pub struct WidgetPos(pub u32, pub u32);


pub fn win_to_layer_pos(pos: WindowPos, layer: &Layer) -> LayerPos {
	let WindowPos(x, y) = pos;
	LayerPos((x as f32 / layer.window_to_layer_ratio_w) as u32, (y as f32 / layer.window_to_layer_ratio_h) as u32)
}

pub fn layer_to_widget_pos(pos: LayerPos, rect: &Rect) -> WidgetPos {
	let LayerPos(x, y) = pos;
	WidgetPos(x - rect.x as u32, y - rect.y as u32)
}

enum LayerEvent {
	LayerMouseMoveEvent(LayerPos),
	LayerMouseClickEvent(LayerPos),
}

pub enum WidgetEvent {
	WidgetMouseMoveEvent(WidgetPos),
	WidgetMouseClickEvent(WidgetPos),
}

struct WidgetLayerInfo<'a> {
	rect: sdl2::rect::Rect,
	widget: Box<WidgetImpl + 'a>,
	need_redraw: bool,
}

pub struct Layer<'a> {
	width: u32, 
	height: u32,
	window_to_layer_ratio_w: f32,
	window_to_layer_ratio_h: f32,
    pub texture: sdl2::render::Texture,
    widgets: Vec<WidgetLayerInfo<'a>>
}


impl<'a> Layer<'a> {

	pub fn new(renderer: &sdl2::render::Renderer, width: u32, height: u32) -> Layer {
		let texture =  match renderer.create_texture(sdl2::pixels::RGBA8888, sdl2::render::AccessTarget, width as int, height as int) {
			Ok(t) => t,
			Err(e) => fail!(e),
		};
		renderer.set_render_target(Some(&texture));
		renderer.set_draw_color(sdl2::pixels::RGB(47 , 47, 47));
        renderer.clear();
        renderer.set_render_target(None);
		Layer {
			width: width,
			height: height,
			widgets: vec![],
			window_to_layer_ratio_w: 1f32,
			window_to_layer_ratio_h: 1f32,
			texture: texture, 
		}
	}

	pub fn add_widget(&mut self, w: Box<WidgetImpl + 'a>, rect: sdl2::rect::Rect) {
		self.widgets.push(WidgetLayerInfo {
			rect: rect,
			need_redraw: true,
			widget: w,
		});
	}

	pub fn draw(&mut self, renderer: &sdl2::render::Renderer) {
		renderer.set_render_target(Some(&self.texture));
        let mut was_draw = false;
		for widget_info in self.widgets.mut_iter() {
			if !widget_info.need_redraw {
				continue;
			}
			renderer.set_viewport(Some(widget_info.rect));
			widget_info.widget.draw(renderer, widget_info.rect.w as u32, widget_info.rect.h as u32);
			widget_info.need_redraw = false;
			was_draw = true;
		}
		renderer.set_render_target(None);
		if was_draw {
			renderer.set_viewport(None);
    	}
    	renderer.copy(&self.texture, None, None);
    }

	// ClickOutsideEvent
	// draw(widget) // csak egyet rajzol ki
	// nem kell render_taget, se texture
	// handle_system_event(widget, e); - csak ezután jöhetnek a widgetek, ha szükséges
	// handle_event(widget, e);
	// hot = hover
	// active = clicked
	// {
	// 	if checkbox(x, y, &mut name, &mut value) {
	//		ez elég fun lenne
	// az átadott ptr lenne az ID.
	//	}
	// }
    pub fn draw2(&mut self, renderer: &sdl2::render::Renderer, widgets: &Vec<(&WidgetImpl, Rect)>) {
		renderer.set_render_target(Some(&self.texture));
        let mut was_draw = false;
		for i in range(0, widgets.len()) {
			//if !widget_info.need_redraw {
			//	continue;
			//}
			let (w, r) = widgets[i];
			renderer.set_viewport(Some(r));
			w.draw(renderer, r.w as u32, r.h as u32);
			//widget_info.need_redraw = false;
			was_draw = true;
		}
		renderer.set_render_target(None);
		if was_draw {
			renderer.set_viewport(None);
    	}
    	renderer.copy(&self.texture, None, None);
    }

    pub fn set_window_size(&mut self, w: u32, h: u32) {
    	self.window_to_layer_ratio_w = w as f32 / self.width as f32;
    	self.window_to_layer_ratio_h = h as f32 / self.height as f32;
    }

    fn align_to_window_with(&self, num: int) -> u32 {
    	(num as f32 / self.window_to_layer_ratio_w) as u32
    }

    fn align_to_window_height(&self, num: int) -> u32 {
    	(num as f32 / self.window_to_layer_ratio_h) as u32 
    }

    fn make_local_event(event: LayerEvent, rect: &Rect) -> Option<WidgetEvent> {
    	match event {
    		LayerMouseMoveEvent(layer_pos) => {
    			let LayerPos(x, y) = layer_pos;
    			let x = x as i32;
            	let y = y as i32;
    			if x > rect.x && x < (rect.x + rect.w) && y > rect.y && y < (rect.y + rect.h) {
    				Some(WidgetMouseMoveEvent(layer_to_widget_pos(layer_pos, rect)))
    			} else {
    				None
    			}
    		},
            LayerMouseClickEvent(layer_pos) => {
            	let LayerPos(x, y) = layer_pos;
            	let x = x as i32;
            	let y = y as i32;
    			if x > rect.x && x < (rect.x + rect.w) && y > rect.y && y < (rect.y + rect.h) {
    				Some(WidgetMouseMoveEvent(layer_to_widget_pos(layer_pos, rect)))
    			} else {
    				None
    			}
    		},
    	}
    }

    pub fn handle_event(&mut self, sdl_event: sdl2::event::Event) {
    	let event = match sdl_event {
        	// /// (timestamp, window, winEventId, data1, data2)
			sdl2::event::WindowEvent(_, _, winEventId, data1, data2) => {
				match winEventId {
					sdl2::event::ResizedWindowEventId => {
						self.set_window_size(data1 as u32, data2 as u32);
					}
					_ => {}
				}
				None
			},
			// (timestamp, window, which, [MouseState], x, y, xrel, yrel)
            sdl2::event::MouseMotionEvent(_, _, _, _, x, y, _, _) => {
            	Some(LayerMouseMoveEvent(win_to_layer_pos(WindowPos(x as u32, y as u32), self)))
            },
            /// (timestamp, window, which, MouseBtn, x, y)
    		sdl2::event::MouseButtonDownEvent(_, _, _, _, x, y) => {
    			Some(LayerMouseClickEvent(win_to_layer_pos(WindowPos(x as u32, y as u32), self)))	
    		},
            _ => None
        };
        if event.is_none() {
        	return;
        }
    	/*for widget_info in self.widgets.iter_mut() {
    		let local_event = Layer::make_local_event(event.unwrap(), widget_info);
    		if local_event.is_none() {
        		continue;
        	}
			let result = widget_info.widget.handle_event(local_event.unwrap());
			if result.contains(NEED_REDRAW) {
				widget_info.need_redraw = true;
			}
			if result.contains(HANDLED) {
				return;
			}
		}*/
    }

    pub fn handle_event2(&mut self, sdl_event: sdl2::event::Event, widgets: &mut Vec<(&mut WidgetImpl, Rect)>) {
    	let event = match sdl_event {
        	// /// (timestamp, window, winEventId, data1, data2)
			sdl2::event::WindowEvent(_, _, winEventId, data1, data2) => {
				match winEventId {
					sdl2::event::ResizedWindowEventId => {
						self.set_window_size(data1 as u32, data2 as u32);
					}
					_ => {}
				}
				None
			},
			// (timestamp, window, which, [MouseState], x, y, xrel, yrel)
            sdl2::event::MouseMotionEvent(_, _, _, _, x, y, _, _) => {
            	Some(LayerMouseMoveEvent(win_to_layer_pos(WindowPos(x as u32, y as u32), self)))
            },
            /// (timestamp, window, which, MouseBtn, x, y)
    		sdl2::event::MouseButtonDownEvent(_, _, _, _, x, y) => {
    			Some(LayerMouseClickEvent(win_to_layer_pos(WindowPos(x as u32, y as u32), self)))	
    		},
            _ => None
        };
        if event.is_none() {
        	return;
        }
        for i in range(0, widgets.len()) {
    		let (ref mut w, r) = *widgets.get_mut(i);
    		let local_event = Layer::make_local_event(event.unwrap(), &r);
    		if local_event.is_none() {
        		continue;
        	}
			let result = w.handle_event(local_event.unwrap());
			if result.contains(NEED_REDRAW) {
				//widget_info.need_redraw = true;
			}
			if result.contains(HANDLED) {
				return;
			}
		}
    }

    pub fn button() {
    	
    }
}

bitflags! {
    flags EventHandlingResult: u32 {
    	const NONE   = 0x00000000,
        const HANDLED   = 0x00000001,
        const NEED_REDRAW   = 0x00000010,
    }
}

pub trait WidgetImpl {
    fn handle_event(&mut self, event: WidgetEvent) -> EventHandlingResult;

    fn draw(&self, &sdl2::render::Renderer, w: u32, h: u32);
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
