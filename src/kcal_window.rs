extern crate sdl2;
extern crate sdl2_ttf;

use imgui;
use imgui::SizeInCharacters;

use sdl2::pixels::RGB;

use scrollbar::scrollbar;
use label::label;

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
	bmr_str: String,
	goal_type: GoalType,
	protein_per_lbm: f32,
	protein: i32,
	fat: i32,
	ch: i32,
	protein_percent: f32,
	ch_percent: f32,
	fat_percent: f32,
	protein_str: String,
	ch_str: String,
	fat_str: String,
	bmr: f32,
	target_calories: f32,
}

impl KCalWindow {
	pub fn new() -> KCalWindow {
		KCalWindow {
			layer: imgui::Layer::new(),
			weight: String::new(),
			age: String::new(),
			height: String::new(),
			bmr_str: String::new(),
			weight_type: Kg,
			activity_mod: Sedentary,
			height_type: 0,
			goal_type: Bulking, 
			protein_per_lbm: 1.8f32,
			protein: 0,
			fat: 0,
			ch: 0,
			protein_percent: 10f32,
			ch_percent: 10f32,
			fat_percent: 10f32,
			protein_str: String::new(),
			fat_str: String::new(),
			ch_str: String::new(),
			bmr: 0f32,
			target_calories: 0f32,
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

        let win_height = if self.bmr_str.len() > 0 {4} else {0};
        self.layer.header(self.bmr_str.as_slice(), SizeInCharacters(21), SizeInCharacters(win_height))
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
			self.bmr = self.activity_mod.get_modified_value(d);
			self.target_calories = self.bmr;
			self.bmr_str.clear();
			self.bmr_str.push_str(("BMR: ".into_string() + (self.bmr as i32).to_string() + " kCal").as_slice());

			self.protein = ((self.protein_percent / 100f32 * self.target_calories) / 4f32) as i32;
		} else {
			self.bmr_str.clear();
		}
		if self.bmr_str.len() >= 0 {
			self.protein_str.clear();
	        self.protein_str.push_str(self.protein.to_string().as_slice() );


			self.layer.dropdown(vec!["Bulking", "Cutting", ].as_slice(), &mut self.goal_type)
	        	.x(SizeInCharacters(7))
	        	.down(SizeInCharacters(1))
	        	.draw(renderer);

	        if self.layer.textfield(&mut self.protein_str, SizeInCharacters(4))
	        	.x(SizeInCharacters(7))
	        	.down(SizeInCharacters(1))
	            .label("Protein: ")
	            .draw(renderer) {
				let maybe_prot = from_str::<i32>(self.protein_str.as_slice());
				if maybe_prot.is_some() {
					self.protein = maybe_prot.unwrap();
					self.protein_percent = (self.protein*4) as f32 / (self.target_calories / 100f32);
				}
			}
	        
	        if scrollbar(&mut self.layer, SizeInCharacters(10), 0f32, 100f32, &mut self.protein_percent )
	        	.right(SizeInCharacters(2))
	        	.draw(renderer) {
	        	self.ch_percent = 100f32 - self.protein_percent - self.fat_percent;

	        	self.protein = ((self.protein_percent / 100f32 * self.target_calories) / 4f32) as i32;
	        }
	        let scrollbar_x = self.layer.last_x;

	        if scrollbar(&mut self.layer, SizeInCharacters(10), 1f32, 4f32, &mut self.protein_per_lbm )
	        	.right(SizeInCharacters(1))
	        	.draw(renderer) {
	        	self.protein_str.clear();
	        	self.protein_str.push_str(self.protein_per_lbm.to_string().as_slice());
	        }


	        if self.layer.textfield(&mut self.ch_str, SizeInCharacters(4))
	        	.x(SizeInCharacters(7))
	        	.down(SizeInCharacters(1))
	            .label("Ch: ")
	            .draw(renderer) {
				let maybe_ch = from_str::<f32>(self.ch_str.as_slice());
				if maybe_ch.is_some() {
					self.ch_percent = maybe_ch.unwrap();
				}
			}
	        if scrollbar(&mut self.layer, SizeInCharacters(10), 0f32, 100f32, &mut self.ch_percent )
	        	.x(scrollbar_x)
	        	.color(RGB(237, 166, 0))
	        	.draw(renderer) {
	        	self.fat_percent = 100f32 - self.protein_percent - self.ch_percent;
	        	self.ch = ((self.ch_percent / 100f32 * self.target_calories) / 4f32) as i32;
	        	self.ch_str.clear();
	        	self.ch_str.push_str(self.ch.to_string().as_slice() );
	        }

	        if self.layer.textfield(&mut self.fat_str, SizeInCharacters(4))
	        	.x(SizeInCharacters(7))
	        	.down(SizeInCharacters(1))
	            .label("Fat: ")
	            .draw(renderer) {
				let maybe_fat = from_str::<f32>(self.fat_str.as_slice());
				if maybe_fat.is_some() {
					self.fat_percent = maybe_fat.unwrap();
				}
			}
	        if scrollbar(&mut self.layer, SizeInCharacters(10), 0f32, 100f32, &mut self.fat_percent )
	        	.x(scrollbar_x)
	        	.color(RGB(210, 93, 90))
	        	.draw(renderer) {
	        	self.ch_percent = 100f32 - self.protein_percent - self.fat_percent;
	        	self.fat = ((self.fat_percent / 100f32 * self.target_calories) / 9f32) as i32;
	        	self.fat_str.clear();
	        	self.fat_str.push_str(self.fat.to_string().as_slice() );
	        }
        }
	}

}