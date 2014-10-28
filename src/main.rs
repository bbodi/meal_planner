
extern crate sdl2;
extern crate sdl2_ttf;


use sdl2::pixels::RGB;
use sdl2::pixels::RGBA;

const SCREEN_WIDHT: u32 = 1024;
const SCREEN_HEIGHT: u32 = 768;
//mod app_event;
//mod widget;
//mod button;
mod imgui;
mod button;
mod textfield;
mod line_chart;
mod checkbox;
mod dropdown;


fn main() {
    sdl2::init(sdl2::INIT_VIDEO);
    sdl2_ttf::init();

    let window = match sdl2::video::Window::new("rust-sdl2 demo: Video", sdl2::video::PosCentered, sdl2::video::PosCentered, SCREEN_WIDHT as int, SCREEN_HEIGHT as int, sdl2::video::SHOWN | sdl2::video::RESIZABLE) {
        Ok(window) => window,
        Err(err) => fail!(format!("failed to create window: {}", err))
    };
    //window.set_size(1280, 900);
    window.set_position(sdl2::video::PosCentered, sdl2::video::PosCentered);

    let renderer = match sdl2::render::Renderer::from_window(window, sdl2::render::DriverAuto, sdl2::render::ACCELERATED) {
        Ok(renderer) => renderer,
        Err(err) => fail!(format!("failed to create renderer: {}", err))
    };
    renderer.set_logical_size(SCREEN_WIDHT as int, SCREEN_HEIGHT as int);
    let _ = renderer.set_draw_color(sdl2::pixels::RGB(0, 0, 255));
    let _ = renderer.clear();

    let font = match sdl2_ttf::Font::from_file(&Path::new("DejaVuSansMono.ttf"), 128) {
        Ok(f) => f,
        Err(e) => fail!(e),
    };
    // render a surface, and convert it to a texture bound to the renderer
    let surface = match font.render_str_blended("Hello Rust!", sdl2::pixels::RGBA(255, 0, 0, 255)) {
        Ok(s) => s,
        Err(e) => fail!(e),
    };
    let texture = match renderer.create_texture_from_surface(&surface) {
        Ok(t) => t,
        Err(e) => fail!(e),
    };
    //renderer.set_viewport(&sdl2::rect::Rect::new(10, 10, 100, 100));
    renderer.copy(&texture, None, None);

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

    let mut layer = imgui::Layer::new();
    let mut frame_count = 0u32;
    let mut next_frame_tick = 0;
    let mut text = "".into_string();
    let mut show_surface = false;
    let mut dropdown_value = 0;
    'main : loop {
        sdl2::timer::delay(10);
        let current_tick = sdl2::timer::get_ticks();

        //layer.draw(&renderer);


        let event = match sdl2::event::poll_event() {
            sdl2::event::QuitEvent(_) => break 'main,
            e => e, 
            // _ => {},
        };
        layer.handle_event(event);
        {
            {
                //let mut widgets = vec![];
                //widgets.push((&chart as &widget::WidgetImpl, sdl2::rect::Rect::new(10, 10, 410, 410)));
                //widgets.push((&btn as &widget::WidgetImpl, sdl2::rect::Rect::new(420, 20, 62, 16)));
                //layer.draw2(&renderer, &widgets);
                if layer.button("Add data", 420, 20, 62, 16).draw(&renderer) {
                    last = last + std::rand::random::<i32>().abs() % 7 - 3;
                    datas[0].push(last);
                }
                
                layer.checkbox("Add data", &mut show_surface, 500, 20).draw(&renderer);

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

                layer.line_chart("Datas", 10, 10, 410, 60).data(datas[0].as_slice()).draw(&renderer);
                layer.line_chart("Datas", 10, 80, 410, 60)
                    .data(datas[1].as_slice())
                    .bottom_color(RGBA(82, 82, 82, 150))
                    .top_color(RGB(60, 60, 60))
                    .surface_color(if show_surface {Some(RGB(255, 255, 255))} else {None})
                    .draw(&renderer);
                layer.line_chart("Datas", 10, 150, 410, 60).data(datas[2].as_slice()).draw(&renderer);
                layer.line_chart("Datas", 10, 230, 410, 60).data(datas[3].as_slice()).draw(&renderer);
                renderer.present();
            } 
            {
                /*let mut widgets = vec![];
                widgets.push((&mut chart as &mut widget::WidgetImpl, sdl2::rect::Rect::new(10, 10, 410, 410)));
                widgets.push((&mut btn as &mut widget::WidgetImpl, sdl2::rect::Rect::new(420, 20, 62, 16)));
                layer.handle_event2(event, &mut widgets);*/
            }
        }

        /*if btn.hover {
            
        }*/

        let keys = sdl2::keyboard::get_keyboard_state();
        if keys[sdl2::scancode::EscapeScanCode] {
            break 'main;
        }
        if keys[sdl2::scancode::RScanCode] {
        } else if keys[sdl2::scancode::FScanCode] {
        }
        if keys[sdl2::scancode::LShiftScanCode] {
        }

        if keys[sdl2::scancode::Num1ScanCode] {
        }


        if keys[sdl2::scancode::LeftScanCode] {
        } else if keys[sdl2::scancode::RightScanCode] {
        }

        let (state, xrel, yrel) = sdl2::mouse::get_relative_mouse_state();
        let m1_pressed = state == sdl2::mouse::LEFTMOUSESTATE;

        renderer.set_draw_color(sdl2::pixels::RGB(60 , 59, 64));
        renderer.clear();
        frame_count += 1;

        if current_tick >= next_frame_tick {
            next_frame_tick = current_tick + 1000;
            frame_count = 0;
        }
    }

    sdl2::quit();
}
