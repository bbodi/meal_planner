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
use imgui::button::button;
use imgui::textfield::textfield_i32;
use imgui::dropdown::dropdown;
use imgui::line_chart::line_chart;

const SCREEN_WIDHT: u32 = 1024;
const SCREEN_HEIGHT: u32 = 768;


mod db;

mod tricolor_field;
mod tricolor_label;

mod food_list_layer;
mod bmr_layer;
mod daily_plan;
mod weekly_plan;


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
    let mut last_meal_id = 0;
    let mut last_daily_menu_id = 0;
    for daily_menu in daily_menus.iter() {
        last_daily_menu_id = std::cmp::max(last_daily_menu_id, daily_menu.id());
        for meal in daily_menu.meals.iter() {
            last_meal_id = std::cmp::max(last_meal_id, meal.id());
        }
    }

    let mut layer = imgui::base::Layer::new();
    let mut nutr_goal = dao.load_nutritional_goals();
    let mut bmr_layer = bmr_layer::KCalWindow::new();
    let mut food_list_layer = food_list_layer::KCalTable::new();

    let mut weekly_plan = weekly_plan::WeeklyPlan::new();

    let mut daily_plan = daily_plan::DailyPlan::new();
    let mut show_cal_win = false;
    let mut show_table_win = true;
    let mut show_daily_win = false;
    let mut show_weekly_plan = false;
    let mut selected_menu_idx: uint = 0;

    /*let mut show_surface = false;
    let mut text = 0;
    let mut dropdown_value: i32 = 0;*/

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

        if layer.control_keys.alt.down {
            panel(&mut layer, SizeInCharacters(20), SizeInCharacters(2 + daily_menus.len() as i32 * 2))
                .x(SizeInCharacters(10))
                .y(SizeInCharacters(10))
                .draw();
            if button(&mut layer, "New...")
                .x(SizeInCharacters(10))
                .y(SizeInCharacters(10))
                .draw() {
                last_daily_menu_id = last_daily_menu_id + 1;
                daily_menus.push(db::DailyMenu::new(last_daily_menu_id, "Unnamed".into_string()));
                selected_menu_idx = 0;
            }
            let mut copy_idx = None;
            for (i, daily_menu) in daily_menus.iter().enumerate() {
                if button(&mut layer, daily_menu.name.as_slice())
                    .x(SizeInCharacters(10))
                    .y(SizeInCharacters(12+i as i32*2))
                    .draw() {
                    selected_menu_idx = i;
                }
                if button(&mut layer, "Copy")
                    .right(SizeInCharacters(1))
                    .draw() {
                    copy_idx = Some(i);
                }
            }
            if copy_idx.is_some() {
                let i = copy_idx.unwrap();
                last_daily_menu_id = last_daily_menu_id + 1;
                let mut new_daily_menu = db::DailyMenu::new(last_daily_menu_id, daily_menus[i].name.clone());
                for meal in daily_menus[i].meals.iter() {
                    last_meal_id = last_meal_id + 1;
                    let new_meal = db::Meal::from_meal(last_meal_id, meal);
                    new_daily_menu.add_meal(new_meal);
                }
                daily_menus.push(new_daily_menu);
            }
        } else if layer.control_keys.ctrl.down {
            panel(&mut layer, SizeInCharacters(20), SizeInCharacters(7))
                .x(SizeInCharacters(10))
                .y(SizeInCharacters(10))
                .draw();
            if checkbox(&mut layer, &mut show_cal_win)
                .label("Calorie window")
                .x(SizeInCharacters(10))
                .y(SizeInCharacters(10)).draw() && show_cal_win {
                show_daily_win = false;
                show_table_win = false;
                show_weekly_plan = false;
            }
            if checkbox(&mut layer, &mut show_table_win)
                .label("Food list window")
                .x(SizeInCharacters(10))
                .down(SizeInCharacters(1)).draw() && show_table_win {
                show_daily_win = false;
                show_cal_win = false;
                show_weekly_plan = false;
            }
            if checkbox(&mut layer, &mut show_daily_win)
                .label("Daily window")
                .x(SizeInCharacters(10))
                .down(SizeInCharacters(1)).draw() && show_daily_win {
                show_table_win = false;
                show_cal_win = false;
                show_weekly_plan = false;
            }
            if checkbox(&mut layer, &mut show_weekly_plan)
                .label("Weekly window")
                .x(SizeInCharacters(10))
                .down(SizeInCharacters(1)).draw() && show_weekly_plan {
                show_table_win = false;
                show_cal_win = false;
                show_daily_win = false;
                show_weekly_plan = true;
            }
        }
        
        if show_cal_win {
            if bmr_layer.do_logic(&event, &mut nutr_goal) {
                dao.persist_nutritional_goals(&nutr_goal);
            }
            bmr_layer.layer.draw(&renderer);
        }
        if show_table_win {
            if food_list_layer.do_logic(&event, &mut foods) {
                dao.persist_foods(foods.as_slice());
            }
            food_list_layer.layer.draw(&renderer);
        }
        if daily_menus.len() > 0 && show_daily_win {
            if daily_plan.do_logic(&event, foods.as_slice(), daily_menus.get_mut(selected_menu_idx), &nutr_goal, &mut last_meal_id) {
                dao.persist_daily_menu(daily_menus.as_mut_slice());
            }
            daily_plan.layer.draw(&renderer);
        }
        if show_weekly_plan {
            if weekly_plan.do_logic(&event, daily_menus.as_slice()) {
                //dao.persist_daily_menu(daily_menus.as_mut_slice());
            }
            weekly_plan.layer.draw(&renderer);
        }

        /*if button(&mut layer, "Add data").draw() {
            last = last + std::rand::random::<i32>().abs() % 7 - 3;
            datas[0].push(last);
        }

        checkbox(&mut layer, &mut show_surface)
            .label("Add data")
            .draw();

        match textfield_i32(&mut layer, &mut text, SizeInCharacters(55))
            .down(SizeInCharacters(0))
            .default_text("Írj be egy számot, majd nyomj entert!")
            .draw() {
            Some(imgui::textfield::Enter) => {
                datas[0].push(text);
            },
            _ => {}
        }
        dropdown(&mut layer, vec!["", "One", "Two", "Three", "Four", "Five"].as_slice(), &mut dropdown_value)
            .down(SizeInCharacters(0))
            .draw();

        for i in range(0, dropdown_value) {
            line_chart(&mut layer, "Datas", 10, 10 + i as i32 *70, 410, 60).data(datas[i as uint].as_slice()).draw();
        }

        line_chart(&mut layer, "Datas", 10, 10 + 5 * 70, 410, 60)
            .data(datas[1].as_slice())
            .bottom_color(RGBA(82, 82, 82, 150))
            .top_color(RGB(60, 60, 60))
            .surface_color(if show_surface {Some(RGB(255, 255, 255))} else {None})
            .draw();*/
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
