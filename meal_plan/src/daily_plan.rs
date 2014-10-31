extern crate sdl2;
extern crate sdl2_ttf;

use imgui::base;
use imgui::base::SizeInCharacters;

use sdl2::pixels::RGB;

use imgui::label::label;
use imgui::textfield::textfield_f32;
use imgui::textfield::textfield_i32;
use imgui::textfield::textfield_str;
use tricolor_field::tricolor_field_str;
use imgui::button::button;
use imgui::header::header;
use tricolor_label::tricolor_label;
use db;



pub struct DailyPlan {
    pub layer: base::Layer,

    page: uint,
}

impl DailyPlan {

    pub fn new() -> DailyPlan {
        DailyPlan {
            layer: base::Layer::new(),
            page: 0
        }
    }



    pub fn do_logic(&mut self, renderer: &sdl2::render::Renderer, event: &sdl2::event::Event, foods: &Vec<db::Food>) -> bool {
        self.layer.handle_event(event);

        let column_height = SizeInCharacters(34);
        header(&mut self.layer, "Foods", SizeInCharacters(17), column_height)
            .x(SizeInCharacters(1))
            .y(SizeInCharacters(1))
            .draw(renderer);
        let price_header_x = self.layer.last_x + self.layer.last_w;
        let first_row = self.layer.last_x + SizeInCharacters(1);
        for (_, food) in foods.iter().skip(self.page * 16).take((self.page+1) * 16).enumerate() {
            let fs = food.weight_type.to_g(food.size);
            let values = [food.protein / fs, food.ch / fs, food.fat / fs];
            tricolor_label(label(&mut self.layer, food.name.as_slice())
                .x(first_row)
                .down(SizeInCharacters(1)), values).draw(renderer);

            if button(&mut self.layer, "Add").right(SizeInCharacters(4)).draw(renderer) {

            }
        }
        if button(&mut self.layer, "Prev")
            .disabled(self.page == 0)
            .down(SizeInCharacters(1))
            .x(first_row)
            .y(SizeInCharacters(35))
            .draw(renderer) {
            self.page = self.page - 1;
        }
        if button(&mut self.layer, "Next")
            .disabled(self.page >= (foods.len() / 16))
            .right(SizeInCharacters(20))
            .draw(renderer) {
            self.page = self.page + 1;
        }
        if button(&mut self.layer, "Close")
            .down(SizeInCharacters(1))
            .x(first_row).draw(renderer) {
            return true;
        }
        return false;
    }

}
