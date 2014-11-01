extern crate sdl2;
extern crate sdl2_ttf;

use imgui::base;
use imgui::base::SizeInCharacters;

use sdl2::pixels::RGB;

use imgui::label::label;
use imgui::textfield::textfield_f32;
use imgui::textfield::textfield_i32;
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
    last_meal_id: &'a mut uint,
    last_meal_food_id: &'a mut uint,
}

impl<'a> DailyPlan<'a> {

    pub fn new(last_meal_id: &'a mut uint, last_meal_food_id: &'a mut uint) -> DailyPlan<'a> {
        DailyPlan {
            layer: base::Layer::new(),
            page: 0,
            selected_meal: 0,
            last_meal_id: last_meal_id, 
            last_meal_food_id: last_meal_food_id
        }
    }

    pub fn do_logic(&mut self, renderer: &sdl2::render::Renderer, event: &sdl2::event::Event, foods: &Vec<db::Food>, daily_menu: &mut DailyMenu) -> bool {
        self.layer.handle_event(event);

        if daily_menu.meals.len() == 0 {
            *self.last_meal_id = *self.last_meal_id + 1;
            daily_menu.meals.push(db::Meal::new(*self.last_meal_id));
        }

        let column_height = SizeInCharacters(36);
        header(&mut self.layer, "Foods", SizeInCharacters(22), column_height)
            .x(SizeInCharacters(72))
            .y(SizeInCharacters(1))
            .draw(renderer);
        let price_header_x = self.layer.last_x + self.layer.last_w;
        let first_row = self.layer.last_x + SizeInCharacters(1);
        let can_add_row = daily_menu.meals[self.selected_meal].foods.len() < 7;
        for (_, food) in foods.iter().skip(self.page * 16).take((self.page+1) * 16).enumerate() {
            let fs = food.weight_type.to_g(food.weight);
            let values = (food.protein, food.ch, food.fat, fs);
            tricolor_label(label(&mut self.layer, food.name.as_slice())
                .x(first_row)
                .down(SizeInCharacters(1)), values, SizeInCharacters(20)).draw(renderer);

            if can_add_row && button(&mut self.layer, "+").right(SizeInCharacters(2)).draw(renderer) {
                *self.last_meal_food_id = *self.last_meal_food_id + 1;
                daily_menu.meals[self.selected_meal].foods.push(db::MealFood::new(*self.last_meal_food_id, food.id));
            }
        }
        if button(&mut self.layer, "Prev")
            .disabled(self.page == 0)
            .x(first_row)
            .y(SizeInCharacters(35))
            .draw(renderer) {
            self.page = self.page - 1;
        }
        if button(&mut self.layer, "Next")
            .disabled(self.page >= (foods.len() / 16))
            .right(SizeInCharacters(10))
            .draw(renderer) {
            self.page = self.page + 1;
        }
        if button(&mut self.layer, "Save")
            .down(SizeInCharacters(1))
            .x(first_row).draw(renderer) {
            return true;
        }

        textfield_str(&mut self.layer, &mut daily_menu.name, SizeInCharacters(20))
            .x(SizeInCharacters(1))
            .y(SizeInCharacters(2))
            .default_text("Daily Menu name...")
            .draw(renderer);
        
        let meals_menu_y = self.draw_meals_table(renderer, foods, daily_menu);
        self.draw_meal_foods_table(renderer, foods, daily_menu, meals_menu_y);
        self.draw_sum_table(renderer, foods, daily_menu);
        return false;
    }

    fn draw_sum_table(&mut self, renderer: &sdl2::render::Renderer, foods: &Vec<db::Food>, daily_menu: &mut DailyMenu) {
        header(&mut self.layer, "Sum", SizeInCharacters(50), SizeInCharacters(4 + (daily_menu.meals.len() as i32*4) ))
            .x(SizeInCharacters(1))
            .y(SizeInCharacters(30))
            .draw_with_body(renderer, |layer| {
            //let (p, ch, f, w) = DailyPlan::calc_macro_ratio(foods, meal);
            let (p, ch, f, w) = (1f32, 1f32, 1f32, 1f32);
            label(layer, "Current")
                .down(SizeInCharacters(2))
                .right(SizeInCharacters(1))
                .draw(renderer);
            label(layer, "Diff")
                .down(SizeInCharacters(0))
                .draw(renderer);
            label(layer, "Recommended")
                .up(SizeInCharacters(2))
                .draw(renderer);
            header(layer, "P", SizeInCharacters(5), SizeInCharacters(3))
                .right(SizeInCharacters(1))
                .color(RGB(76, 166, 79))
                .draw_with_body(renderer, |layer| {
                label(layer, format!("{: ^5.0f}", p).as_slice())
                    .down(SizeInCharacters(0))
                    .draw(renderer);
                label(layer, format!("{: ^5.0f}", p).as_slice())
                    .down(SizeInCharacters(0))
                    .draw(renderer);
            });
            header(layer, "Ch", SizeInCharacters(5), SizeInCharacters(3))
                .right(SizeInCharacters(0))
                .color(RGB(237, 166, 0))
                .draw_with_body(renderer, |layer| {
                label(layer, format!("{: ^5.0f}", ch).as_slice())
                    .down(SizeInCharacters(0))
                    .draw(renderer);
                label(layer, format!("{: ^5.0f}", ch).as_slice())
                    .down(SizeInCharacters(0))
                    .draw(renderer);
            });
            header(layer, "F", SizeInCharacters(5), SizeInCharacters(3))
                .right(SizeInCharacters(0))
                .color(RGB(210, 93, 90))
                .draw_with_body(renderer, |layer| {
                label(layer, format!("{: ^5.0f}", f).as_slice())
                    .down(SizeInCharacters(0))
                    .draw(renderer);
                label(layer, format!("{: ^5.0f}", f).as_slice())
                    .down(SizeInCharacters(0))
                    .draw(renderer);
            });
            header(layer, "kCal", SizeInCharacters(5), SizeInCharacters(3))
                .right(SizeInCharacters(0))
                //.color(RGB(210, 93, 90))
                .draw_with_body(renderer, |layer| {
                label(layer, format!("{: ^5.0f}", p*4f32+ch*4f32+f*9f32).as_slice())
                    .down(SizeInCharacters(0))
                    .draw(renderer);
                label(layer, format!("{: ^5.0f}", p*4f32+ch*4f32+f*9f32).as_slice())
                    .down(SizeInCharacters(0))
                    .draw(renderer);
            });        
        });
    }

    fn draw_meals_table(&mut self, renderer: &sdl2::render::Renderer, foods: &Vec<db::Food>, daily_menu: &mut DailyMenu) -> SizeInCharacters{
        let mut meals_menu_y = SizeInCharacters(0);
        let mut selected_meal = self.selected_meal;
        let mut last_meal_id = *self.last_meal_id;
        header(&mut self.layer, "Meals", SizeInCharacters(24), SizeInCharacters(4 + (daily_menu.meals.len() as i32*4) ))
            .down(SizeInCharacters(1))
            .draw_with_body(renderer, |layer| {
            meals_menu_y = layer.last_y;
            let meal_checkbox_x = layer.last_x + SizeInCharacters(1);
            for (i, meal) in daily_menu.meals.iter_mut().enumerate() {
                let mut checkbox_value = selected_meal == i;
                let mut meal_was_selected = false;
                if checkbox(layer, &mut checkbox_value)
                    .x(meal_checkbox_x)
                    .down(SizeInCharacters(2))
                    .draw(renderer) && checkbox_value {
                    meal_was_selected = true;
                }
                let macro_ratio = DailyPlan::calc_macro_ratio(foods, meal);
                match tricolor_field_str(textfield_str(layer, &mut meal.name, SizeInCharacters(20))
                    .right(SizeInCharacters(1))
                    .up(SizeInCharacters(1))
                    .default_text("Meal name..."), macro_ratio)
                    .draw(renderer) {
                    Some(textfield::Selected) => meal_was_selected = true,
                    _ => {},
                }
                let (p, ch, f, w) = DailyPlan::calc_macro_ratio(foods, meal);
                header(layer, "P", SizeInCharacters(5), SizeInCharacters(2))
                    .x(meal_checkbox_x + SizeInCharacters(2))
                    .down(SizeInCharacters(0))
                    .color(RGB(76, 166, 79))
                    .draw_with_body(renderer, |layer| {
                    label(layer, format!("{: ^5.0f}", p).as_slice())
                        .down(SizeInCharacters(0))
                        .draw(renderer);
                });
                header(layer, "Ch", SizeInCharacters(5), SizeInCharacters(2))
                    .right(SizeInCharacters(0))
                    .color(RGB(237, 166, 0))
                    .draw_with_body(renderer, |layer| {
                    label(layer, format!("{: ^5.0f}", ch).as_slice())
                        .down(SizeInCharacters(0))
                        .draw(renderer);
                });
                header(layer, "F", SizeInCharacters(5), SizeInCharacters(2))
                    .right(SizeInCharacters(0))
                    .color(RGB(210, 93, 90))
                    .draw_with_body(renderer, |layer| {
                    label(layer, format!("{: ^5.0f}", f).as_slice())
                        .down(SizeInCharacters(0))
                        .draw(renderer);
                });
                header(layer, "kCal", SizeInCharacters(5), SizeInCharacters(2))
                    .right(SizeInCharacters(0))
                    //.color(RGB(210, 93, 90))
                    .draw_with_body(renderer, |layer| {
                    label(layer, format!("{: ^5.0f}", p*4f32+ch*4f32+f*9f32).as_slice())
                        .down(SizeInCharacters(0))
                        .draw(renderer);
                });
                let hori_line_x = (meal_checkbox_x-SizeInCharacters(1)).in_pixels(layer.char_w);
                let hori_line_y = (layer.last_y + layer.last_h).in_pixels(layer.char_h) + layer.char_h/2;
                base::draw_line(renderer, hori_line_x, hori_line_y, hori_line_x+SizeInCharacters(24).in_pixels(layer.char_w), hori_line_y, RGB(0, 0, 0));

                if meal_was_selected {
                    selected_meal = i;
                }
                if selected_meal == i {
                    let food_count = meal.foods.len() as i32;
                    let table_height = SizeInCharacters(3+food_count *2);
                    let line_x1 = (layer.last_x + layer.last_w + SizeInCharacters(1)).in_pixels(layer.char_w);
                    let line_y1 = (layer.last_y).in_pixels(layer.char_h);
                    let line_x2 = line_x1 + SizeInCharacters(4).in_pixels(layer.char_w);
                    let line_y2 = meals_menu_y.in_pixels(layer.char_h);
                    let one_and_half_row = (layer.char_h + layer.char_h/2);
                    let color = RGB(0, 0, 0);
                    base::draw_line(renderer, line_x1, line_y1-one_and_half_row, line_x2, line_y2, color);
                    base::draw_line(renderer, line_x1, line_y1+layer.char_h+one_and_half_row, line_x2, line_y2+table_height.in_pixels(layer.char_h), color);
                }
            }
            if button(layer, "Add new")
                .x(meal_checkbox_x)
                .down(SizeInCharacters(1))
                .draw(renderer) {
                last_meal_id = last_meal_id + 1;
                daily_menu.meals.push(db::Meal::new(last_meal_id));
            }
            if button(layer, "Delete last")
                .right(SizeInCharacters(2))
                .draw(renderer) {
                let _ = daily_menu.meals.pop();
            }
        });
        *self.last_meal_id = last_meal_id;
        self.selected_meal = selected_meal;
        return meals_menu_y;
    }

    fn draw_meal_foods_table(&mut self, renderer: &sdl2::render::Renderer, foods: &Vec<db::Food>, daily_menu: &mut DailyMenu, meals_menu_y: SizeInCharacters) {
        let food_count = daily_menu.meals[self.selected_meal].foods.len() as i32;
        let table_height = SizeInCharacters(3+food_count *2);
        let selected_meal = self.selected_meal;
        let head_name = daily_menu.meals[self.selected_meal].name.clone();
        header(&mut self.layer, head_name.as_slice(), SizeInCharacters(39), table_height) 
            .right(SizeInCharacters(4))
            .bold(true)
            .draw_with_body(renderer, |layer| {
            header(layer, "Name", SizeInCharacters(22), table_height - SizeInCharacters(1))
                .down(SizeInCharacters(0))
                .draw(renderer);
            let name_column_index = layer.last_x + SizeInCharacters(1);
            header(layer, "Tömeg", SizeInCharacters(12), table_height - SizeInCharacters(1))
                .right(SizeInCharacters(0))
                .draw(renderer);
            let weight_column_index = layer.last_x + SizeInCharacters(1);
            if food_count == 0 {
                return;
            }        
            let mut meal = daily_menu.meals.get_mut(selected_meal);
            let mut deleting_index = None;
            for (i, meal_food) in meal.foods.iter_mut().enumerate() {
                let ref food = foods[meal_food.food_id-1];
                let fs = food.weight_type.to_g(food.weight);
                let values = (food.protein, food.ch, food.fat, fs);
                tricolor_label(label(layer, food.name.as_slice())
                    .x(name_column_index)
                    .down(SizeInCharacters(1)), values, SizeInCharacters(20)).draw(renderer);

                match textfield_f32(layer, &mut meal_food.weight, SizeInCharacters(4))
                    .x(weight_column_index)
                    .draw(renderer) {
                    Some(textfield::Changed) => {},
                    _ => {},
                }
                dropdown(layer, vec!["g", "dkg", "kg"].as_slice(), &mut meal_food.weight_type)
                    .right(SizeInCharacters(1))
                    .draw(renderer);
                if button(layer, "-")
                    .right(SizeInCharacters(2))
                    .draw(renderer) {
                    deleting_index = Some(i);
                }
            }
            if deleting_index.is_some() {
                meal.foods.remove(deleting_index.unwrap());
            }
        });
    }

    fn calc_macro_ratio(foods: &Vec<db::Food>, meal: &db::Meal) -> (f32, f32, f32, f32) {
        let (mut p, mut ch, mut f, mut w) = (0f32, 0f32, 0f32, 0f32);
        for meal_food in meal.foods.iter() {
            let ref food = foods[meal_food.food_id-1];
            let standard_weight = food.weight_type.to_g(food.weight);
            let input_weight =  meal_food.weight_type.to_g(meal_food.weight);
            let ratio = input_weight / standard_weight;
            p = p + food.protein * ratio;
            ch = ch + food.ch * ratio;
            f = f + food.fat * ratio;
            w = w + input_weight;
        }
        return (p, ch, f, w);
    }

}
