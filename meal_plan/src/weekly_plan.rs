extern crate sdl2;
extern crate sdl2_ttf;
extern crate time;

use std::cmp::max;
use std::cmp::min;

use imgui::base;
use imgui::base::SizeInCharacters;


use imgui::label::label;
use imgui::button::button;
use imgui::header::header;
use imgui::dropdown::dropdown;
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

    pub fn do_logic(&mut self, event: &sdl2::event::Event, daily_menus: &[DailyMenu]) -> bool {
        self.layer.handle_event(event);

        let current_week = (time::now().tm_yday+6) / 7;
        let day_names = ["Monday", "Thuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];
        for (i, week_num) in range(max(0, current_week-3), min(52, current_week+4) ).enumerate() {
            header(&mut self.layer, format!("Week {}", week_num).as_slice(), SizeInCharacters(11), SizeInCharacters(10))
                .x(SizeInCharacters(10 + (11 * i as i32)) )
                .y(SizeInCharacters(1))
                .draw_with_body(|layer| {
                    let mut selected_index = 0;
                    let header_start_x = layer.last_x;
                    for (_, _) in daily_menus.iter().enumerate() {
                        let daily_menu_names = if daily_menus.len() > 0 {
                            daily_menus.iter().map(|x| x.name.as_slice()).collect::<Vec<&str>>()
                        } else {
                            vec!["Add new"]
                        };

                        dropdown(layer, daily_menu_names.as_slice(), &mut selected_index)
                            .down(SizeInCharacters(0))
                            .x(header_start_x + SizeInCharacters(1))
                            .draw();
                        if button(layer, "Add new")
                            .x(header_start_x + SizeInCharacters(1))
                            .draw() {
                            //let mut daily_plan = daily_plan::DailyPlan::new(&mut last_meal_id);
                        }
                    }
                    if button(layer, "Add new")
                        .x(header_start_x + SizeInCharacters(1))
                        .draw() {
                        //let mut daily_plan = daily_plan::DailyPlan::new(&mut last_meal_id);
                    }
                });
        }
        for (i, day_name) in day_names.iter().enumerate() {
            label(&mut self.layer, *day_name)
                .x(SizeInCharacters(1))
                .y(SizeInCharacters(2 + i as i32))
                .draw();
        }
        return false;
    }
}
