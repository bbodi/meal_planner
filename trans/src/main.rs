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

use std::default::Default;
use std::error::FromError;
use std::io::{IoResult, IoError, InvalidInput};
use std::os;

use sqlite3::{
    Access,
    DatabaseConnection,
    DatabaseUpdate,
    Query,
    ResultRowAccess,
    SqliteResult,
};
use sqlite3::access;
use sqlite3::access::flags::OPEN_READONLY;

const SCREEN_WIDHT: u32 = 1024;
const SCREEN_HEIGHT: u32 = 768;

#[deriving(Show)]
struct Person {
    id: i32,
    name: String,
}

fn make_people(conn: &mut DatabaseConnection) -> SqliteResult<Vec<Person>> {
    try!(conn.exec("CREATE TABLE person (
                 id              SERIAL PRIMARY KEY,
                 name            VARCHAR NOT NULL
               )"));

    {
        let mut tx = try!(conn.prepare("INSERT INTO person (id, name)
                           VALUES (0, 'Dan')"));
        let changes = try!(conn.update(&mut tx, []));
        assert_eq!(changes, 1);
    }

    let mut stmt = try!(conn.prepare("SELECT id, name FROM person"));

    let mut ppl = vec!();
    try!(stmt.query(
        [], |row| {
            ppl.push(Person {
                id: row.get(0u),
                name: row.get(1u)
            });
            Ok(())
        }));
    Ok(ppl)
}

fn main() {
	let access = access::ByFilename { flags: Default::default(), filename: "db.db" };
	let mut conn = match DatabaseConnection::new(access) {
		Ok(x) => x,
		Err(e) =>  panic!(e),
	};
    make_people(&mut conn);//.map_err(|e| FromError::from_error(e));



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
    

    let mut last: i32 = 30;
    let mut datas = vec![];
    for _ in range(0, 5i32) {
        let mut data = vec![];
        for _ in range(0, 50i32) {
            last = last + std::rand::random::<i32>().abs() % 7 - 3;
            data.push(last);
        }
        datas.push(data);
    }

    let mut frame_count = 0u32;
    let mut next_frame_tick = 0;
    let mut fps = 0;

    let mut layer = imgui::base::Layer::new();
    let mut slider_val = SizeInCharacters(5);
    let mut slider_val2 = SizeInCharacters(5);
    let mut slider_val3 = SizeInCharacters(5);

    
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

        slider(&mut layer, &mut slider_val, Vertical, SizeInCharacters(127), SizeInCharacters(45))
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
            .draw(&renderer);
        
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
