use imgui::base;
use imgui::base::SizeInCharacters;

use imgui::textfield::textfield_str;
use imgui::label::label;
use imgui::button::button;
use imgui::header::header;
use imgui::dropdown::dropdown;


pub fn do_logic(layer: &mut base::Layer, event_template: &mut ::EventTemplate) -> bool {

    header(layer, "Event Template", SizeInCharacters(11), SizeInCharacters(10))
        .x(SizeInCharacters(10) )
        .y(SizeInCharacters(1))
        .draw_with_body(|layer| {
            let first_column = layer.last_x + SizeInCharacters(1);

            textfield_str(layer, &mut event_template.name, SizeInCharacters(20))
                .x(first_column)
                .y(SizeInCharacters(1))
                .default_text("Template name...")
                .draw();

            println!("{}", event_template.input_type );
            dropdown(layer, ["Num", "Bool", "Stack", "Text", "Img"], &mut event_template.input_type)
                .down(SizeInCharacters(0))
                .draw();
            dropdown(layer, ["Vitaminok", "Edzés", "Munka", "Life"], &mut event_template.input_type)
                .down(SizeInCharacters(0))
                .draw();
            if button(layer, "Add new")
                .x(first_column)
                .draw() {
                
            }
    });
    label(layer, "Labelname")
            .x(SizeInCharacters(1))
            .y(SizeInCharacters(2))
            .draw();
    return false;
}

