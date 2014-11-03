extern crate sdl2;
extern crate sdl2_ttf;

use imgui::base;
use imgui::base::SizeInCharacters;

use sdl2::pixels::RGB;

use imgui::label::label;
use imgui::textfield::textfield_f32;
use imgui::textfield::textfield_i32;
use imgui::textfield::textfield_str;
use tricolor_field::tricolor_field_str;
use imgui::button::button;
use imgui::header::header;
use imgui::dropdown::dropdown;
use db;



pub struct KCalTable<'a> {
	pub layer: base::Layer,
	pub page: uint,
	pub last_food_id: uint
}

impl<'a> KCalTable<'a> {

	pub fn new() -> KCalTable<'a> {
		KCalTable {
			layer: base::Layer::new(),
			page: 0,
			last_food_id: 0,
		}
	}

	pub fn draw_rects(&mut self) {
		for x in range(0, 20) {
			for y in range(0, 20) {
				self.layer.fill_rect(x * 42, y * 22, 40, 10, RGB(51, 51, 51));
			}
		}
	}

	pub fn draw_gradient_rects(&mut self, renderer: &sdl2::render::Renderer) {
		for x in range(0, 20) {
			for y in range(0, 20) {
				self.layer.draw_rect_gradient(x * 42, y * 22, 40, 10, RGB(114, 114, 114), RGB(68, 68, 68));
			}
		}
	}

	pub fn draw_texts(&mut self) {
		for x in range(0, 20) {
			for y in range(0, 20) {
				self.layer.draw_text(x * 42, 20 + y * 22, "bali", RGB(221, 221, 221));
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

		let column_height = SizeInCharacters(36);
		header(&mut self.layer, "Foods", SizeInCharacters(70), column_height)
			.x(SizeInCharacters(1))
			.y(SizeInCharacters(1))
			.draw(renderer);
		let price_header_x = self.layer.last_x + self.layer.last_w;
		let first_row = self.layer.last_x + SizeInCharacters(1);
		self.last_food_id = foods.len();
		for (i, food) in foods.iter_mut().skip(self.page * 16).take((self.page+1) * 16).enumerate() {
			let fs = food.weight_type.to_g(food.weight);
			let values = (food.protein, food.ch, food.fat, fs);
			let _ = tricolor_field_str(textfield_str(&mut self.layer, &mut food.name, SizeInCharacters(20))
				.x(first_row)
				.down(SizeInCharacters(1))
	            .default_text("Name..."), values).draw(renderer);

			let _ = textfield_f32(&mut self.layer, &mut food.weight, SizeInCharacters(4))
				.right(SizeInCharacters(2))
	            .draw(renderer);

	        dropdown(&mut self.layer, vec!["g", "dkg", "kg"].as_slice(), &mut food.weight_type)
	        	.right(SizeInCharacters(1))
	        	.draw(renderer);
	        let _ = textfield_f32(&mut self.layer, &mut food.protein, SizeInCharacters(4))
				.right(SizeInCharacters(2))
	            .label_color(RGB(76, 166, 79))
	            .value_color(RGB(76, 166, 79))
	            .draw(renderer);

	        let _ = textfield_f32(&mut self.layer, &mut food.ch, SizeInCharacters(4))
				.right(SizeInCharacters(2))
	            .label_color(RGB(237, 166, 0))
	            .value_color(RGB(237, 166, 0))
	            .draw(renderer);

	        let _ = textfield_f32(&mut self.layer, &mut food.fat, SizeInCharacters(4))
				.right(SizeInCharacters(2))
	            .label_color(RGB(210, 93, 90))
	            .value_color(RGB(210, 93, 90))
	            .draw(renderer);

	        label(&mut self.layer, format!("{: >4}", food.get_kcal()).as_slice())
				.right(SizeInCharacters(2))
	            .draw(renderer);


	        // PRICE
			let _ = textfield_i32(&mut self.layer, &mut food.price_weight, SizeInCharacters(4))
				.right(SizeInCharacters(2))
	            .draw(renderer);
	        dropdown(&mut self.layer, vec!["g", "dkg", "kg"].as_slice(), &mut food.price_weight_type)
	        	.right(SizeInCharacters(1))
	        	.draw(renderer);
	        let _ = textfield_i32(&mut self.layer, &mut food.price, SizeInCharacters(4))
				.right(SizeInCharacters(2))
	            .draw(renderer);

	        if button(&mut self.layer, "Del").right(SizeInCharacters(4)).draw(renderer) {

			}
		}
		if button(&mut self.layer, "Prev")
			.disabled(self.page == 0)
			.down(SizeInCharacters(1))
			.x(first_row)
			.y(SizeInCharacters(35))
			.draw(renderer) {
			self.page = self.page - 1;
		}
		if button(&mut self.layer, "Next")
			.disabled(self.page >= (foods.len() / 16))
			.right(SizeInCharacters(60))
			.draw(renderer) {
			self.page = self.page + 1;
		}
		if button(&mut self.layer, "New")
			.down(SizeInCharacters(1))
			.x(first_row).draw(renderer) {
			self.last_food_id = self.last_food_id + 1;
			foods.push(db::Food::new(self.last_food_id));
		}
		if button(&mut self.layer, "Save").right(SizeInCharacters(2)).draw(renderer) {
			return true;
		}
		header(&mut self.layer, "Név", SizeInCharacters(22), column_height - SizeInCharacters(1))
			.x(SizeInCharacters(1))
			.y(SizeInCharacters(2))
			.draw(renderer);
		header(&mut self.layer, "Tömeg", SizeInCharacters(12), column_height - SizeInCharacters(1))
			.right(SizeInCharacters(0))
			.draw(renderer);
		header(&mut self.layer, "P", SizeInCharacters(6), column_height - SizeInCharacters(1))
			.right(SizeInCharacters(0))
			.draw(renderer);
		header(&mut self.layer, "C", SizeInCharacters(6), column_height - SizeInCharacters(1))
			.right(SizeInCharacters(0))
			.draw(renderer);
		header(&mut self.layer, "F", SizeInCharacters(6), column_height - SizeInCharacters(1))
			.right(SizeInCharacters(0))
			.draw(renderer);
		header(&mut self.layer, "kCal", SizeInCharacters(6), column_height - SizeInCharacters(1))
			.right(SizeInCharacters(0))
			.draw(renderer);

		header(&mut self.layer, "Prices", SizeInCharacters(18), column_height)
				.up(SizeInCharacters(1))
				.right(SizeInCharacters(0))
				.draw(renderer);
		header(&mut self.layer, "Tömeg", SizeInCharacters(12), column_height - SizeInCharacters(1))
			.down(SizeInCharacters(0))
			.draw(renderer);

		header(&mut self.layer, "Price", SizeInCharacters(6), column_height - SizeInCharacters(1))
			.right(SizeInCharacters(0))
			.draw(renderer);
		return false;
	}

}
