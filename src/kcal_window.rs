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

impl WeightType {
	pub fn to_kg(&self, value: i32) -> f32 {
		match *self {
			Kg => value as f32,
			Lb => (value as f32 * 0.453592f32),
		}
	}
}

enum GoalType {
	Bulking, Cutting
}

impl imgui::IndexValue for GoalType {
	fn set(&mut self, value: uint) {
		*self = match value {
			0 => Bulking,
			_ => Cutting,
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
			1 => Lightly,
			2 => Moderately,
			3 => Very,
			_ => Extremely,
		};
	}
	fn get(&self) -> uint {
		*self as uint
	}
}

impl ActivityModifier {
	pub fn get_modified_value(&self, value: f32) -> f32 {
		match *self {
			Sedentary => (value as f32 * 1.2f32),
			Lightly => (value as f32 * 1.375f32),
			Moderately => (value as f32 * 1.55f32),
			Very => (value as f32 * 1.725f32),
			Extremely => (value as f32 * 1.9f32),
		}
	}
}

pub struct KCalWindow {
	layer: imgui::Layer,

	weight: String,
	age: String,
	height: String,
	weight_type: WeightType,
	activity_mod: ActivityModifier,
	height_type: i32,
	bmr: String,
	goal_type: GoalType,
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
			activity_mod: Sedentary,
			height_type: 0,
			goal_type: Bulking, 
		}
	}

	pub fn do_logic(&mut self, renderer: &sdl2::render::Renderer, event: sdl2::event::Event) {
		self.layer.handle_event(event);
		self.layer.header("BMR", SizeInCharacters(21), SizeInCharacters(10))
			.x(SizeInCharacters(6))
			.y(SizeInCharacters(5))
			.draw(renderer);
		if self.layer.textfield(&mut self.weight, SizeInCharacters(4))
			.x(SizeInCharacters(7))
			.y(SizeInCharacters(7))
            .label("Mass: ")
            .draw(renderer) {
        }
        self.layer.dropdown(vec!["kg", "lb"].as_slice(), &mut self.weight_type)
        	.right(SizeInCharacters(2))
        	.draw(renderer);

        if self.layer.textfield(&mut self.age, SizeInCharacters(4))
        	.x(SizeInCharacters(7))
        	.down(SizeInCharacters(1))
            .label("Age: ")
            .draw(renderer) {
        }

        if self.layer.textfield(&mut self.height, SizeInCharacters(4))
        	.x(SizeInCharacters(7))
        	.down(SizeInCharacters(1))
            .label("Height: ")
            .draw(renderer) {
        }

        self.layer.dropdown(vec!["cm", "ft"].as_slice(), &mut self.height_type)
        	.right(SizeInCharacters(2))
        	.draw(renderer);

        self.layer.dropdown(vec!["Sedentary", "Lightly active", "Moderately active", "Very active", "Extremely active", ].as_slice(), &mut self.activity_mod)
        	.x(SizeInCharacters(7))
        	.down(SizeInCharacters(1))
        	.draw(renderer);

        let win_height = if self.bmr.len() > 0 {4} else {0};
        self.layer.header(self.bmr.as_slice(), SizeInCharacters(21), SizeInCharacters(win_height))
			.x(SizeInCharacters(6))
			.y(SizeInCharacters(15))
			.draw(renderer);
		let maybe_weight: Option<i32> = ::std::from_str::FromStr::from_str(self.weight.as_slice());
		let maybe_age: Option<i32> = ::std::from_str::FromStr::from_str(self.age.as_slice());
		let maybe_height: Option<i32> = ::std::from_str::FromStr::from_str(self.height.as_slice());
		if maybe_weight.is_some() && maybe_age.is_some() && maybe_height.is_some() {
			//(66 + (13,7*78 kg) + (5*190 cm) - ( 6,8*25 year)) * 1.55 activity level
			let a = 13.7f32 * self.weight_type.to_kg(maybe_weight.unwrap()) as f32;
			let height = maybe_height.unwrap() as f32 * if self.height_type == 1 {30.48f32} else {1f32};
			let b = 5f32 * height;
			let c = 6.8f32 * maybe_age.unwrap() as f32;
			let d = (66f32+ a+b+c);
			let activity_modified = self.activity_mod.get_modified_value(d);
			self.bmr.clear();
			self.bmr.push_str(((activity_modified as i32).to_string() + " kCal").as_slice());
		} else {
			self.bmr.clear();
		}
		if self.bmr.len() > 0 {
			self.layer.dropdown(vec!["Bulking", "Cutting", ].as_slice(), &mut self.goal_type)
	        	.x(SizeInCharacters(7))
	        	.down(SizeInCharacters(1))
	        	.draw(renderer);
        }
	}

}