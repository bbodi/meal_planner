use imgui::base;
use imgui::base::SizeInCharacters;

use imgui::textfield::textfield_str;
use imgui::label::label;
use imgui::button::button;
use imgui::header::header;
use imgui::dropdown::dropdown;

pub struct EventTemplateWindow {
	new_event_group_name: String,
}

impl EventTemplateWindow {
	pub fn new() -> EventTemplateWindow {
		EventTemplateWindow {
			new_event_group_name: String::new(),
		}
	}

	pub fn do_logic(&mut self, layer: &mut base::Layer, event_template: &mut ::EventTemplate, event_groups: &mut Vec<::EventGroup>) -> bool {

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
	            if !event_groups.is_empty() {
		            let group_names = event_groups.iter().map(|x| x.name.as_slice()).collect::<Vec<_>>();
		            dropdown(layer, group_names.as_slice(), &mut event_template.event_group_id)
		                .down(SizeInCharacters(1))
		                .draw();

		            if button(layer, "Add new")
		                .down(SizeInCharacters(1))
		                .draw() {
		                
		            }
	        	}
	    });
	    header(layer, "Event Groups", SizeInCharacters(27), SizeInCharacters(10))
	        .right(SizeInCharacters(0) )
	        .draw_with_body(|layer| {
	            let first_column = layer.last_x + SizeInCharacters(1);
	            layer.last_y = layer.last_y + SizeInCharacters(1);
	            for event_group in event_groups.iter_mut() {
		            textfield_str(layer, &mut event_group.name, SizeInCharacters(20))
		                .x(first_column)
		                .down(SizeInCharacters(0))
		                .draw();
		        }
	            let result = textfield_str(layer, &mut self.new_event_group_name, SizeInCharacters(20))
	                .x(first_column)
	                .down(SizeInCharacters(1))
	                .default_text("Group name...")
	                .draw();
	            if !self.new_event_group_name.is_empty() && button(layer, "Add")
	                .right(SizeInCharacters(1))
	                .draw() || (result.is_some() && (result.unwrap() == ::imgui::textfield::Enter) ) {
	                event_groups.push(::EventGroup::new(0, self.new_event_group_name.as_slice()));
	                self.new_event_group_name.clear();
	                layer.clear_textfield_states();
	            }
	    });
	    return false;
	}
}

