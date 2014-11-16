use imgui::base;
use imgui::base::SizeInCharacters;

use imgui::textfield::textfield_str;
use imgui::label::label;
use imgui::button::button;
use imgui::header::header;
use imgui::dropdown::dropdown;
use imgui::checkbox::checkbox;

use db::Event;
use db::EventTemplate;
use db::Tag;


pub struct EventTemplateWindow {
	new_event_tag_name: String,
	selected_template_index: Option<uint>,
}


impl EventTemplateWindow {
	pub fn new() -> EventTemplateWindow {
		EventTemplateWindow {
			new_event_tag_name: String::new(),
			selected_template_index: None,
		}
	}

	pub fn do_logic(&mut self, layer: &mut base::Layer, event_templates: &mut Vec<EventTemplate>, event_tags: &mut Vec<Tag>) -> bool {
		header(layer, "Event Templates", SizeInCharacters(25), SizeInCharacters(20))
	        .x(SizeInCharacters(10) )
	        .y(SizeInCharacters(10))
	        .draw_with_body(|layer| {
	        let first_column = layer.last_x + SizeInCharacters(1);
	        for (idx, event_template) in event_templates.iter().enumerate() {
		        if button(layer, event_template.name.as_slice())
		                .x(first_column)
		                .down(SizeInCharacters(1))
		                .draw() {
		        	self.selected_template_index = Some(idx);
		        }
			}
			if button(layer, "Add new")
                .down(SizeInCharacters(1))
                .draw() {
                let mut new_temp = EventTemplate::new();
                new_temp.name = "Unknown".into_string();
                new_temp.tag_id = event_tags[0].id();
            	event_templates.push(new_temp);
            }
		});
		if self.selected_template_index.is_some() {
			let mut event_template = &mut event_templates[self.selected_template_index.unwrap()];
			let mut event_tag_index = if event_template.tag_id.is_none() {
				0
			} else {
				event_tags.iter().enumerate()
				.filter(|&(i, x)| x.id().unwrap() == event_template.tag_id.unwrap_or(0))
				.map(|(i, _)| i).next().unwrap()
			};
			header(layer, "Event Template", SizeInCharacters(25), SizeInCharacters(20))
		        .right(SizeInCharacters(0) )
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

	            checkbox(layer, &mut event_template.private)
	            	.label("private")
	                .down(SizeInCharacters(1))
	                .draw();

	            checkbox(layer, &mut event_template.mandatory)
	                .down(SizeInCharacters(1))
	                .label("mandatory")
	                .draw();
	            
	            if !event_tags.is_empty() {
		            let group_names = event_tags.iter().map(|x| x.name.as_slice()).collect::<Vec<_>>();
		            if dropdown(layer, group_names.as_slice(), &mut event_tag_index)
		                .down(SizeInCharacters(1))
		                .draw() {
		            	event_template.tag_id = event_tags[event_tag_index].id();
		            }
	        	}
		    });
		}
	    header(layer, "Event Tag", SizeInCharacters(27), SizeInCharacters(20))
	        .right(SizeInCharacters(0) )
	        .draw_with_body(|layer| {
	            let first_column = layer.last_x + SizeInCharacters(1);
	            layer.last_y = layer.last_y + SizeInCharacters(1);
	            for event_group in event_tags.iter_mut() {
		            textfield_str(layer, &mut event_group.name, SizeInCharacters(20))
		                .x(first_column)
		                .down(SizeInCharacters(0))
		                .draw();
		        }
	            let result = textfield_str(layer, &mut self.new_event_tag_name, SizeInCharacters(20))
	                .x(first_column)
	                .down(SizeInCharacters(1))
	                .default_text("Tag name...")
	                .draw();
	            if button(layer, "Add")
	            	.disabled(self.new_event_tag_name.is_empty())
	                .right(SizeInCharacters(1))
	                .draw() || (result.is_some() && (result.unwrap() == ::imgui::textfield::Enter) ) {
	                event_tags.push(Tag::new(self.new_event_tag_name.as_slice()));
	                self.new_event_tag_name.clear();
	                layer.clear_textfield_states();
	            }
	    });
		if button(layer, "Save").down(SizeInCharacters(1)).draw() {
			return true;
	    }
	    return false;
	}
}

