extern crate sdl2;
extern crate sdl2_ttf;

use imgui::imgui;
use imgui::imgui::SizeInCharacters;

use sdl2::pixels::RGB;

use imgui::label::label;
use imgui::textfield::textfield_f32;
use imgui::textfield::textfield_i32;
use imgui::textfield::textfield_str;
use tricolor_field::tricolor_field_str;
use imgui::button::button;
use imgui::header::header;
use db;


impl imgui::IndexValue for db::WeightType {
	fn set(&mut self, value: uint) {
		*self = match value {
			0 => db::G,
			1 => db::Dkg,
			_ => db::Kg,
		};
	}
	fn get(&self) -> uint {
		*self as uint
	}
}


pub struct KCalTable<'a> {
	pub layer: imgui::Layer,
}

impl<'a> KCalTable<'a> {

	pub fn new() -> KCalTable<'a> {
		KCalTable {
			layer: imgui::Layer::new(),
		}
	}

	pub fn draw_rects(&self, renderer: &sdl2::render::Renderer) {
		for x in range(0, 20) {
			for y in range(0, 20) {
				imgui::fill_rect(renderer, x * 42, y * 22, 40, 10, RGB(51, 51, 51));
			}
		}
	}

	pub fn draw_gradient_rects(&mut self, renderer: &sdl2::render::Renderer) {
		for x in range(0, 20) {
			for y in range(0, 20) {
				self.layer.draw_rect_gradient(renderer, x * 42, y * 22, 40, 10, RGB(114, 114, 114), RGB(68, 68, 68));
			}
		}
	}

	pub fn draw_texts(&mut self, renderer: &sdl2::render::Renderer) {
		for x in range(0, 20) {
			for y in range(0, 20) {
				self.layer.draw_text(x * 42, 20 + y * 22, renderer, "bali", RGB(221, 221, 221));
			}
		}
	}

	// 1 oldalon 16
	pub fn do_logic(&'a mut self, renderer: &sdl2::render::Renderer, event: &sdl2::event::Event, foods: &'a mut Vec<db::Food>) -> bool {
		self.layer.handle_event(event);
		//self.draw_gradient_rects(renderer);
		//self.draw_rects(renderer);
		//self.draw_texts(renderer);

		//return false;

		header(&mut self.layer, "Foods", SizeInCharacters(53), SizeInCharacters(37))
			.x(SizeInCharacters(1))
			.y(SizeInCharacters(1))
			.draw(renderer);
		let price_header_x = self.layer.last_x + self.layer.last_w;
		let first_row = self.layer.last_x + SizeInCharacters(1);
		for (_, food) in foods.iter_mut().enumerate() {
			let fs = food.weight_type.to_g(food.size);
			let values = [food.protein / fs, food.ch / fs, food.fat / fs];
			if tricolor_field_str(textfield_str(&mut self.layer, &mut food.name, SizeInCharacters(15))
				.x(first_row)
				.down(SizeInCharacters(1))
	            .default_text("Name..."), values).draw(renderer) {
	        }

			if textfield_i32(&mut self.layer, &mut food.size, SizeInCharacters(4))
				.right(SizeInCharacters(2))
	            .draw(renderer) {
	        }
	        self.layer.dropdown(vec!["g", "dkg", "kg"].as_slice(), &mut food.weight_type)
	        	.right(SizeInCharacters(1))
	        	.draw(renderer);
	        if textfield_f32(&mut self.layer, &mut food.protein, SizeInCharacters(4))
				.right(SizeInCharacters(2))
	            .label_color(RGB(76, 166, 79))
	            .value_color(RGB(76, 166, 79))
	            .draw(renderer) {
	        }
	        if textfield_f32(&mut self.layer, &mut food.ch, SizeInCharacters(4))
				.right(SizeInCharacters(2))
	            .label_color(RGB(237, 166, 0))
	            .value_color(RGB(237, 166, 0))
	            .draw(renderer) {
	        }
	        if textfield_f32(&mut self.layer, &mut food.fat, SizeInCharacters(4))
				.right(SizeInCharacters(2))
	            .label_color(RGB(210, 93, 90))
	            .value_color(RGB(210, 93, 90))
	            .draw(renderer) {
	        }
	        label(&mut self.layer, format!("{: >4}", food.get_kcal()).as_slice())
				.right(SizeInCharacters(2))
	            .draw(renderer);


	        // PRICE
			if textfield_i32(&mut self.layer, &mut food.price_weight, SizeInCharacters(4))
				.right(SizeInCharacters(2))
	            .draw(renderer) {
	        }
	        self.layer.dropdown(vec!["g", "dkg", "kg"].as_slice(), &mut food.price_weight_type)
	        	.right(SizeInCharacters(1))
	        	.draw(renderer);
	        if textfield_i32(&mut self.layer, &mut food.price, SizeInCharacters(4))
				.right(SizeInCharacters(2))
	            .draw(renderer) {
	        }

	        if button(&mut self.layer, "Del").right(SizeInCharacters(4)).draw(renderer) {
				
			}
		}
		if button(&mut self.layer, "New").down(SizeInCharacters(1))
			.x(first_row).draw(renderer) {
			foods.push(db::Food::new());
		}
		if button(&mut self.layer, "Close").right(SizeInCharacters(2)).draw(renderer) {
			return true;
		}
		header(&mut self.layer, "Prices", SizeInCharacters(18), SizeInCharacters(37))
				.x(price_header_x)
				.y(SizeInCharacters(1))
				.draw(renderer);
		header(&mut self.layer, "Név", SizeInCharacters(17), SizeInCharacters(36))
			.x(SizeInCharacters(1))
			.y(SizeInCharacters(2))
			.draw(renderer);
		header(&mut self.layer, "Tömeg", SizeInCharacters(12), SizeInCharacters(36))
			.x(SizeInCharacters(18))
			.y(SizeInCharacters(2))
			.draw(renderer);
		header(&mut self.layer, "P", SizeInCharacters(6), SizeInCharacters(36))
			.x(SizeInCharacters(30))
			.y(SizeInCharacters(2))
			.draw(renderer);
		header(&mut self.layer, "C", SizeInCharacters(6), SizeInCharacters(36))
			.x(SizeInCharacters(36))
			.y(SizeInCharacters(2))
			.draw(renderer);
		header(&mut self.layer, "F", SizeInCharacters(6), SizeInCharacters(36))
			.x(SizeInCharacters(42))
			.y(SizeInCharacters(2))
			.draw(renderer);
		header(&mut self.layer, "kCal", SizeInCharacters(6), SizeInCharacters(36))
			.x(SizeInCharacters(48))
			.y(SizeInCharacters(2))
			.draw(renderer);

		header(&mut self.layer, "Tömeg", SizeInCharacters(12), SizeInCharacters(36))
			.x(SizeInCharacters(54))
			.y(SizeInCharacters(2))
			.draw(renderer);
		header(&mut self.layer, "Price", SizeInCharacters(6), SizeInCharacters(36))
			.x(SizeInCharacters(66))
			.y(SizeInCharacters(2))
			.draw(renderer);
		return false;
	}

}