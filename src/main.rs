#![allow(non_snake_case, dead_code)]

mod vec2;
use crate::vec2::*;

mod displayable;
use crate::displayable::{Displayable, physics::{Shape, Physics, Intersect, rect::Rect, circle::Circle}};

use crate::displayable::UI::{UI, button::Button};

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
use crossbeam::thread;

use std::sync::{Arc, Mutex};

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

    let mut objects: Vec<Shape> = vec![
        Shape::Rect(Rect::new(Vec2::new(0.0, 0.0), Vec2::new(100.0, 200.0), 0.0)),
        Shape::Circle(Circle::new(Vec2::new(-390.0, 100.0), 100.0)),
    ];

    'running: loop {
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

        let points = Arc::new(Mutex::new(Vec::new()));
        
        thread::scope( |s| {
            for UI in &UIs {
                s.spawn(|_| {
                    let p = &mut UI.display();
                    points.lock().unwrap().append(p);
                });
            }
            for object in &objects {
                s.spawn(|_| {
                    if objects[0].intersect(&objects[1]) {
                        println!("Intersecting!");
                    }
                    else {
                        println!("Not");
                    }
                    let p = &mut object.display();
                    points.lock().unwrap().append(p);
                });
            }
        }).unwrap();

        thread::scope( |s| {
            for object in &mut objects {
                s.spawn(|_| {
                    object.integrate();
                });
            }
        }).unwrap();

        let points = points.lock().unwrap();

        canvas.set_draw_color(points[0].1);
        let mut points = points.iter().peekable();
        while let Some((point, color)) = points.next() {
            canvas.draw_point(*point)
                .expect("Problem drawing to screen");
            if let Some(futColor) = points.peek() {
                if *color != futColor.1 {
                    canvas.set_draw_color(futColor.1);
                }
            }
        }

        canvas.present();
        std::thread::sleep(Duration::from_millis(1_000/60));
    }
}
