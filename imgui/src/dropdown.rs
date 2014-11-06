extern crate sdl2;
extern crate sdl2_ttf;


use sdl2::pixels::RGB;
use sdl2::rect::Rect;
use base;
use base::SizeInCharacters;
use base::IndexValue;

pub struct DropdownBuilder<'a> {
	disabled: bool,
	x: SizeInCharacters,
	y: SizeInCharacters,
	labels: &'a [&'a str],
	layer: &'a mut base::Layer,
	value: &'a mut IndexValue + 'a
}

pub fn dropdown<'a>(layer: &'a mut base::Layer, labels: &'a [&str], value: &'a mut IndexValue) -> DropdownBuilder<'a> {
	DropdownBuilder::new(layer, labels, value)
}

impl<'a> DropdownBuilder<'a> {
	pub fn new(layer: &'a mut base::Layer, labels: &'a [&str], value: &'a mut IndexValue) -> DropdownBuilder<'a> {
		DropdownBuilder {
			disabled: false,
			x: layer.last_x,
			y: layer.last_y,
			labels: labels,
			layer: layer,
			value: value,
		}
	}

	pub fn disabled(mut self, v: bool) -> DropdownBuilder<'a> {self.disabled = v; self}
	pub fn x(mut self, v: SizeInCharacters) -> DropdownBuilder<'a> {self.x = v; self}
	pub fn y(mut self, v: SizeInCharacters) -> DropdownBuilder<'a> {self.y = v; self}
	pub fn right(mut self, x: SizeInCharacters) -> DropdownBuilder<'a> {
		self.x = self.layer.last_x + self.layer.last_w + x;
		self
	}

	pub fn inner_right(mut self, x: SizeInCharacters) -> DropdownBuilder<'a> {
		self.x = self.layer.last_x + x;
		self
	}

	pub fn down(mut self, y: SizeInCharacters) -> DropdownBuilder<'a> {
		self.y = self.layer.last_y + self.layer.last_h + y;
		self
	}

	pub fn inner_down(mut self, y: SizeInCharacters) -> DropdownBuilder<'a> {
		self.y = self.layer.last_y + y;
		self
	}


	pub fn draw(&mut self) -> bool {
		draw(self)
	}
}

fn get_longest_word_len(labels: &[&str]) -> i32 {
	let mut len = 0;
	for label in labels.iter() {
		if len < label.len() {
			len = label.len();
		}
	}
	return len as i32;
}

pub fn draw(builder: &mut DropdownBuilder) -> bool {
	let char_w = builder.layer.char_w;
	let char_h = builder.layer.char_h;
	let x = builder.x.in_pixels(char_w);
	let y = builder.y.in_pixels(char_h);
	let h = builder.layer.char_h;
	let longest_word_len = get_longest_word_len(builder.labels);
	let label_w = char_w * longest_word_len;
	let down_arrow_w = char_w*2;
	let all_w = label_w + down_arrow_w;

	builder.layer.last_x = builder.x;
	builder.layer.last_y = builder.y;
	builder.layer.last_w = SizeInCharacters(longest_word_len + 2);
	builder.layer.last_h = SizeInCharacters(1);

	let id = x << 16 | (y&0xFFFF);
	let was_active = builder.layer.is_active_widget(id);
	let hover = builder.layer.is_mouse_in(x, y, all_w, h);
	let click = builder.layer.is_mouse_released() && hover;
	let just_clicked = click && !was_active;
	let click_while_active = click && was_active;
	let clicked_out = builder.layer.is_mouse_released() && !hover && was_active;
	let active = was_active && !clicked_out;

	let mut modified = false;
	if just_clicked {
		builder.layer.set_active_widget(id);
	} else if click_while_active {
		builder.layer.clear_active_widget();
	} else if clicked_out {
		if builder.layer.is_mouse_in(x, y + char_h, all_w, builder.labels.len() as i32 * char_h) {
			let selected_index = ((builder.layer.mouse_y() - (y + char_h)) / char_h) as uint ;
			if selected_index != builder.value.get() {
				builder.value.set(selected_index);
				modified = true;
			}
		}
		builder.layer.clear_active_widget();
	}

	let top_color = match hover {
		false => RGB(82, 85, 90),
		true => RGB(102, 105, 110),
	};
	let bottom_color = match hover {
		false => RGB(47, 50, 53),
		true => RGB(47, 50, 53),
	};
	builder.layer.fill_rect(x, y, label_w, h, RGB(0, 0, 0));
	builder.layer.draw_rect_gradient(x+1, y+1, label_w-2, h-2, top_color, bottom_color);
	if builder.labels[builder.value.get()].len() > 0 {
		builder.layer.draw_text(x+1, y+1, builder.labels[builder.value.get()], RGB(221, 221, 221));
	}

	builder.layer.fill_rect(x+label_w, y, down_arrow_w, h, RGB(0, 0, 0));
	builder.layer.draw_rect_gradient(x+label_w+1, y+1, down_arrow_w-2, h-2, top_color, bottom_color);
	let arrow_char = if active {"▲"} else {"▼"};
	builder.layer.draw_text(x + label_w+char_w/3, y, arrow_char, RGB(221, 221, 221));

	if active {
		let mut layer = builder.layer.popup_layer.as_mut().unwrap();
		for (i, label) in builder.labels.iter().enumerate() {
			let i = i as i32;
			layer.fill_rect(x, y + (1+i)*char_h, all_w, h, RGB(0, 0, 0));

			let color = match layer.is_mouse_in(x, y + (1+i)*char_h, all_w, h) {
				true => sdl2::pixels::RGB(82, 82, 90),
				false => sdl2::pixels::RGB(51, 51, 51),
			};
			layer.fill_rect(x+1, y+1+(1+i)*char_h, all_w-2, h-2, color);
			if label.len() == 0 {
				continue;
			}
			layer.draw_text(x+1, y +1+ (1+i)*char_h, *label, RGB(198, 198, 198));
		}
	}
	modified
}
