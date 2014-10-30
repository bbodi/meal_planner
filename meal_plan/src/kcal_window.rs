extern crate sdl2;
extern crate sdl2_ttf;

use imgui;
use imgui::SizeInCharacters;

use sdl2::pixels::RGB;

use scrollbar::scrollbar;
use label::label;
use textfield::textfield_f32;
use textfield::textfield_i32;
use panel::panel;
use header::header;

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
	pub fn to_kg(&self, value: f32) -> f32 {
		match *self {
			Kg => value,
			Lb => (value * 0.453592f32),
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

	weight: f32,
	age: i32,
	height: i32,
	weight_type: WeightType,
	activity_mod: ActivityModifier,
	height_type: i32,
	bmr_str: String,
	goal_type: GoalType,
	protein_per_kg: f32,
	protein: i32,
	fat: i32,
	ch: i32,
	protein_percent: f32,
	ch_percent: f32,
	fat_percent: f32,
	bmr: f32,
	target_calories: f32,
}

impl KCalWindow {

	fn set_target_calories(&mut self, tc: f32) {
		self.target_calories = tc;
		self.protein = ((self.protein_percent / 100f32 * self.target_calories) / 4f32) as i32;
		self.ch = ((self.ch_percent / 100f32 * self.target_calories) / 4f32) as i32;
		self.fat = ((self.fat_percent / 100f32 * self.target_calories) / 9f32) as i32;
		self.protein_per_kg = self.protein as f32 / self.weight;
	}

	fn protein_percent_changed(&mut self) {
		self.ch_percent = 100f32 - self.protein_percent - self.fat_percent;
		self.protein = ((self.protein_percent / 100f32 * self.target_calories) / 4f32) as i32;
		self.protein_per_kg = self.protein as f32 / self.weight;
	}

	fn ch_percent_changed(&mut self) {
		self.fat_percent = 100f32 - self.protein_percent - self.ch_percent;
	    self.ch = ((self.ch_percent / 100f32 * self.target_calories) / 4f32) as i32;
	}

	fn fat_percent_changed(&mut self) {
		self.ch_percent = 100f32 - self.protein_percent - self.fat_percent;
		self.fat = ((self.fat_percent / 100f32 * self.target_calories) / 9f32) as i32;
	}

	fn recalc_bmr(&mut self) {
		//(66 + (13,7*78 kg) + (5*190 cm) - ( 6,8*25 year)) * 1.55 activity level
		let a = 13.7f32 * self.weight_type.to_kg(self.weight);
		let height = self.height as f32 * if self.height_type == 1 {30.48f32} else {1f32};
		let b = 5f32 * height;
		let c = 6.8f32 * self.age as f32;
		let d = 66f32 + a+b+c;
		let bmr = self.activity_mod.get_modified_value(d);
		self.bmr = bmr;
		self.set_target_calories(bmr);
		self.bmr_str.clear();
		self.bmr_str.push_str(("BMR: ".into_string() + (self.bmr as i32).to_string() + " kCal").as_slice());
	}

	pub fn new() -> KCalWindow {
		KCalWindow {
			layer: imgui::Layer::new(),
			weight: 0f32,
			age: 0,
			height: 0,
			bmr_str: String::new(),
			weight_type: Kg,
			activity_mod: Sedentary,
			height_type: 0,
			goal_type: Bulking, 
			protein_per_kg: 1.8f32,
			protein: 0,
			fat: 0,
			ch: 0,
			protein_percent: 10f32,
			ch_percent: 10f32,
			fat_percent: 10f32,
			bmr: 0f32,
			target_calories: 0f32,
		}
	}

	pub fn do_logic(&mut self, renderer: &sdl2::render::Renderer, event: &sdl2::event::Event) {
		self.layer.handle_event(event);
		header(&mut self.layer, "BMR", SizeInCharacters(70), SizeInCharacters(25))
			.x(SizeInCharacters(6))
			.y(SizeInCharacters(5))
			.draw(renderer);
		if textfield_f32(&mut self.layer, &mut self.weight, SizeInCharacters(4))
			.x(SizeInCharacters(7))
			.y(SizeInCharacters(7))
            .label("Mass: ")
            .draw(renderer) {
			self.recalc_bmr();            	
        }
        self.layer.dropdown(vec!["kg", "lb"].as_slice(), &mut self.weight_type)
        	.right(SizeInCharacters(1))
        	.draw(renderer);

        if textfield_i32(&mut self.layer, &mut self.age, SizeInCharacters(4))
        	.x(SizeInCharacters(7))
        	.right(SizeInCharacters(2))
            .label("Age: ")
            .draw(renderer) {
            self.recalc_bmr();
        }

        if textfield_i32(&mut self.layer, &mut self.height, SizeInCharacters(4))
        	.x(SizeInCharacters(7))
        	.right(SizeInCharacters(2))
            .label("Height: ")
            .draw(renderer) {
            self.recalc_bmr();
        }

        self.layer.dropdown(vec!["cm", "ft"].as_slice(), &mut self.height_type)
        	.right(SizeInCharacters(1))
        	.draw(renderer);

        if self.layer.dropdown(vec!["Sedentary", "Lightly active", "Moderately active", "Very active", "Extremely active", ].as_slice(), &mut self.activity_mod)
        	.x(SizeInCharacters(7))
        	.right(SizeInCharacters(1))
        	.draw(renderer) {
        	self.recalc_bmr();
        }

        header(&mut self.layer, self.bmr_str.as_slice(), SizeInCharacters(70), SizeInCharacters(0))
			.x(SizeInCharacters(6))
			.down(SizeInCharacters(2))
			.draw(renderer);

		if self.bmr_str.len() > 0 {
			self.layer.dropdown(vec!["Bulking", "Cutting", ].as_slice(), &mut self.goal_type)
	        	.x(SizeInCharacters(7))
	        	.down(SizeInCharacters(1))
	        	.draw(renderer);

	        let mut scrollbar_x = SizeInCharacters(0);
	        panel(&mut self.layer, SizeInCharacters(68), SizeInCharacters(3))
	        	.x(SizeInCharacters(7))
	        	.down(SizeInCharacters(0))
	        	.draw(renderer);
	        {
	        	
		        label(&mut self.layer, format!("Protein: {:3}", self.protein).as_slice() )
		        	.inner_down(SizeInCharacters(1))
		            .draw(renderer);
		        
		        if scrollbar(&mut self.layer, SizeInCharacters(20), 0f32, 100f32, &mut self.protein_percent )
		        	.right(SizeInCharacters(2))
		        	.draw(renderer) {
		        	self.protein_percent_changed();
		        }
		        scrollbar_x = self.layer.last_x;

		        label(&mut self.layer, "g/kg: ")
		        	.right(SizeInCharacters(2))
		            .draw(renderer);

		        if scrollbar(&mut self.layer, SizeInCharacters(6), 1f32, 4f32, &mut self.protein_per_kg )
		        	.right(SizeInCharacters(1))
		        	.draw(renderer) {
		        	let calced_prot = self.protein_per_kg * self.weight;
		        	self.protein_percent = (calced_prot*4f32) as f32 / (self.target_calories / 100f32);
		        	self.protein_percent_changed();
		        }
	    	}

	    	panel(&mut self.layer, SizeInCharacters(68), SizeInCharacters(3))
	        	.x(SizeInCharacters(7))
	        	.down(SizeInCharacters(1))
	        	.draw(renderer);
	        {
		        label(&mut self.layer, ("Ch: ".into_string() + self.ch.to_string().as_slice()).as_slice() )
		        	.inner_down(SizeInCharacters(1))
		            .draw(renderer);

		        if scrollbar(&mut self.layer, SizeInCharacters(20), 0f32, 100f32, &mut self.ch_percent )
		        	.x(scrollbar_x)
		        	.color(RGB(237, 166, 0))
		        	.draw(renderer) {
		        	self.ch_percent_changed();
		        }
		    }

		    panel(&mut self.layer, SizeInCharacters(68), SizeInCharacters(3))
	        	.x(SizeInCharacters(7))
	        	.down(SizeInCharacters(1))
	        	.draw(renderer);
	        {
		        label(&mut self.layer, ("Fat: ".into_string() + self.fat.to_string().as_slice()).as_slice() )
		        	.inner_down(SizeInCharacters(1))
		            .draw(renderer);

		        if scrollbar(&mut self.layer, SizeInCharacters(20), 0f32, 100f32, &mut self.fat_percent )
		        	.x(scrollbar_x)
		        	.color(RGB(210, 93, 90))
		        	.draw(renderer) {
		        	self.fat_percent_changed();
		        }
		    }
        }
	}

}