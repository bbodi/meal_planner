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
use db::NutritionGoal;


pub struct KCalWindow<'a> {
	pub layer: base::Layer,
}


fn set_target_calories(rm: &mut NutritionGoal, tc: f32) {
	rm.target_calories = tc;
	recalc_macronutrients(rm);
}

fn protein_percent_changed(rm: &mut NutritionGoal) {
	rm.ch_percent = 100f32 - rm.protein_percent - rm.fat_percent;
	recalc_macronutrients(rm);
}

fn ch_percent_changed(rm: &mut NutritionGoal) {
	rm.fat_percent = 100f32 - rm.protein_percent - rm.ch_percent;
	recalc_macronutrients(rm);
}

fn fat_percent_changed(rm: &mut NutritionGoal) {
	rm.ch_percent = 100f32 - rm.protein_percent - rm.fat_percent;
	recalc_macronutrients(rm);
}

fn recalc_macronutrients(rm: &mut NutritionGoal) {
	rm.macros.protein = ((rm.protein_percent / 100f32 * rm.target_calories) / 4f32);
	rm.protein_per_kg = rm.macros.protein as f32 / rm.weight;
	rm.macros.ch = ((rm.ch_percent / 100f32 * rm.target_calories) / 4f32);
	rm.macros.fat = ((rm.fat_percent / 100f32 * rm.target_calories) / 9f32);
}

fn recalc_bmr(rm: &mut NutritionGoal) {
	//(66 + (13,7*78 kg) + (5*190 cm) - ( 6,8*25 year)) * 1.55 activity level
	let a = 13.7f32 * rm.weight_type.to_g(rm.weight) / 1000f32;
	let height = rm.height as f32 * if rm.height_type == 1 {30.48f32} else {1f32};
	let b = 5f32 * height;
	let c = 6.8f32 * rm.age as f32;
	let d = 66f32 + a+b-c;
	let bmr = rm.activity_mod.get_modified_value(d);
	rm.bmr = bmr;
	set_target_calories(rm, bmr);
}

impl<'a> KCalWindow<'a> {
	pub fn new() -> KCalWindow<'a> {
		KCalWindow {
			layer: base::Layer::new(),
		}
	}

	pub fn do_logic(&'a mut self, renderer: &sdl2::render::Renderer, event: &sdl2::event::Event, nutr_goal: &mut db::NutritionGoal) -> bool {
		self.layer.handle_event(event);

		header(&mut self.layer, "BMR", SizeInCharacters(70), SizeInCharacters(22))
			.x(SizeInCharacters(6))
			.y(SizeInCharacters(5))
			.draw(renderer);
		match textfield_f32(&mut self.layer, &mut nutr_goal.weight, SizeInCharacters(4))
			.x(SizeInCharacters(7))
			.y(SizeInCharacters(7))
            .label("Mass: ")
            .draw(renderer) {
        	Some(textfield::Changed) => recalc_bmr(nutr_goal),
        	_ => {},
        };
        dropdown(&mut self.layer, vec!["g", "dkg", "kg", "lb"].as_slice(), &mut nutr_goal.weight_type)
        	.right(SizeInCharacters(1))
        	.draw(renderer);

        match textfield_i32(&mut self.layer, &mut nutr_goal.age, SizeInCharacters(4))
        	.x(SizeInCharacters(7))
        	.right(SizeInCharacters(2))
            .label("Age: ")
            .draw(renderer) {
            Some(textfield::Changed) => recalc_bmr(nutr_goal),
        	_ => {},
        };

        match textfield_i32(&mut self.layer, &mut nutr_goal.height, SizeInCharacters(4))
        	.x(SizeInCharacters(7))
        	.right(SizeInCharacters(2))
            .label("Height: ")
            .draw(renderer) {
            Some(textfield::Changed) => recalc_bmr(nutr_goal),
        	_ => {},
        };

        dropdown(&mut self.layer, vec!["cm", "ft"].as_slice(), &mut nutr_goal.height_type)
        	.right(SizeInCharacters(1))
        	.draw(renderer);

        if dropdown(&mut self.layer, vec!["Sedentary", "Lightly active", "Moderately active", "Very active", "Extremely active", ].as_slice(), &mut nutr_goal.activity_mod)
        	.x(SizeInCharacters(7))
        	.right(SizeInCharacters(1))
        	.draw(renderer) {
        	recalc_bmr(nutr_goal);
        }
        let bmr_str = format!("BMR: {:.0f}", nutr_goal.bmr);
        header(&mut self.layer, bmr_str.as_slice(), SizeInCharacters(70), SizeInCharacters(0))
			.x(SizeInCharacters(6))
			.down(SizeInCharacters(2))
			.draw(renderer);

		if nutr_goal.height > 0 && nutr_goal.weight > 0f32 && nutr_goal.age > 0 {
			dropdown(&mut self.layer, vec!["Bulking", "Cutting", ].as_slice(), &mut nutr_goal.goal_type)
	        	.x(SizeInCharacters(7))
	        	.down(SizeInCharacters(1))
	        	.draw(renderer);

	        let mut scrollbar_x = SizeInCharacters(0);
	        panel(&mut self.layer, SizeInCharacters(68), SizeInCharacters(3))
	        	.x(SizeInCharacters(7))
	        	.down(SizeInCharacters(0))
	        	.draw();
	        {

		        label(&mut self.layer, format!("Protein: {:3.0f}", nutr_goal.macros.protein).as_slice() )
		        	.inner_down(SizeInCharacters(1))
		            .draw();

		        if scrollbar(&mut self.layer, SizeInCharacters(20), 0f32, 100f32, &mut nutr_goal.protein_percent )
		        	.right(SizeInCharacters(2))
		        	.draw(renderer) {
		        	protein_percent_changed(nutr_goal);
		        }
		        scrollbar_x = self.layer.last_x;

		        label(&mut self.layer, "g/kg: ")
		        	.right(SizeInCharacters(2))
		            .draw();

		        if scrollbar(&mut self.layer, SizeInCharacters(6), 1f32, 4f32, &mut nutr_goal.protein_per_kg )
		        	.right(SizeInCharacters(1))
		        	.draw(renderer) {
		        	let calced_prot = nutr_goal.protein_per_kg * nutr_goal.weight;
		        	nutr_goal.protein_percent = (calced_prot*4f32) as f32 / (nutr_goal.target_calories / 100f32);
		        	protein_percent_changed(nutr_goal);
		        }
	    	}

	    	panel(&mut self.layer, SizeInCharacters(68), SizeInCharacters(3))
	        	.x(SizeInCharacters(7))
	        	.down(SizeInCharacters(1))
	        	.draw();
	        {
		        label(&mut self.layer, format!("Ch: {:3.0f}", nutr_goal.macros.ch).as_slice() )
		        	.inner_down(SizeInCharacters(1))
		            .draw();

		        if scrollbar(&mut self.layer, SizeInCharacters(20), 0f32, 100f32, &mut nutr_goal.ch_percent )
		        	.x(scrollbar_x)
		        	.color(RGB(237, 166, 0))
		        	.draw(renderer) {
		        	ch_percent_changed(nutr_goal);
		        }
		    }

		    panel(&mut self.layer, SizeInCharacters(68), SizeInCharacters(3))
	        	.x(SizeInCharacters(7))
	        	.down(SizeInCharacters(1))
	        	.draw();
	        {
		        label(&mut self.layer, format!("Fat: {:3.0f}", nutr_goal.macros.fat).as_slice() )
		        	.inner_down(SizeInCharacters(1))
		            .draw();

		        if scrollbar(&mut self.layer, SizeInCharacters(20), 0f32, 100f32, &mut nutr_goal.fat_percent )
		        	.x(scrollbar_x)
		        	.color(RGB(210, 93, 90))
		        	.draw(renderer) {
		        	fat_percent_changed(nutr_goal);
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
