extern crate sdl2;
extern crate sdl2_ttf;
extern crate time;

use std::time::duration::Duration;
use std::cmp::max;
use std::cmp::min;

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


pub struct WeeklyPlan<'a> {
    pub layer: base::Layer,
}

impl<'a> WeeklyPlan<'a> {

    pub fn new() -> WeeklyPlan<'a> {
        WeeklyPlan {
            layer: base::Layer::new(),
        }
    }

    pub fn do_logic(&mut self, renderer: &sdl2::render::Renderer, event: &sdl2::event::Event, foods: &[db::Food], daily_menus: &[DailyMenu], nutr_goal: &db::NutritionGoal, last_daily_menu_id: &mut uint) -> bool {
        self.layer.handle_event(event);

        let julian = time::now().tm_yday+1;
        //let dow = time::now().tm_wday;
        //let dowJan1 = 
        let current_week = (time::now().tm_yday+6) / 7;
        let day_names = ["Monday", "Thuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];
        for (i, week_num) in range(max(0, current_week-3), min(52, current_week+4) ).enumerate() {
            header(&mut self.layer, format!("Week {}", week_num).as_slice(), SizeInCharacters(11), SizeInCharacters(10))
                .x(SizeInCharacters(10 + (11 * i as i32)) )
                .y(SizeInCharacters(1))
                .draw_with_body(renderer, |layer| {
                    let mut selected_index = 0;
                    let header_start_x = layer.last_x;
                    for (i, _) in day_names.iter().enumerate() {
                        let daily_menu_names = if daily_menus.len() > 0 {
                            daily_menus.iter().map(|x| x.name.as_slice()).collect::<Vec<&str>>()
                        } else {
                            vec!["Add new"]
                        };

                        dropdown(layer, daily_menu_names.as_slice(), &mut selected_index)
                            .down(SizeInCharacters(0))
                            .x(header_start_x + SizeInCharacters(1))
                            .draw(renderer);
                        /*if button(&mut self.layer, "Add new")
                            .x(header_start_x + SizeInCharacters(1))
                            .draw(renderer) {
                            let mut daily_plan = daily_plan::DailyPlan::new(&mut last_meal_id);
                        }*/
                    }
                });
        }
        for (i, day_name) in day_names.iter().enumerate() {
            label(&mut self.layer, *day_name)
                .x(SizeInCharacters(1))
                .y(SizeInCharacters(2 + i as i32))
                .draw(renderer);
        }
        return false;
    }
}
