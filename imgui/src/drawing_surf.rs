extern crate sdl2;
extern crate sdl2_ttf;

use std::cmp::max;
use std::collections::LruCache;

use sdl2::rect::Rect as SdlRect;
use sdl2::rect::Point;
use sdl2::pixels::RGB;

#[deriving(PartialEq, Clone)]
pub enum DrawCommand {
	FilledRect(i32, i32, i32, i32, sdl2::pixels::Color),
	Rect(i32, i32, i32, i32, i32, sdl2::pixels::Color),
	Line(i32, i32, i32, i32, sdl2::pixels::Color),
	// x, y, bold, color
	Text(i32, i32, bool, String, sdl2::pixels::Color),
	GradientRect(i32, i32, i32, i32, sdl2::pixels::Color, sdl2::pixels::Color),
}

pub struct DrawingSurface {
	draw_commands: Vec<DrawCommand>,
    text_cache: LruCache<(u8, u8, u8, bool, u64), sdl2::render::Texture>,
    // w, h, src_color, dst_color
    gradient_rect_cache: LruCache<(u32, u32, u8, u8, u8, u8, u8, u8), sdl2::render::Texture>,
    font: sdl2_ttf::Font,
    bfont: sdl2_ttf::Font,
}

impl DrawingSurface {
	pub fn new(font: sdl2_ttf::Font, bfont: sdl2_ttf::Font) -> DrawingSurface {
		DrawingSurface {
			draw_commands: vec![],
            text_cache: LruCache::new(200),
            gradient_rect_cache: LruCache::new(200),
            font: font,
            bfont: bfont,
		}
	}

	pub fn draw(&mut self, renderer: &sdl2::render::Renderer) {
        let newvec = self.draw_commands.iter().map(|x| x.clone()).collect::<Vec<DrawCommand>>();
    	for draw_command in newvec.iter() {
    		match draw_command {
    			&Line(x1, y1, x2, y2, color) => {
    				sdl_line(renderer, x1, y1, x2, y2, color);
    			}
    			&FilledRect(x, y, w, h, color) => {
    				sdl_fill_rect(renderer, x, y, w, h, color);
    			},
    			&Rect(x, y, w, h, b, color) => {
    				sdl_rect(renderer, x, y, w, h, b, color);
    			},
    			&GradientRect(x, y, w, h, color1, color2) => {
    				self.do_draw_rect_gradient(renderer, x, y, w, h, color1, color2);
    			},
    			&Text(x, y, bold, ref text, color) => {
    				self.do_do_draw_text(renderer, x, y, bold, text.as_slice(), color);
    			},
    		}
    	}
    }

    pub fn window_resized(&mut self) {
        self.text_cache.clear();
        self.gradient_rect_cache.clear();
    }

    pub fn clear(&mut self) {
    	self.draw_commands.clear();
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
        self.draw_commands.push(FilledRect(x, y, w, h, color));
    }

    pub fn draw_rect(&mut self, x: i32, y: i32, w: i32, h: i32, border: i32, color: sdl2::pixels::Color) {
        self.draw_commands.push(Rect(x, y, w, h, border, color));
    }
}

fn sdl_rect(renderer: &sdl2::render::Renderer, x: i32, y: i32, w: i32, h: i32, border: i32, color: sdl2::pixels::Color) {
    let _ = renderer.set_draw_color(color);
    for i in range(0, border) {
        let _ = renderer.draw_rect(&SdlRect::new(x+i, y+i, w-2*i, h-2*i));
    }
}

fn sdl_fill_rect(renderer: &sdl2::render::Renderer, x: i32, y: i32, w: i32, h: i32, color: sdl2::pixels::Color) {
    let _ = renderer.set_draw_color(color);
    let _ = renderer.fill_rect(&SdlRect::new(x, y, w, h));
}

fn sdl_line(renderer: &sdl2::render::Renderer, x1: i32, y1: i32, x2: i32, y2: i32, color: sdl2::pixels::Color) {
    let _ = renderer.set_draw_color(color);
    let _ = renderer.draw_line(Point::new(x1, y1), Point::new(x2, y2));
}

fn create_gradient_texture(renderer: &sdl2::render::Renderer, w: i32, h: i32, start_color: sdl2::pixels::Color, end_color: sdl2::pixels::Color) -> sdl2::render::Texture {
    let texture =  match renderer.create_texture(sdl2::pixels::RGBA8888, sdl2::render::AccessTarget, w as int, h as int) {
        Ok(t) => t,
        Err(e) => panic!(e),
    };
    let _ = renderer.set_render_target(Some(&texture));
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
    let _ = renderer.set_render_target(None);
    texture
}
