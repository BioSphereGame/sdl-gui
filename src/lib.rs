extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

use logger;

pub fn say_hi() {
    logger::log(
        logger::PREFIX_DEBUG,
        format!("Booting {}SDL-GUI v{}{} up...",
            logger::COLOR_BOLD_GREEN,
            env!("CARGO_PKG_VERSION"),
            logger::COLOR_RESET,
        ).as_str()
    );
}

pub struct Screen {
    pub size: (u32, u32),
    pub position: (u32, u32),
    pub title: String,
    canvas: Canvas<Window>,
    event_pump: EventPump,
}
impl Screen {
    pub fn new(title: &str, width: u32, height: u32) -> Screen {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();

        let window = video_subsystem
            .window(title, width, height)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        logger::log(logger::PREFIX_DEBUG, "Creating Screen...");
        return Screen {
            size: (width, height),
            position: (0, 0),
            title: title.to_string(),
            canvas,
            event_pump,
        }
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
    }

    pub fn draw(&mut self) {
        self.canvas.present();
    }

    fn draw_polygon_by_points(&mut self, vertices: &[Point], color: Color) {
        self.canvas.set_draw_color(color);

        for i in 0..vertices.len() {
            let j = (i + 1) % vertices.len();
            self.canvas.draw_line(vertices[i], vertices[j]).unwrap();
        }
    }

    fn draw_filled_polygon_by_points(&mut self, vertices: &[Point], color: Color) {
        self.canvas.set_draw_color(color);

        let mut min_y = i32::max_value();
        let mut max_y = i32::min_value();
        for vertex in vertices {
            if vertex.y < min_y {
                min_y = vertex.y;
            }
            if vertex.y > max_y {
                max_y = vertex.y;
            }
        }

        for y in min_y..=max_y {
            let mut intersections = Vec::<i32>::new();

            for i in 0..vertices.len() {
                let j = (i + 1) % vertices.len();
                let vertex_i = vertices[i];
                let vertex_j = vertices[j];
    
                if (vertex_i.y <= y && vertex_j.y > y) || (vertex_i.y > y && vertex_j.y <= y) {
                    let x_intersection = vertex_i.x + ((y - vertex_i.y) * (vertex_j.x - vertex_i.x)) / (vertex_j.y - vertex_i.y);
                    intersections.push(x_intersection);
                }
            }

            intersections.sort();

            for i in (0..intersections.len()).step_by(2) {
                let x_start = intersections[i];
                let x_end = intersections[i + 1];
                self.canvas.draw_line(sdl2::rect::Point::new(x_start, y), sdl2::rect::Point::new(x_end, y)).unwrap();
            }
        }
    }

    pub fn draw_polygon(&mut self, vertices: &[(i32, i32)], color: &[u8]) {
        if color.len() != 4 {
            logger::log(logger::PREFIX_ERROR, "Array `color` in `add_polygon()` fn, must have 4 `u8` elements.");
            std::process::exit(1);
        }
        if vertices.len() < 2 {
            logger::log(logger::PREFIX_ERROR, "Array `vertices` in `add_polygon()` fn, must have at least 2 `(i32, i32)` elements.");
            std::process::exit(1);
        }

        let mut points: Vec<Point> = Vec::new();
        for vertex in vertices {
            points.push(Point::new(vertex.0, vertex.1));
        }
        self.draw_polygon_by_points(&points, Color::RGBA(color[0], color[1], color[2], color[3]));
    }

    pub fn draw_filled_polygon(&mut self, vertices: &[(i32, i32)], color: &[u8]) {
        if color.len() != 4 {
            logger::log(logger::PREFIX_ERROR, "Array `color` in `add_polygon()` fn, must have 4 `u8` elements.");
            std::process::exit(1);
        }
        if vertices.len() < 2 {
            logger::log(logger::PREFIX_ERROR, "Array `vertices` in `add_polygon()` fn, must have at least 2 `(i32, i32)` elements.");
            std::process::exit(1);
        }

        let mut points: Vec<Point> = Vec::new();
        for vertex in vertices {
            points.push(Point::new(vertex.0, vertex.1));
        }
        self.draw_filled_polygon_by_points(&points, Color::RGBA(color[0], color[1], color[2], color[3]));
    }

    pub fn get_all_pressed_buttons(&mut self) -> Vec<Keycode> {
        let mut pressed_buttons: Vec<Keycode> = Vec::new();
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => std::process::exit(0),
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => pressed_buttons.push(keycode),
                _ => {}
            }
        }
        return pressed_buttons;
    }

    pub fn update(&mut self) {
        let new_position = self.position;
        let new_size = self.size;
        let new_title = &self.title;

        let mut should_update = false;

        let window = self.canvas.window_mut();
        let current_position = window.position();
        if current_position != (new_position.0 as i32, new_position.1 as i32) {
            window.set_position(sdl2::video::WindowPos::Positioned(new_position.0 as i32), sdl2::video::WindowPos::Positioned(new_position.1 as i32));
            should_update = true;
        }

        let current_size = window.size();
        if current_size != new_size {
            window.set_size(new_size.0, new_size.1).unwrap();
            should_update = true;
        }

        let current_title = window.title();
        if current_title != new_title {
            window.set_title(new_title).unwrap();
        }

        if should_update {
            self.canvas.present();
        }
    }
}