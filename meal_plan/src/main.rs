extern crate sdl2;
extern crate sdl2_ttf;

extern crate imgui;
extern crate csv;
extern crate serialize;


use sdl2::pixels::RGB;
use sdl2::pixels::RGBA;

use imgui::label::label;
use imgui::base::SizeInCharacters;
use imgui::checkbox::checkbox;
use imgui::panel::panel;

const SCREEN_WIDHT: u32 = 1024;
const SCREEN_HEIGHT: u32 = 768;


mod db;

mod tricolor_field;
mod tricolor_label;

mod kcal_window;
mod kcal_table;
mod daily_plan;


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
    let _ = renderer.set_draw_color(sdl2::pixels::RGB(0, 0, 255));
    let _ = renderer.clear();

    let font = match sdl2_ttf::Font::from_file(&Path::new("DejaVuSansMono.ttf"), 128) {
        Ok(f) => f,
        Err(e) => panic!(e),
    };
    let surface = match font.render_str_blended("Hello Rust!", sdl2::pixels::RGBA(255, 0, 0, 255)) {
        Ok(s) => s,
        Err(e) => panic!(e),
    };
    let texture = match renderer.create_texture_from_surface(&surface) {
        Ok(t) => t,
        Err(e) => panic!(e),
    };
    let _ = renderer.copy(&texture, None, None);

    let mut last: i32 = 20;
    let mut datas = vec![];
    for _ in range(0, 5i32) {
        let mut data = vec![];
        for _ in range(0, 50i32) {
            last = last + std::rand::random::<i32>().abs() % 7 - 3;
            data.push(last);
        }
        datas.push(data);
    }
    /*let mut chart = line_chart::Chart::new(400, 400);

    let mut layer = widget::Layer::new(&renderer, SCREEN_WIDHT, SCREEN_HEIGHT);
    //layer.add_widget(box chart, sdl2::rect::Rect::new(10, 10, 410, 410));
    let mut btn = button::Button::new("Add data");*/
    //layer.add_widget(btn, sdl2::rect::Rect::new(420, 20, 62, 16));

    let mut frame_count = 0u32;
    let mut next_frame_tick = 0;
    let mut fps = 0;

    let mut dao = db::Dao::new();
    let mut foods = dao.load_foods();
    let mut daily_menus = dao.load_daily_menus();

    let mut layer = imgui::base::Layer::new();
    let mut recommended_macros = dao.load_recommended_macros();
    println!("{}", recommended_macros.protein);
    let mut kcal_win = kcal_window::KCalWindow::new();
    let mut kcal_table = kcal_table::KCalTable::new();
    let mut last_meal_id = 0;
    let mut last_meal_food_id = 0;
    let mut daily_plan = daily_plan::DailyPlan::new(&mut last_meal_id, &mut last_meal_food_id);
    let mut show_cal_win = false;
    let mut show_table_win = false;
    let mut show_daily_win = true;
    'main : loop {
        sdl2::timer::delay(10);
        let current_tick = sdl2::timer::get_ticks();


        let event = match sdl2::event::poll_event() {
            sdl2::event::QuitEvent(_) => break 'main,
            e => e,
            // _ => {},
        };

        if show_cal_win {
            if kcal_win.do_logic(&renderer, &event, &mut recommended_macros) {
                dao.persist_recommended_macros(&recommended_macros);
            }
        }
        if show_table_win {
            if kcal_table.do_logic(&renderer, &event, &mut foods) {
                dao.persist_foods(foods.as_slice());
            }
        }
        if show_daily_win {
            if daily_plan.do_logic(&renderer, &event, &mut foods, daily_menus.get_mut(0)) {
                dao.persist_daily_menu(daily_menus.as_mut_slice());
            }
        }

        if layer.control_keys.ctrl.down {
            panel(&mut layer, SizeInCharacters(20), SizeInCharacters(5))
                .x(SizeInCharacters(10))
                .y(SizeInCharacters(10))
                .draw(&renderer);
            if checkbox(&mut layer, &mut show_cal_win)
                .label("Calorie window")
                .x(SizeInCharacters(10))
                .y(SizeInCharacters(10)).draw(&renderer) && show_cal_win {
                show_daily_win = false;
                show_table_win = false;
            }
            if checkbox(&mut layer, &mut show_table_win)
                .label("Food list window")
                .x(SizeInCharacters(10))
                .down(SizeInCharacters(1)).draw(&renderer) && show_table_win {
                show_daily_win = false;
                show_cal_win = false;
            }
            if checkbox(&mut layer, &mut show_daily_win)
                .label("Daily window")
                .x(SizeInCharacters(10))
                .down(SizeInCharacters(1)).draw(&renderer) && show_daily_win {
                show_table_win = false;
                show_cal_win = false;
            }
        }


        layer.handle_event(&event);
        let mouse_str = format!("FPS: {}, {}, {}", fps, layer.mouse_x() / layer.char_w, layer.mouse_y()/ layer.char_h);
        match renderer.get_parent() {
            &sdl2::render::WindowParent(ref w) => w.set_title(mouse_str.as_slice()),
            _ => {},
        };

        /*layer.handle_event(event);
        if layer.button("Add data", 420, 20).draw(&renderer) {
            last = last + std::rand::random::<i32>().abs() % 7 - 3;
            datas[0].push(last);
        }

        layer.checkbox("Add data", &mut show_surface, 550, 20).draw(&renderer);

        if layer.textfield(&mut text, 420, 50, 400, 55)
            .default_text("Írj be egy számot, majd nyomj entert!")
            .draw(&renderer) {
            match std::from_str::FromStr::from_str(text.as_slice()) {
                Some(d) => {
                    datas[0].push(d);
                    text.clear();
                },
                None => {},
            }
        }
        layer.dropdown(vec!["", "One", "Two", "Three", "Four", "Five"].as_slice(), &mut dropdown_value,  420, 120).draw(&renderer);

        for i in range(0, dropdown_value) {
            layer.line_chart("Datas", 10, 10 + i as i32 *70, 410, 60).data(datas[i].as_slice()).draw(&renderer);
        }

        layer.line_chart("Datas", 10, 10 + 5 * 70, 410, 60)
            .data(datas[1].as_slice())
            .bottom_color(RGBA(82, 82, 82, 150))
            .top_color(RGB(60, 60, 60))
            .surface_color(if show_surface {Some(RGB(255, 255, 255))} else {None})
            .draw(&renderer);*/
        renderer.present();

        let keys = sdl2::keyboard::get_keyboard_state();
        if keys[sdl2::scancode::EscapeScanCode] {
            break 'main;
        }

        let _ = renderer.set_draw_color(sdl2::pixels::RGB(60 , 59, 64));
        let _ = renderer.clear();
        frame_count += 1;

        if current_tick >= next_frame_tick {
            fps = frame_count;
            next_frame_tick = current_tick + 1000;
            frame_count = 0;
        }
    }

    sdl2::quit();
}
