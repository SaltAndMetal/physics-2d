#![allow(non_snake_case, dead_code)]

mod vec2;
use crate::vec2::*;

use std::time::Instant;

mod displayable;
use crate::displayable::physics::Physics;
use crate::displayable::physics::rect::Rect;
use crate::displayable::physics::circle::Circle;

use crate::displayable::UI::UI;
use crate::displayable::UI::button::Button;

extern crate assert_approx_eq;
pub use assert_approx_eq::assert_approx_eq;

extern crate bmp;

extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::video::Window;
use sdl2::rect::Point;

use std::time::Duration;

extern crate crossbeam;
use crossbeam_utils::thread;


const WINDOW_DIMENSIONS: (u32, u32) = (1000, 1000);

fn init() -> (sdl2::render::Canvas<Window>, sdl2::EventPump)
{
    let sdl_context = sdl2::init().expect("Error initialising");
    
    let video_subsystem = sdl_context.video().unwrap();
    
    let window = video_subsystem.window("rust-sdl2 demo", WINDOW_DIMENSIONS.0, WINDOW_DIMENSIONS.1)
        .position_centered()
        .build()
        .expect("Error creating window");
    
    let canvas = window.into_canvas().build().expect("Error creating canvas");

    let event_pump = sdl_context.event_pump().expect("Error creating event pump");
    
    (canvas, event_pump)
}

fn main()
{
    let (mut canvas, mut event_pump) = init();
    
    let mut UIs: Vec<Box<dyn UI + Send + Sync>> = vec![
        Box::new(Button::new(Point::new(0, 0), Point::new(100, 100), "images/rect.bmp")),
        Box::new(Button::new(Point::new(100, 100), Point::new(200, 200), "images/circle.bmp"))
    ];

    let objects: Vec<Box<dyn Physics + Send + Sync>> = vec![
        Box::new(Rect::new(Vec2::new(0.0, 0.0), Vec2::new(100.0, 100.0), std::f64::consts::FRAC_PI_8)),
        Box::new(Circle::new(Vec2::new(0.0, 100.0), 100.0))
    ];

    'running: loop {
        let now = Instant::now();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..} => break 'running,
                Event::MouseButtonDown{x, y, ..} => {
                    for UI in &mut UIs {
                        if UI.in_bounds(Point::new(x, y)) {
                            UI.click_down();
                        }
                    }
                },

                Event::MouseButtonUp{..} => {
                    for UI in &mut UIs {
                        if UI.clicked() {
                            UI.click_up();
                        }
                    }
                }
                _ => {}
            }
        }
        
        for UI in &UIs {
            UI.display(&mut canvas);
        }

        for object in &objects {
           object.display(&mut canvas);
        }

        let elapsed_time = now.elapsed();
        //println!("{}", elapsed_time.as_millis());
        canvas.present();
        std::thread::sleep(Duration::from_millis(1_000/60));
    }
}
