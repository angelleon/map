use std::fs::File;
use std::ops::{Add, Div, Mul, Sub};

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

extern crate point;
pub use point::{Point2D, PointComp};

fn map_val(val: f64, a: f64, b: f64, c: f64, d: f64) -> f64 {
    let proportion = val - a / b - a;
    c + (d - c) * proportion
}

pub struct BaseMap {
    map_pic: File,
    min_lat: f32,
    min_lon: f32,
    max_lat: f32,
    max_lon: f32,
    title: String,
    points: Vec<Point2D<f32>>
}

impl BaseMap {
    pub fn new(
        path: &String,
        min_lat: f32,
        min_lon: f32,
        max_lat: f32,
        max_lon: f32,
        title: String
    ) -> Self {
        BaseMap {
            map_pic: File::open(path.as_str()).expect("Can not open world_map.png file"),
            min_lat,
            min_lon,
            max_lat,
            max_lon,
            title,
            points: Vec::<Point2D<f32>::new()
        }
    }



    
}

struct Drawer {
    sdl_context: sdl2::Sdl,
    video_subsys: sdl2::VideoSubsystem,
    desktop_mode: sdl2::video::DisplayMode,
    canvas: sdl2::render::Canvas<sdl2::video::Window>
}

impl Drawer {
    pub fn new(title: &String) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsys = sdl_context.video().unwrap();
        let desktop_mode = video_subsys.desktop_display_mode(0).unwrap();
        println!("height: {}", &desktop_mode.h);
        println!("width: {}", &desktop_mode.w);

        let window = video_subsys
            .window(title.as_str(), 800, 600)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        Drawer {
            sdl_context,
            video_subsys,
            desktop_mode,
            canvas
        }
    }

    pub fn draw_point(p: Point2D<f32>) {}

    pub fn draw_ellipse(center: Point2D<f32>, a: f32, b: f32) {

    }

    pub fn show(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 255, 255));
        self.canvas.clear();
        self.canvas.present();
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        let mut i = 0;
        'running: loop {
            i = (i + 1) % 255;
            self.canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
            self.canvas.clear();
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }
            // The rest of the game loop goes here...

            self.canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}