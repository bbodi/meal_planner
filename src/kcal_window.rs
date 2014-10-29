extern crate sdl2;
extern crate sdl2_ttf;

use imgui;
use imgui::SizeInCharacters;

enum WeightType {
	Kg, Lb,
}

impl imgui::IndexValue for WeightType {
	fn set(&mut self, value: uint) {
		*self = match value {
			0 => Kg,
			_ => Lb,
		};
	}
	fn get(&self) -> uint {
		*self as uint
	}
}

enum ActivityModifier {
	Sedentary, Lightly, Moderately, Very, Extremely
}

impl imgui::IndexValue for ActivityModifier {
	fn set(&mut self, value: uint) {
		*self = match value {
			0 => Sedentary,
			2 => Lightly,
			3 => Moderately,
			4 => Very,
			_ => Extremely,
		};
	}
	fn get(&self) -> uint {
		*self as uint
	}
}

pub struct KCalWindow {
	layer: imgui::Layer,

	weight: String,
	age: String,
	height: String,
	weight_type: WeightType,
	bmr: String
}

impl KCalWindow {
	pub fn new() -> KCalWindow {
		KCalWindow {
			layer: imgui::Layer::new(),
			weight: String::new(),
			age: String::new(),
			height: String::new(),
			bmr: String::new(),
			weight_type: Kg,
		}
	}

	pub fn do_logic(&mut self, renderer: &sdl2::render::Renderer, event: sdl2::event::Event) {
		self.layer.handle_event(event);
		self.layer.header("BMR", SizeInCharacters(21), SizeInCharacters(10))
			.x(SizeInCharacters(6))
			.y(SizeInCharacters(5))
			.draw(renderer);
		if self.layer.textfield(&mut self.weight, SizeInCharacters(4))
			.x(SizeInCharacters(10))
			.y(SizeInCharacters(7))
            .default_text("Tömeg")
            .draw(renderer) {
        }
        self.layer.dropdown(vec!["kg", "lb"].as_slice(), &mut self.weight_type)
        	.right(SizeInCharacters(2))
        	.draw(renderer);

        if self.layer.textfield(&mut self.age, SizeInCharacters(4))
        	.x(SizeInCharacters(10))
        	.down(SizeInCharacters(1))
            .default_text("Kor")
            .draw(renderer) {
        }

        if self.layer.textfield(&mut self.height, SizeInCharacters(4))
        	.x(SizeInCharacters(10))
        	.down(SizeInCharacters(1))
            .default_text("Height")
            .draw(renderer) {
        }

        self.layer.dropdown(vec!["cm", "ft"].as_slice(), &mut self.weight_type)
        	.right(SizeInCharacters(2))
        	.draw(renderer);

        self.layer.dropdown(vec!["Sedentary", "Lightly active", "Moderately active", "Very active", "Extremely active", ].as_slice(), &mut self.weight_type)
        	.x(SizeInCharacters(7))
        	.down(SizeInCharacters(1))
        	.draw(renderer);

        self.layer.header(self.bmr.as_slice(), SizeInCharacters(21), SizeInCharacters(0))
			.x(SizeInCharacters(6))
			.y(SizeInCharacters(15))
			.draw(renderer);
		let maybe_weight: Option<i32> = ::std::from_str::FromStr::from_str(self.weight.as_slice());
		let maybe_age: Option<i32> = ::std::from_str::FromStr::from_str(self.age.as_slice());
		let maybe_height: Option<i32> = ::std::from_str::FromStr::from_str(self.height.as_slice());
		if maybe_weight.is_some() && maybe_age.is_some() && maybe_height.is_some() {
			//(66 + (13,7*78 kg) + (5*190 cm) - ( 6,8*25 year)) * 1.55 activity level
			let a = 13.7f32 * maybe_weight.unwrap() as f32;
			let b = 5f32 * maybe_height.unwrap() as f32;
			let c = 6.8f32 * maybe_age.unwrap() as f32;
			let d = (66f32+ a+b+c) * 1.55f32;
			self.bmr.clear();
			self.bmr.push_str(d.to_string().as_slice());
		}
	}

}