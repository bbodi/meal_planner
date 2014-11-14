use imgui::base;
use imgui::base::SizeInCharacters;

use imgui::textfield::textfield_str;
use imgui::label::label;
use imgui::button::button;
use imgui::header::header;
use imgui::dropdown::dropdown;


pub fn do_logic(layer: &mut base::Layer, event_template: &mut ::EventTemplate) -> bool {

    header(layer, "Event Template", SizeInCharacters(25), SizeInCharacters(10))
        .x(SizeInCharacters(10) )
        .y(SizeInCharacters(10))
        .draw_with_body(|layer| {
            let first_column = layer.last_x + SizeInCharacters(1);

            textfield_str(layer, &mut event_template.name, SizeInCharacters(20))
                .x(first_column)
                .down(SizeInCharacters(1))
                .default_text("Template name...")
                .draw();

            dropdown(layer, ["Num", "Bool", "Stack", "Text", "Img"], &mut event_template.input_type)
                .down(SizeInCharacters(1))
                .draw();
            dropdown(layer, ["Vitaminok", "Edzés", "Munka", "Life"], &mut event_template.input_type)
                .down(SizeInCharacters(1))
                .draw();
            if button(layer, "Add new")
                .down(SizeInCharacters(1))
                .draw() {
                
            }
    });
    label(layer, "Labelname")
            .x(SizeInCharacters(10))
            .y(SizeInCharacters(20))
            .draw();
    return false;
}

