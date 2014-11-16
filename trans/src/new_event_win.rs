use imgui::base;
use imgui::base::SizeInCharacters;

use imgui::textfield::textfield_str;
use imgui::textfield::textfield_f32;
use imgui::label::label;
use imgui::button::button;
use imgui::header::header;
use imgui::dropdown::dropdown;
use imgui::checkbox::checkbox;

use db::Event;
use db::EventTemplate;
use db::Tag;
use db;

pub struct EventWindow;


impl EventWindow {
	pub fn new() -> EventWindow {
		EventWindow
	}

	pub fn do_logic(&mut self, layer: &mut base::Layer, event: &mut Event, event_template: Option<&mut EventTemplate>) -> bool {
		header(layer, "Event", SizeInCharacters(50), SizeInCharacters(20))
	        .x(SizeInCharacters(10) )
	        .y(SizeInCharacters(10))
	        .draw_with_body(|layer| {
	        let first_column = layer.last_x + SizeInCharacters(1);
	        let event_template_ref = event_template.as_ref().unwrap();
	        if event_template.is_none() || event_template_ref.input_type == db::Num {
	        	textfield_f32(layer, &mut event.num, SizeInCharacters(20))
		                .x(first_column)
		                .down(SizeInCharacters(1))
		                .default_text("Value...")
		                .draw();
	        } else if event_template.is_none() || event_template_ref.input_type == db::Bool {
	        	checkbox(layer, &mut event.private)
		            	.label("Value")
		                .down(SizeInCharacters(1))
		                .draw();
	        } else if event_template.is_none() || event_template_ref.input_type == db::Stack {
	        	checkbox(layer, &mut event.private)
		            	.label("Value")
		                .down(SizeInCharacters(1))
		                .draw();
	        } else if event_template.is_none() || event_template_ref.input_type == db::Text {
	        	textfield_str(layer, &mut event.text, SizeInCharacters(40))
		                .x(first_column)
		                .down(SizeInCharacters(1))
		                .default_text("Value...")
		                .draw();
	        }

	        textfield_str(layer, &mut event.note, SizeInCharacters(40))
	                .x(first_column)
	                .down(SizeInCharacters(1))
	                .default_text("Notes...")
	                .draw();
		});
		if button(layer, "Ok").down(SizeInCharacters(1)).draw() {
			return true;
	    }
	    return false;
	}
}

