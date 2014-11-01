extern crate sdl2;
extern crate sdl2_ttf;

use imgui::base;
use imgui::base::SizeInCharacters;

use sdl2::pixels::RGB;

use imgui::scrollbar::scrollbar;
use imgui::label::label;
use imgui::textfield;
use imgui::textfield::textfield_f32;
use imgui::textfield::textfield_i32;
use imgui::panel::panel;
use imgui::dropdown::dropdown;
use imgui::header::header;
use imgui::button::button;

use db;
use db::RecommendedMacros;


pub struct KCalWindow<'a> {
	layer: base::Layer,

	/*weight: f32,
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
	target_calories: f32,*/
}


fn set_target_calories(rm: &mut RecommendedMacros, tc: f32) {
	rm.target_calories = tc;
	rm.protein = ((rm.protein_percent / 100f32 * rm.target_calories) / 4f32) as i32;
	rm.ch = ((rm.ch_percent / 100f32 * rm.target_calories) / 4f32) as i32;
	rm.fat = ((rm.fat_percent / 100f32 * rm.target_calories) / 9f32) as i32;
	rm.protein_per_kg = rm.protein as f32 / rm.weight;
}

fn protein_percent_changed(rm: &mut RecommendedMacros) {
	rm.ch_percent = 100f32 - rm.protein_percent - rm.fat_percent;
	rm.protein = ((rm.protein_percent / 100f32 * rm.target_calories) / 4f32) as i32;
	rm.protein_per_kg = rm.protein as f32 / rm.weight;
}

fn ch_percent_changed(rm: &mut RecommendedMacros) {
	rm.fat_percent = 100f32 - rm.protein_percent - rm.ch_percent;
    rm.ch = ((rm.ch_percent / 100f32 * rm.target_calories) / 4f32) as i32;
}

fn fat_percent_changed(rm: &mut RecommendedMacros) {
	rm.ch_percent = 100f32 - rm.protein_percent - rm.fat_percent;
	rm.fat = ((rm.fat_percent / 100f32 * rm.target_calories) / 9f32) as i32;
}

fn recalc_bmr(rm: &mut RecommendedMacros) {
	//(66 + (13,7*78 kg) + (5*190 cm) - ( 6,8*25 year)) * 1.55 activity level
	let a = 13.7f32 * rm.weight_type.to_g(rm.weight) / 1000f32;
	let height = rm.height as f32 * if rm.height_type == 1 {30.48f32} else {1f32};
	let b = 5f32 * height;
	let c = 6.8f32 * rm.age as f32;
	let d = 66f32 + a+b+c;
	let bmr = rm.activity_mod.get_modified_value(d);
	rm.bmr = bmr;
	set_target_calories(rm, bmr);
	//rm.bmr_str.clear();
	//rm.bmr_str.push_str(("BMR: ".into_string() + (rm.bmr as i32).to_string() + " kCal").as_slice());
}

impl<'a> KCalWindow<'a> {
	pub fn new() -> KCalWindow<'a> {
		KCalWindow {
			layer: base::Layer::new(),
		}
	}

	pub fn do_logic(&'a mut self, renderer: &sdl2::render::Renderer, event: &sdl2::event::Event, recommended_macros: &mut db::RecommendedMacros) -> bool {
		self.layer.handle_event(event);

		header(&mut self.layer, "BMR", SizeInCharacters(70), SizeInCharacters(22))
			.x(SizeInCharacters(6))
			.y(SizeInCharacters(5))
			.draw(renderer);
		match textfield_f32(&mut self.layer, &mut recommended_macros.weight, SizeInCharacters(4))
			.x(SizeInCharacters(7))
			.y(SizeInCharacters(7))
            .label("Mass: ")
            .draw(renderer) {
        	Some(textfield::Changed) => recalc_bmr(recommended_macros),
        	_ => {},
        };
        dropdown(&mut self.layer, vec!["g", "dkg", "kg", "lb"].as_slice(), &mut recommended_macros.weight_type)
        	.right(SizeInCharacters(1))
        	.draw(renderer);

        match textfield_i32(&mut self.layer, &mut recommended_macros.age, SizeInCharacters(4))
        	.x(SizeInCharacters(7))
        	.right(SizeInCharacters(2))
            .label("Age: ")
            .draw(renderer) {
            Some(textfield::Changed) => recalc_bmr(recommended_macros),
        	_ => {},
        };

        match textfield_i32(&mut self.layer, &mut recommended_macros.height, SizeInCharacters(4))
        	.x(SizeInCharacters(7))
        	.right(SizeInCharacters(2))
            .label("Height: ")
            .draw(renderer) {
            Some(textfield::Changed) => recalc_bmr(recommended_macros),
        	_ => {},
        };

        dropdown(&mut self.layer, vec!["cm", "ft"].as_slice(), &mut recommended_macros.height_type)
        	.right(SizeInCharacters(1))
        	.draw(renderer);

        if dropdown(&mut self.layer, vec!["Sedentary", "Lightly active", "Moderately active", "Very active", "Extremely active", ].as_slice(), &mut recommended_macros.activity_mod)
        	.x(SizeInCharacters(7))
        	.right(SizeInCharacters(1))
        	.draw(renderer) {
        	recalc_bmr(recommended_macros);
        }
        let bmr_str = format!("BMR: {:.0f}", recommended_macros.bmr);
        header(&mut self.layer, bmr_str.as_slice(), SizeInCharacters(70), SizeInCharacters(0))
			.x(SizeInCharacters(6))
			.down(SizeInCharacters(2))
			.draw(renderer);

		if recommended_macros.height > 0 && recommended_macros.weight > 0f32 && recommended_macros.age > 0 {
			dropdown(&mut self.layer, vec!["Bulking", "Cutting", ].as_slice(), &mut recommended_macros.goal_type)
	        	.x(SizeInCharacters(7))
	        	.down(SizeInCharacters(1))
	        	.draw(renderer);

	        let mut scrollbar_x = SizeInCharacters(0);
	        panel(&mut self.layer, SizeInCharacters(68), SizeInCharacters(3))
	        	.x(SizeInCharacters(7))
	        	.down(SizeInCharacters(0))
	        	.draw(renderer);
	        {

		        label(&mut self.layer, format!("Protein: {:3}", recommended_macros.protein).as_slice() )
		        	.inner_down(SizeInCharacters(1))
		            .draw(renderer);

		        if scrollbar(&mut self.layer, SizeInCharacters(20), 0f32, 100f32, &mut recommended_macros.protein_percent )
		        	.right(SizeInCharacters(2))
		        	.draw(renderer) {
		        	protein_percent_changed(recommended_macros);
		        }
		        scrollbar_x = self.layer.last_x;

		        label(&mut self.layer, "g/kg: ")
		        	.right(SizeInCharacters(2))
		            .draw(renderer);

		        if scrollbar(&mut self.layer, SizeInCharacters(6), 1f32, 4f32, &mut recommended_macros.protein_per_kg )
		        	.right(SizeInCharacters(1))
		        	.draw(renderer) {
		        	let calced_prot = recommended_macros.protein_per_kg * recommended_macros.weight;
		        	recommended_macros.protein_percent = (calced_prot*4f32) as f32 / (recommended_macros.target_calories / 100f32);
		        	protein_percent_changed(recommended_macros);
		        }
	    	}

	    	panel(&mut self.layer, SizeInCharacters(68), SizeInCharacters(3))
	        	.x(SizeInCharacters(7))
	        	.down(SizeInCharacters(1))
	        	.draw(renderer);
	        {
		        label(&mut self.layer, ("Ch: ".into_string() + recommended_macros.ch.to_string().as_slice()).as_slice() )
		        	.inner_down(SizeInCharacters(1))
		            .draw(renderer);

		        if scrollbar(&mut self.layer, SizeInCharacters(20), 0f32, 100f32, &mut recommended_macros.ch_percent )
		        	.x(scrollbar_x)
		        	.color(RGB(237, 166, 0))
		        	.draw(renderer) {
		        	ch_percent_changed(recommended_macros);
		        }
		    }

		    panel(&mut self.layer, SizeInCharacters(68), SizeInCharacters(3))
	        	.x(SizeInCharacters(7))
	        	.down(SizeInCharacters(1))
	        	.draw(renderer);
	        {
		        label(&mut self.layer, ("Fat: ".into_string() + recommended_macros.fat.to_string().as_slice()).as_slice() )
		        	.inner_down(SizeInCharacters(1))
		            .draw(renderer);

		        if scrollbar(&mut self.layer, SizeInCharacters(20), 0f32, 100f32, &mut recommended_macros.fat_percent )
		        	.x(scrollbar_x)
		        	.color(RGB(210, 93, 90))
		        	.draw(renderer) {
		        	fat_percent_changed(recommended_macros);
		        }
		    }
        }

        if button(&mut self.layer, "Save")
            .x(SizeInCharacters(10))
            .y(SizeInCharacters(25))
            .draw(renderer) {
            return true;
    	}
    	return false;
	}

}
