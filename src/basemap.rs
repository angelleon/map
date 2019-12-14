use std::fs::File;
use std::ops::{Add, Div, Mul, Sub};
use std::path::Path;
use std::time::Duration;

extern crate sdl2;

use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

extern crate sdl2_sys;
use sdl2_sys::{SDL_GetDisplayUsableBounds, SDL_Rect};

extern crate geometry;
use geometry::{Circle, Point2D, Shape2D};

fn map_val(val: f64, a: f64, b: f64, c: f64, d: f64) -> f64 {
    assert_ne!(a, b);
    let proportion = (val - a) / (b - a);
    c + (d - c) * proportion
}

pub struct BaseMap {
    map_pic_path: String,
    min_lat: f32,
    min_lon: f32,
    max_lat: f32,
    max_lon: f32,
    shapes: Vec<Shape2D>,
    drawer: Drawer
}

impl BaseMap {
    pub fn new(
        map_pic_path: String,
        min_lat: f32,
        min_lon: f32,
        max_lat: f32,
        max_lon: f32,
        title: String
    ) -> Self {
        BaseMap {
            map_pic_path,
            min_lat,
            min_lon,
            max_lat,
            max_lon,
            shapes: Vec::<Shape2D>::new(),
            drawer: Drawer::new(title)
        }
    }
    pub fn draw_circle_radi(&mut self, lat: f32, lon: f32, r: u32) {
        if lat >= self.min_lat && lat <= self.max_lat && lon >= self.min_lon && lon <= self.max_lon
        {
            let w_rect = Drawer::get_display_usable_bounds();
            // println!("desktop usable size: w: {}, h: {}", &w_rect.w, &w_rect.h);
            let x = map_val(
                lon as f64,
                self.min_lon as f64,
                self.max_lon as f64,
                0f64,
                w_rect.w as f64
            ) as f32;
            let y = map_val(
                lat as f64,
                self.min_lat as f64,
                self.max_lat as f64,
                w_rect.h as f64,
                0f64
            ) as f32;
            //println!("Circle constructor values: {{x: {}, y: {}, r: {}}}", &x, &y, &r);
            let c = Circle::new(x, y, r as f32);
            //println!("Drawing circle: {}", &c);
            self.shapes.push(Shape2D::CIRCLE(c));
        } else {
            //panic!("Invalid values {} {}", &lat, &lon);
        }
    }

    pub fn draw_circle_ang(&mut self, lat: f32, lon: f32, theta: f32) {
        //println!("Drawing circle lat: {}, lon: {}, theta: {}", &lat, &lon, &theta);
        let r = map_val(
            theta as f64,
            0f64,
            (self.max_lat - self.min_lat) as f64,
            0f64,
            90f64,
        ) as u32;
        //println!("r: {}", &r);
        if theta <= self.max_lat - self.min_lat && theta <= self.max_lon - self.min_lon {
            self.draw_circle_radi(lat, lon, r);
        } else {
            //panic!("Invalid values");
        }
    }

    pub fn show(&mut self) {
        let path = Path::new(&self.map_pic_path);
        self.drawer.show(&path, &self.shapes);
    }
}

struct Drawer {
    sdl_context: sdl2::Sdl,
    video_subsys: sdl2::VideoSubsystem,
    desktop_mode: sdl2::video::DisplayMode,
    bounds: SDL_Rect,
    title: String
}

impl Drawer {
    pub fn new(title: String) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsys = sdl_context.video().unwrap();
        let desktop_mode = video_subsys.desktop_display_mode(0).unwrap();
        //println!("height: {}", &desktop_mode.h);
        //println!("width: {}", &desktop_mode.w);
        let desktop_rect = Drawer::get_display_usable_bounds();
        

        Drawer {
            sdl_context,
            video_subsys,
            desktop_mode,
            bounds: desktop_rect,
            title
        }
    }

    fn get_display_usable_bounds() -> SDL_Rect {
        let mut rect: SDL_Rect = SDL_Rect {
                x: 0,
                y: 0,
                w: 0,
                h: 0
        };
        unsafe {
            SDL_GetDisplayUsableBounds(0, &mut rect);
        }
        rect
    }

    pub fn get_w(&self) -> i32 {
        self.bounds.w
    }

    pub fn get_h(&self) -> i32 {
        self.bounds.h
    }

    pub fn show(&mut self, map_pic_path: &Path, shapes: &Vec<Shape2D>) {
        let window = self.video_subsys
            .window(self.title.as_str(), self.bounds.w as u32, self.bounds.h as u32)
            .position(0, 0)
            //.fullscreen_desktop()
            .borderless()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();
        let image_context = sdl2::image::init(sdl2::image::InitFlag::PNG).unwrap();
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator.load_texture(map_pic_path).unwrap();
        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();
        canvas.present();
        
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        let c = Color::RGB(255, 0, 255);
        canvas.present();
        'running: loop {
            canvas.copy(&texture, None, None)
                  .expect("Error rendering image");
            for shape in shapes {
                match &shape {
                    Shape2D::CIRCLE(cir) => {
                        //println!("Drawing circle {}", &cir);
                        canvas.filled_circle(cir.x as i16, cir.y as i16, cir.r as i16, c)
                            .unwrap();
                    }
                    _ => {
                        //println!("Has not circle");
                        }
                }
            }
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
            canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}
