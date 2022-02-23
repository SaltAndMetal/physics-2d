#![allow(non_snake_case, dead_code)]

mod vec2;
use crate::vec2::*;

mod displayable;
use crate::displayable::{Displayable, physics::{Shape, Physics, Intersect, rect::Rect, circle::Circle}};

use crate::displayable::UI::{UI, button::Button, button::pauseButton::PauseButton};

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

use std::f64::consts;

const WINDOW_DIMENSIONS: (u32, u32) = (1000, 1000);

#[derive(PartialEq, Eq)]
enum PausedMode {
    Paused,
    Unpaused,
}

enum MouseMode {
    Move,
    Insert(Shape),
}

pub struct State {
    paused: PausedMode,
    mouseMode: MouseMode,
}

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
    let mut state = State{paused: PausedMode::Unpaused, mouseMode: MouseMode::Move};

    let (mut canvas, mut event_pump) = init();
    
    let mut buttons: Vec<Box<dyn Button + Send + Sync>> = vec![
        Box::new(PauseButton::new(Point::new(0, 0), Point::new(100, 100), "images/pause.bmp")),
        Box::new(PauseButton::new(Point::new(100, 0), Point::new(200, 100), "images/move.bmp")),
        Box::new(PauseButton::new(Point::new(200, 0), Point::new(300, 100), "images/circle.bmp")),
        Box::new(PauseButton::new(Point::new(300, 0), Point::new(400, 100), "images/rect.bmp"))
    ];

    let mut objects: Vec<Shape> = vec![
        Shape::Rect(Rect::from_centre(Vec2::new(-30.0, 200.0), Vec2::new(100.0, 200.0), consts::FRAC_PI_2)),
        Shape::Circle(Circle::new(Vec2::new(-390.0, 400.0), 150.0)),
    ];
    
    objects[0].impulse(&Vec2::new(0.0, -2.0));
    objects[1].impulse(&Vec2::new(2.0, -2.0));
    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..} => break 'running,
                Event::MouseButtonDown{x, y, ..} => {
                    for button in &mut buttons {
                        if button.in_bounds(Point::new(x, y)) {
                            button.click_down(&mut state);
                        }
                    }
                },

                Event::MouseButtonUp{..} => {
                    for button in &mut buttons {
                        if button.clicked() {
                            button.click_up();
                        }
                    }
                }
                _ => {}
            }
        }

        let points = Arc::new(Mutex::new(Vec::new()));
        
        thread::scope( |s| {
            for UI in &buttons {
                s.spawn(|_| {
                    let p = &mut UI.display();
                    points.lock().unwrap().append(p);
                });
            }
            for object in &objects {
                s.spawn(|_| {
                    let p = &mut object.display();
                    points.lock().unwrap().append(p);
                });
            }

        }).unwrap();
        if state.paused == PausedMode::Unpaused {
            thread::scope( |s| {
                for object in &mut objects {
                    s.spawn(|_| {
                        object.integrate();
                    });
                }
            }).unwrap();
        }

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
