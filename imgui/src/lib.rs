#![crate_name = "imgui"]
#![crate_type = "lib"]

 // TODO: is_active_widtget SizeInCharacter-t fogadjon el, ne pixelt!

extern crate sdl2;
extern crate sdl2_ttf;

pub mod drawing_surf;
pub mod base;
pub mod button;
pub mod textfield;
pub mod line_chart;
pub mod checkbox;
pub mod dropdown;
pub mod header;
pub mod scrollbar;
pub mod label;
pub mod panel;
pub mod slider;