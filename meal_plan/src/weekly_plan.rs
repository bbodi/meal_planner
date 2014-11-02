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

    last_daily_menu_id: &'a mut uint,
}

impl<'a> WeeklyPlan<'a> {

    pub fn new(last_daily_menu_id: &'a mut uint) -> WeeklyPlan<'a> {
        WeeklyPlan {
            layer: base::Layer::new(),
            last_daily_menu_id: last_daily_menu_id, 
        }
    }

    pub fn do_logic(&mut self, renderer: &sdl2::render::Renderer, event: &sdl2::event::Event, foods: &[db::Food], daily_menu: &[DailyMenu], nutr_goal: &db::NutritionGoal) -> bool {
        self.layer.handle_event(event);

        let current_week = time::now().tm_yday / 7;
        for (i, week_num) in range(max(0, current_week-3), min(52, current_week+3) ).enumerate() {
            header(&mut self.layer, format!("Week {}", week_num).as_slice(), SizeInCharacters(10), SizeInCharacters(10))
                .x(SizeInCharacters(10 + (10 * i as i32)) )
                .y(SizeInCharacters(1))
                .draw(renderer);
        }
        return false;
    }
}
