
extern crate sdl2;
extern crate sdl2_ttf;

const SCREEN_WIDHT: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

//mod line_chart;

struct Layer {
    texture: sdl2::render::Texture,
}

bitflags! {
    flags EventHandlingResult: u32 {
        const HANDLED   = 0x00000001,
        const NEED_REDRAW   = 0x00000010,
    }
}

trait WidgetImpl {
    fn handle_event(&mut self) -> EventHandlingResult;

    fn draw(&self);
}

struct Widget<'a> {
    widgetImpl: Box<WidgetImpl + 'a>,
    rect: sdl2::rect::Rect,
}

impl<'a> Widget<'a> {
    fn handle_event(&mut self) -> EventHandlingResult {
        self.widgetImpl.handle_event()
    }

    fn draw(&self) {
        self.widgetImpl.draw()
    }

    fn get_rect(&self) -> sdl2::rect::Rect {
        self.rect
    }
}

fn main() {
    sdl2::init(sdl2::INIT_VIDEO);
    sdl2_ttf::init();

    let window = match sdl2::video::Window::new("rust-sdl2 demo: Video", sdl2::video::PosCentered, sdl2::video::PosCentered, SCREEN_WIDHT as int, SCREEN_HEIGHT as int, sdl2::video::SHOWN | sdl2::video::RESIZABLE) {
        Ok(window) => window,
        Err(err) => fail!(format!("failed to create window: {}", err))
    };
    window.set_size(1280, 900);
    window.set_position(sdl2::video::PosCentered, sdl2::video::PosCentered);

    let renderer = match sdl2::render::Renderer::from_window(window, sdl2::render::DriverAuto, sdl2::render::ACCELERATED) {
        Ok(renderer) => renderer,
        Err(err) => fail!(format!("failed to create renderer: {}", err))
    };

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
    renderer.set_viewport(&sdl2::rect::Rect::new(10, 10, 100, 100));
    renderer.copy(&texture, None, None);
    renderer.present();



    let mut frame_count = 0u32;
    let mut next_frame_tick = 0;
    'main : loop {
        let current_tick = sdl2::timer::get_ticks();

        renderer.set_viewport(&sdl2::rect::Rect::new(10, 10, 100, 100));
        let _ = renderer.copy(&texture, None, None);

        renderer.present();

        match sdl2::event::poll_event() {
            sdl2::event::QuitEvent(_) => break 'main,
            _ => {},
        }
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

        let _ = renderer.clear();
        frame_count += 1;

        if current_tick >= next_frame_tick {
            next_frame_tick = current_tick + 1000;
            frame_count = 0;
        }
    }

    sdl2::quit();
}
