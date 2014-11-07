extern crate sdl2;
extern crate sdl2_ttf;

use imgui::base;
use imgui::base::SizeInCharacters;

use sdl2::pixels::RGB;

use imgui::label::label;
use imgui::textfield::textfield_f32;
use imgui::textfield::textfield_str;
use imgui::textfield;
use tricolor_field::tricolor_field_str;
use imgui::button::button;
use imgui::header::header;
use imgui::checkbox::checkbox;
use imgui::dropdown::dropdown;
use tricolor_label::tricolor_label;
use db;
use db::DailyMenu;



pub struct DailyPlan<'a> {
    pub layer: base::Layer,

    page: uint,
    selected_meal: uint,
    selected_food_type: db::FoodType,
}

impl<'a> DailyPlan<'a> {

    pub fn new() -> DailyPlan<'a> {
        DailyPlan {
            layer: base::Layer::new(),
            page: 0,
            selected_meal: 0,
            selected_food_type: db::Meat,
        }
    }

    pub fn do_logic(&mut self, event: &sdl2::event::Event, foods: &[db::Food], daily_menu: &mut DailyMenu, nutr_goal: &db::NutritionGoal, last_meal_id: &mut uint) -> bool {
        self.layer.handle_event(event);

        if daily_menu.meals.len() == 0 {
            *last_meal_id = *last_meal_id + 1;
            daily_menu.add_new_meal(*last_meal_id);
        }

        let column_height = SizeInCharacters(36);
        header(&mut self.layer, "Foods", SizeInCharacters(22), column_height)
            .x(SizeInCharacters(85))
            .y(SizeInCharacters(1))
            .draw();
        let first_row = self.layer.last_x + SizeInCharacters(1);

        dropdown(&mut self.layer, db::FoodType::names(), &mut self.selected_food_type)
            .down(SizeInCharacters(1))
            .x(first_row)
            .draw();

        let can_add_row = daily_menu.meals[self.selected_meal].foods.len() < 9;
        let selected_food_type = self.selected_food_type;
        for (_, food) in foods.iter()
            .filter(|x| x.food_type == selected_food_type)
            .skip(self.page * 16)
            .take((self.page+1) * 16).enumerate() {
            let fs = food.weight_type.to_g(food.weight);
            let values = (food.protein, food.ch, food.fat, fs);
            tricolor_label(label(&mut self.layer, food.name.as_slice())
                .x(first_row)
                .down(SizeInCharacters(1)), values, SizeInCharacters(20)).draw();

            if can_add_row && button(&mut self.layer, "+").right(SizeInCharacters(2)).draw() {
                daily_menu.meals[self.selected_meal].add_food(food.id);
            }
        }
        if button(&mut self.layer, "Prev")
            .disabled(self.page == 0)
            .x(first_row)
            .y(SizeInCharacters(37))
            .draw() {
            self.page = self.page - 1;
        }
        if button(&mut self.layer, "Next")
            .disabled(self.page >= (foods.len() / 16))
            .right(SizeInCharacters(10))
            .draw() {
            self.page = self.page + 1;
        }
        if button(&mut self.layer, "Save")
            .down(SizeInCharacters(1))
            .x(first_row).draw() {
            return true;
        }

        textfield_str(&mut self.layer, &mut daily_menu.name, SizeInCharacters(20))
            .x(SizeInCharacters(1))
            .y(SizeInCharacters(1))
            .default_text("Daily Menu name...")
            .draw();
        
        self.draw_meals_table(foods, daily_menu, last_meal_id);
        self.draw_meal_foods_table(foods, daily_menu);
        self.draw_sum_table(foods, daily_menu, nutr_goal);
        return false;
    }

    fn draw_sum_table(&mut self, foods: &[db::Food], daily_menu: &mut DailyMenu, nutr_goal: &db::NutritionGoal) {
        let mut daily_macros = db::MacroNutrient::new(0f32, 0f32, 0f32);
        let mut daily_weight = 0f32;
        for meal in daily_menu.meals.iter() {
            let (meal_macros, w) = DailyPlan::calc_meal_macro(foods, meal);
            daily_macros = daily_macros + meal_macros;
            daily_weight = daily_weight + w;
        }
        header(&mut self.layer, "Sum", SizeInCharacters(50), SizeInCharacters(5) )
            .x(SizeInCharacters(1))
            .y(SizeInCharacters(35))
            .draw_with_body(|layer| {
            let start_x = layer.last_x;
            let start_y = layer.last_y;

            let goal_w = nutr_goal.macros.protein + nutr_goal.macros.ch + nutr_goal.macros.fat;
            let values = (nutr_goal.macros.protein, nutr_goal.macros.ch, nutr_goal.macros.fat, goal_w);
            tricolor_label(label(layer, "")  
                .right(SizeInCharacters(13))
                .down(SizeInCharacters(1)), values, SizeInCharacters(21)).draw();

            let values = (daily_macros.protein, daily_macros.ch, daily_macros.fat, goal_w);
            tricolor_label(label(layer, "")  
                .down(SizeInCharacters(0)), values, SizeInCharacters(21)).draw();

            let values = (nutr_goal.macros.protein - daily_macros.protein, nutr_goal.macros.ch - daily_macros.ch, nutr_goal.macros.fat - daily_macros.fat, goal_w);
            tricolor_label(label(layer, "")  
                .down(SizeInCharacters(0)), values, SizeInCharacters(21)).draw();

            label(layer, "Current")
                .y(start_y+SizeInCharacters(3))
                .x(start_x + SizeInCharacters(1))
                .draw();
            label(layer, "Diff")
                .down(SizeInCharacters(0))
                .draw();
            label(layer, "Recommended")
                .up(SizeInCharacters(2))
                .draw();
            header(layer, "P", SizeInCharacters(5), SizeInCharacters(4))
                .up(SizeInCharacters(1))
                .right(SizeInCharacters(1))
                .color(RGB(76, 166, 79))
                .draw_with_body(|layer| {
                label(layer, format!("{: ^5.0f}", nutr_goal.macros.protein).as_slice())
                    .color(RGB(0, 0, 0))
                    .down(SizeInCharacters(0))
                    .draw();
                label(layer, format!("{: ^5.0f}", daily_macros.protein).as_slice())
                    .bold(true)
                    .color(RGB(0, 0, 0))
                    .down(SizeInCharacters(0))
                    .draw();
                label(layer, format!("{: ^5.0f}", nutr_goal.macros.protein - daily_macros.protein).as_slice())
                    .color(RGB(0, 0, 0))           
                    .down(SizeInCharacters(0))
                    .draw();
            });
            header(layer, "Ch", SizeInCharacters(5), SizeInCharacters(4))
                .right(SizeInCharacters(0))
                .color(RGB(237, 166, 0))
                .draw_with_body(|layer| {
                label(layer, format!("{: ^5.0f}", nutr_goal.macros.ch).as_slice())
                    .color(RGB(0, 0, 0))
                    .down(SizeInCharacters(0))
                    .draw();
                label(layer, format!("{: ^5.0f}", daily_macros.ch).as_slice())
                    .bold(true)
                    .color(RGB(0, 0, 0))
                    .down(SizeInCharacters(0))
                    .draw();
                label(layer, format!("{: ^5.0f}", nutr_goal.macros.ch - daily_macros.ch).as_slice())
                    .color(RGB(0, 0, 0))
                    .down(SizeInCharacters(0))
                    .draw();
            });
            header(layer, "F", SizeInCharacters(5), SizeInCharacters(4))
                .right(SizeInCharacters(0))
                .color(RGB(210, 93, 90))
                .draw_with_body(|layer| {
                label(layer, format!("{: ^5.0f}", nutr_goal.macros.fat).as_slice())
                    .color(RGB(0, 0, 0))
                    .down(SizeInCharacters(0))
                    .draw();
                label(layer, format!("{: ^5.0f}", daily_macros.fat).as_slice())
                    .bold(true)
                    .color(RGB(0, 0, 0))
                    .down(SizeInCharacters(0))
                    .draw();
                label(layer, format!("{: ^5.0f}", nutr_goal.macros.fat - daily_macros.fat).as_slice())
                    .color(RGB(0, 0, 0))
                    .down(SizeInCharacters(0))
                    .draw();
            });
            header(layer, "kCal", SizeInCharacters(6), SizeInCharacters(4))
                .right(SizeInCharacters(0))
                .draw_with_body(|layer| {
                label(layer, format!("{: ^6.0f}", nutr_goal.macros.kcal()).as_slice())
                    .color(RGB(0, 0, 0))
                    .down(SizeInCharacters(0))
                    .draw();
                label(layer, format!("{: ^6.0f}", daily_macros.kcal()).as_slice())
                    .bold(true)
                    .color(RGB(0, 0, 0))
                    .down(SizeInCharacters(0))
                    .draw();
                label(layer, format!("{: ^6.0f}", nutr_goal.macros.kcal() - daily_macros.kcal()).as_slice())
                    .color(RGB(0, 0, 0))
                    .down(SizeInCharacters(0))
                    .draw();
            });
            header(layer, "Price", SizeInCharacters(6), SizeInCharacters(4))
                .right(SizeInCharacters(0))
                .draw_with_body(|layer| {
                label(layer, format!("{: ^6}", daily_menu.price(foods)).as_slice())
                    .bold(true)
                    .color(RGB(0, 0, 0))
                    .down(SizeInCharacters(1))
                    .draw();
            });
        });
    }

    fn draw_meals_table(&mut self, foods: &[db::Food], daily_menu: &mut DailyMenu, last_meal_id: &mut uint) {
        let mut meals_menu_y = SizeInCharacters(0);
        let mut selected_meal = self.selected_meal;
        let mut delete_idx = None;
        let meal_count = daily_menu.meals.len();
        let mut move_up_idx = None;
        let mut move_down_idx = None;
        let mut copy_idx = None;
        let meals_panel_width = SizeInCharacters(33);
        header(&mut self.layer, "Meals", meals_panel_width, SizeInCharacters(4 + (daily_menu.meals.len() as i32*5) ))
            .down(SizeInCharacters(1))
            .draw_with_body(|layer| {
            meals_menu_y = layer.last_y;
            let meal_checkbox_x = layer.last_x + SizeInCharacters(1);
            for (i, meal) in daily_menu.meals.iter_mut().enumerate() {
                let mut checkbox_value = selected_meal == i;
                let mut meal_was_selected = false;
                if checkbox(layer, &mut checkbox_value)
                    .x(meal_checkbox_x)
                    .down(SizeInCharacters(2))
                    .draw() && checkbox_value {
                    meal_was_selected = true;
                }
                let (macros, w) = DailyPlan::calc_meal_macro(foods, meal);
                //let mut only_macro_weight = macros.protein + macros.ch + macros.fat;
                match tricolor_field_str(textfield_str(layer, &mut meal.name, SizeInCharacters(27))
                    .right(SizeInCharacters(1))
                    .up(SizeInCharacters(1))
                    .default_text("Meal name..."), (macros.protein, macros.ch, macros.fat, w) )
                    .draw() {
                    Some(textfield::Selected) => meal_was_selected = true,
                    _ => {},
                }
                header(layer, "P", SizeInCharacters(5), SizeInCharacters(2))
                    .down(SizeInCharacters(0))
                    .x(meal_checkbox_x + SizeInCharacters(2))
                    .color(RGB(76, 166, 79))
                    .draw_with_body(|layer| {
                    label(layer, format!("{: >5.0f}", macros.protein).as_slice())
                        .down(SizeInCharacters(0))
                        .draw();
                });
                header(layer, "Ch", SizeInCharacters(5), SizeInCharacters(2))
                    .right(SizeInCharacters(0))
                    .color(RGB(237, 166, 0))
                    .draw_with_body(|layer| {
                    label(layer, format!("{: >5.0f}", macros.ch).as_slice())
                        .down(SizeInCharacters(0))
                        .draw();
                });
                header(layer, "F", SizeInCharacters(5), SizeInCharacters(2))
                    .right(SizeInCharacters(0))
                    .color(RGB(210, 93, 90))
                    .draw_with_body(|layer| {
                    label(layer, format!("{: >5.0f}", macros.fat).as_slice())
                        .down(SizeInCharacters(0))
                        .draw();
                });
                header(layer, "kCal", SizeInCharacters(6), SizeInCharacters(2))
                    .right(SizeInCharacters(0))
                    .draw_with_body(|layer| {
                    label(layer, format!("{: >6.0f}", macros.kcal()).as_slice())
                        .down(SizeInCharacters(0))
                        .draw();
                });
                header(layer, "Price", SizeInCharacters(6), SizeInCharacters(2))
                    .right(SizeInCharacters(0))
                    .draw_with_body(|layer| {
                    label(layer, format!("{: >6}", meal.price(foods)).as_slice())
                        .down(SizeInCharacters(0))
                        .draw();
                });
                if button(layer, "▲")
                    .disabled(i == 0)
                    .right(SizeInCharacters(0))
                    .up(SizeInCharacters(1))
                    .draw() {
                    move_up_idx = Some(i);
                }
                if button(layer, "C")
                    .down(SizeInCharacters(0))
                    .draw() {
                    copy_idx = Some(i);
                }
                if button(layer, "-")
                    .disabled(meal_count <= 1)
                    .down(SizeInCharacters(0))
                    .draw() {
                    delete_idx = Some(i);
                }
                if button(layer, "▼")
                    .disabled(i >= meal_count-1)
                    .down(SizeInCharacters(0))
                    .draw() {
                    move_down_idx = Some(i);
                }
                let hori_line_x = (meal_checkbox_x-SizeInCharacters(1)).in_pixels(layer.char_w);
                let hori_line_y = (layer.last_y + layer.last_h).in_pixels(layer.char_h);
                let char_w = layer.char_w;
                layer.bottom_surface.draw_line(hori_line_x, hori_line_y, hori_line_x+meals_panel_width.in_pixels(char_w), hori_line_y, RGB(0, 0, 0));

                if meal_was_selected {
                    selected_meal = i;
                }
                if selected_meal == i {
                    let food_count = meal.foods.len() as i32;
                    let table_height = SizeInCharacters(3+food_count *4);
                    let line_x1 = (layer.last_x + layer.last_w + SizeInCharacters(2)).in_pixels(layer.char_w);
                    let line_y1 = (layer.last_y).in_pixels(layer.char_h);
                    let line_x2 = line_x1 + SizeInCharacters(4).in_pixels(layer.char_w);
                    let line_y2 = meals_menu_y.in_pixels(layer.char_h);
                    let row = layer.char_h;
                    let color = RGB(0, 0, 0);
                    layer.bottom_surface.draw_line(line_x1, line_y1-3*row, line_x2, line_y2, color);
                    let char_h = layer.char_h;
                    layer.bottom_surface.draw_line(line_x1, line_y1+row, line_x2, line_y2+table_height.in_pixels(char_h), color);
                }
            }
            
            if button(layer, "Add new")
                .x(meal_checkbox_x)
                .down(SizeInCharacters(1))
                .draw() {
                *last_meal_id = *last_meal_id + 1;
                daily_menu.add_new_meal(*last_meal_id);
            }

            if delete_idx.is_some() {
                let i = delete_idx.unwrap();
                daily_menu.meals.remove(i);
                layer.clear_textfield_states();
            } else if move_up_idx.is_some() {
                let i = move_up_idx.unwrap();
                let moved_meal = daily_menu.meals.remove(i);
                daily_menu.meals.insert(i-1, moved_meal.unwrap());
                layer.clear_textfield_states();
            } else if move_down_idx.is_some() {
                let i = move_down_idx.unwrap();
                let moved_meal = daily_menu.meals.remove(i);
                daily_menu.meals.insert(i+1, moved_meal.unwrap());
                layer.clear_textfield_states();
            } else if copy_idx.is_some() {
                let i = copy_idx.unwrap();
                *last_meal_id = *last_meal_id + 1;
                let new_meal = db::Meal::from_meal(*last_meal_id, &daily_menu.meals[i]);
                daily_menu.add_meal(new_meal);
                layer.clear_textfield_states();
            }
        });
        if selected_meal >= daily_menu.meals.len() {
            selected_meal = daily_menu.meals.len()-1;
        }
        self.selected_meal = selected_meal;
    }

    fn draw_meal_foods_table(&mut self, foods: &[db::Food], daily_menu: &mut DailyMenu) {
        let food_count = daily_menu.meals[self.selected_meal].foods.len() as i32;
        let table_height = SizeInCharacters(3+food_count *4);
        let selected_meal = self.selected_meal;
        let head_name = daily_menu.meals[self.selected_meal].name.clone();
        
        let mut deleting_index = None;
        header(&mut self.layer, head_name.as_slice(), SizeInCharacters(42), table_height) 
            .right(SizeInCharacters(4))
            .bold(true)
            .draw_with_body(|layer| {

            header(layer, "Name", SizeInCharacters(30), table_height - SizeInCharacters(1))
                .down(SizeInCharacters(0))
                .draw();
            let name_column_index = layer.last_x + SizeInCharacters(1);
            header(layer, "Tömeg", SizeInCharacters(12), table_height - SizeInCharacters(1))
                .right(SizeInCharacters(0))
                .draw();
            let weight_column_index = layer.last_x + SizeInCharacters(1);
            if food_count == 0 {
                return;
            }
            let meal = &mut daily_menu.meals[selected_meal];
            let (meal_macros, _) = DailyPlan::calc_meal_macro(foods, meal);
            for (i, meal_food) in meal.foods.iter_mut().enumerate() {
                let ref food = foods[meal_food.food_id-1];
                let fs = food.weight_type.to_g(food.weight);
                let only_macro_weight = meal_macros.protein + meal_macros.ch + meal_macros.fat;
                let meal_food_macros = DailyPlan::calc_meal_food_macro(foods, meal_food);
                let values = (meal_food_macros.protein, meal_food_macros.ch, meal_food_macros.fat, only_macro_weight);
                tricolor_label(label(layer, food.name.as_slice())
                    .x(name_column_index)
                    .down(SizeInCharacters(1)), values, SizeInCharacters(27)).draw();

                
                let macros = DailyPlan::calc_meal_food_macro(foods, meal_food);

                match textfield_f32(layer, &mut meal_food.weight, SizeInCharacters(5))
                    .x(weight_column_index)
                    .draw() {
                    Some(textfield::Changed) => {},
                    _ => {},
                }
                dropdown(layer, vec!["g", "dkg", "kg"].as_slice(), &mut meal_food.weight_type)
                    .right(SizeInCharacters(1))
                    .draw();
                if button(layer, "-")
                    .right(SizeInCharacters(2))
                    .draw() {
                    deleting_index = Some(i);
                }
                header(layer, "P", SizeInCharacters(5), SizeInCharacters(2))
                    .down(SizeInCharacters(0))
                    .x(name_column_index + SizeInCharacters(0))
                    .color(RGB(76, 166, 79))
                    .draw_with_body(|layer| {
                    label(layer, format!("{: >5.0f}", macros.protein).as_slice())
                        .down(SizeInCharacters(0))
                        .draw();
                });
                header(layer, "Ch", SizeInCharacters(5), SizeInCharacters(2))
                    .right(SizeInCharacters(0))
                    .color(RGB(237, 166, 0))
                    .draw_with_body(|layer| {
                    label(layer, format!("{: >5.0f}", macros.ch).as_slice())
                        .down(SizeInCharacters(0))
                        .draw();
                });
                header(layer, "F", SizeInCharacters(5), SizeInCharacters(2))
                    .right(SizeInCharacters(0))
                    .color(RGB(210, 93, 90))
                    .draw_with_body(|layer| {
                    label(layer, format!("{: >5.0f}", macros.fat).as_slice())
                        .down(SizeInCharacters(0))
                        .draw();
                });
                header(layer, "kCal", SizeInCharacters(6), SizeInCharacters(2))
                    .right(SizeInCharacters(0))
                    .draw_with_body(|layer| {
                    label(layer, format!("{: >6.0f}", macros.kcal()).as_slice())
                        .down(SizeInCharacters(0))
                        .draw();
                });
                header(layer, "Price", SizeInCharacters(6), SizeInCharacters(2))
                    .right(SizeInCharacters(0))
                    .draw_with_body(|layer| {
                    label(layer, format!("{: >6}", meal_food.price(foods)).as_slice())
                        .down(SizeInCharacters(0))
                        .draw();
                });
            }
        });
        if deleting_index.is_some() {
            daily_menu.meals[selected_meal].foods.remove(deleting_index.unwrap());
            self.layer.clear_textfield_states();
        }
    }

    fn calc_meal_food_macro(foods: &[db::Food], meal_food: &db::MealFood) -> db::MacroNutrient {
        let ref food = foods[meal_food.food_id-1];

        let standard_weight = food.weight_type.to_g(food.weight);
        let input_weight =  meal_food.weight_type.to_g(meal_food.weight);
        let ratio = input_weight / standard_weight;
        let mut macros = db::MacroNutrient::new(0f32, 0f32, 0f32);
        macros.protein = macros.protein + food.protein * ratio;
        macros.ch = macros.ch + food.ch * ratio;
        macros.fat = macros.fat + food.fat * ratio;
        macros
    }

    fn calc_meal_macro(foods: &[db::Food], meal: &db::Meal) -> (db::MacroNutrient, f32) {
        let mut macros = db::MacroNutrient::new(0f32, 0f32, 0f32);
        let mut w = 0f32;
        for meal_food in meal.foods.iter() {
            let ref food = foods[meal_food.food_id-1];
            let standard_weight = food.weight_type.to_g(food.weight);
            let input_weight =  meal_food.weight_type.to_g(meal_food.weight);
            let ratio = input_weight / standard_weight;
            macros.protein = macros.protein + food.protein * ratio;
            macros.ch = macros.ch + food.ch * ratio;
            macros.fat = macros.fat + food.fat * ratio;
            w = w + input_weight;
        }
        return (macros, w);
    }

}
