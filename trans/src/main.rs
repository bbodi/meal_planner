extern crate sdl2;
extern crate sdl2_ttf;

extern crate imgui;
extern crate sqlite3;

use sdl2::pixels::RGB;
use sdl2::pixels::RGBA;

use imgui::label::label;
use imgui::base::SizeInCharacters;
use imgui::checkbox::checkbox;
use imgui::panel::panel;
use imgui::button::button;
use imgui::textfield::textfield_i32;
use imgui::dropdown::dropdown;
use imgui::line_chart::line_chart;
use imgui::slider::slider;
use imgui::slider::Vertical;
use imgui::slider::Horizontal;

use db::Event;
use db::EventTemplate;
use db::Tag;

use event_template_window::EventTemplateWindow;
use timeline::TimelineWindow;

mod event_template_window;
mod timeline;
mod db;
mod new_event_win;

const SCREEN_WIDHT: u32 = 1024;
const SCREEN_HEIGHT: u32 = 768;


struct Project {
    start_event_id: u32,
    end_event_id: u32,
}

fn main() {
    sdl2::init(sdl2::INIT_VIDEO);
    sdl2_ttf::init();

    let window = match sdl2::video::Window::new("rust-sdl2 demo: Video", sdl2::video::PosCentered, sdl2::video::PosCentered, SCREEN_WIDHT as int, SCREEN_HEIGHT as int, sdl2::video::SHOWN | sdl2::video::RESIZABLE) {
        Ok(window) => window,
        Err(err) => panic!(format!("paniced to create window: {}", err))
    };
    //window.set_size(1280, 900);
    window.set_position(sdl2::video::PosCentered, sdl2::video::PosCentered);

    let renderer = match sdl2::render::Renderer::from_window(window, sdl2::render::DriverAuto, sdl2::render::ACCELERATED) {
        Ok(renderer) => renderer,
        Err(err) => panic!(format!("paniced to create renderer: {}", err))
    };
    let _ = renderer.set_logical_size(SCREEN_WIDHT as int, SCREEN_HEIGHT as int);
    

    let mut frame_count = 0u32;
    let mut next_frame_tick = 0;
    let mut fps = 0;

    let mut layer = imgui::base::Layer::new();
    let mut slider_val = SizeInCharacters(5);
    let mut slider_val2 = SizeInCharacters(5);
    let mut slider_val3 = SizeInCharacters(5);
    
    let mut dao = db::Dao::new();
    let mut event_tags: Vec<Tag> = dao.load_tags();
    let mut event_templates = dao.load_event_templates();

    let mut event_template_window = EventTemplateWindow::new();
    let mut timeline = TimelineWindow::new(SizeInCharacters(0), SizeInCharacters(0), SizeInCharacters(128), SizeInCharacters(45));
    
    'main : loop {
        sdl2::timer::delay(10);


        let event = match sdl2::event::poll_event() {
            sdl2::event::QuitEvent(_) => break 'main,
            e => e,
            // _ => {},
        };

        layer.handle_event(&event);
        if !layer.active {
            sdl2::timer::delay(100);
            continue;
        }
        let mouse_str = format!("FPS: {}, {}, {}", fps, layer.mouse_x() / layer.char_w, layer.mouse_y()/ layer.char_h);
        match renderer.get_parent() {
            &sdl2::render::WindowParent(ref w) => w.set_title(mouse_str.as_slice()),
            _ => {},
        };

        /*slider(&mut layer, &mut slider_val, Vertical, SizeInCharacters(127), SizeInCharacters(45))
            .x(SizeInCharacters(0))
            .y(SizeInCharacters(0)).draw();

        slider(&mut layer, &mut slider_val2, Horizontal, SizeInCharacters(127-1)-slider_val, SizeInCharacters(45))
            .x(slider_val + SizeInCharacters(1))
            .y(SizeInCharacters(0)).draw();

        slider(&mut layer, &mut slider_val3, Vertical, SizeInCharacters(127-1)-slider_val, SizeInCharacters(45) - slider_val2)
            .x(slider_val + SizeInCharacters(1))
            .y(slider_val2 + SizeInCharacters(1)).draw();


        line_chart(&mut layer, "Datas", slider_val, SizeInCharacters(10))
            .x(SizeInCharacters(0))
            .y(SizeInCharacters(0))
            .data(datas[0].as_slice())
            .bottom_color(RGBA(82, 82, 82, 150))
            .top_color(RGB(60, 60, 60))
            .draw(&renderer);
        line_chart(&mut layer, "Datas", SizeInCharacters(127-1)-slider_val, slider_val2)
            .x(slider_val + SizeInCharacters(1))
            .y(SizeInCharacters(0))
            .data(datas[1].as_slice())
            .bottom_color(RGBA(82, 82, 82, 150))
            .top_color(RGB(60, 60, 60))
            .draw(&renderer);
        line_chart(&mut layer, "Datas", slider_val3, SizeInCharacters(45) - slider_val2)
            .x(slider_val + SizeInCharacters(1))
            .y(slider_val2 + SizeInCharacters(1))
            .data(datas[2].as_slice())
            .bottom_color(RGBA(82, 82, 82, 150))
            .top_color(RGB(60, 60, 60))
            .draw(&renderer);*/
        

        if event_template_window.do_logic(&mut layer, &mut event_templates, &mut event_tags) {
        	dao.persist_event_tags(&mut event_tags);
        	dao.persist_event_templates(&mut event_templates);
        }
        //timeline.do_logic(&mut layer);
        

        layer.draw(&renderer);

        renderer.present();

        let keys = sdl2::keyboard::get_keyboard_state();
        if keys[sdl2::scancode::EscapeScanCode] {
            break 'main;
        }

        let _ = renderer.set_draw_color(sdl2::pixels::RGB(60 , 59, 64));
        let _ = renderer.clear();
        frame_count += 1;

        let current_tick = sdl2::timer::get_ticks();
        if current_tick >= next_frame_tick {
            fps = frame_count;
            next_frame_tick = current_tick + 1000;
            frame_count = 0;
        }
    }

    sdl2::quit();
}
