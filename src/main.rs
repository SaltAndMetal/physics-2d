#![allow(non_snake_case, dead_code)]

mod vec2;
use crate::vec2::*;

mod displayable;
use crate::displayable::{Displayable, physics::{detectIntersections, Shape, Physics, rect::Rect, circle::Circle}};

use crate::displayable::UI::button::{Button, pauseButton::PauseButton, moveButton::MoveButton, rectButton::RectButton, circleButton::CircleButton};

extern crate assert_approx_eq;
pub use assert_approx_eq::assert_approx_eq;

extern crate bmp;

extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::video::Window;
use sdl2::rect::Point;
use sdl2::mouse::MouseButton;

use std::time::Duration;

extern crate crossbeam;
use crossbeam::thread;

use std::sync::{Arc, Mutex};

const WINDOW_DIMENSIONS: (u32, u32) = (1000, 1000);
const DELTA_TIME: Duration = Duration::from_millis(1_000/60);

#[derive(Debug)]
pub enum ManipMode {
    Move,
    Carrying(Shape, Vec2),
    VelSetting(Shape, Vec2),
    AngVelSetting(Shape, Vec2),
    Resizing(Shape, Vec2,  Shape),
    Rotating(Shape, Vec2, Shape),
}

#[derive(Debug)]
pub enum Mode {
    Paused(ManipMode),
    Unpaused,
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
    let mut mode = Mode::Paused(ManipMode::Move);

    let (mut canvas, mut event_pump) = init();
    

    let mut buttons: Vec<Box<dyn Button + Send + Sync>> = vec![
        Box::new(PauseButton::new(Point::new(0, 0), Point::new(100, 100), "images/pause.bmp")),
        Box::new(CircleButton::new(Point::new(100, 0), Point::new(200, 100), "images/circle.bmp")),
        //Box::new(RectButton::new(Point::new(200, 0), Point::new(300, 100), "images/rect.bmp"))
    ];

    let mut objects: Vec<Shape> = vec![
        Shape::Circle(Circle::new(Vec2::new(0.0, -300.0), 100.0)),
        Shape::Circle(Circle::new(Vec2::new(-300.0, 0.0), 100.0)),
        Shape::Circle(Circle::new(Vec2::new(300.0, 0.0), 100.0)),
        Shape::Circle(Circle::new(Vec2::new(0.0, 300.0), 100.0)),
        Shape::Circle(Circle::new(Vec2::new(300.0, 300.0), 100.0)),
        Shape::Circle(Circle::new(Vec2::new(-300.0, 300.0), 100.0)),
        Shape::Circle(Circle::new(Vec2::new(300.0, -300.0), 100.0)),
        Shape::Circle(Circle::new(Vec2::new(-300.0, -300.0), 100.0)),
        Shape::Rect(Rect::from_centre(Vec2::new(-(WINDOW_DIMENSIONS.0 as f64)/2.0, -(WINDOW_DIMENSIONS.1 as f64)/2.0), Vec2::new(1.0, WINDOW_DIMENSIONS.1 as f64 * 2.0-1.0), 0.0)),
        Shape::Rect(Rect::from_centre(Vec2::new((WINDOW_DIMENSIONS.0 as f64)/2.0, (WINDOW_DIMENSIONS.1 as f64)/2.0), Vec2::new(1.0, WINDOW_DIMENSIONS.1 as f64 * 2.0-1.0), 0.0)),
        Shape::Rect(Rect::from_centre(Vec2::new(-(WINDOW_DIMENSIONS.0 as f64)/2.0, (WINDOW_DIMENSIONS.1 as f64)/2.0), Vec2::new(WINDOW_DIMENSIONS.1 as f64 * 2.0-1.0, 1.0), 0.0)),
        Shape::Rect(Rect::from_centre(Vec2::new((WINDOW_DIMENSIONS.0 as f64)/2.0, -(WINDOW_DIMENSIONS.1 as f64)/2.0), Vec2::new(WINDOW_DIMENSIONS.1 as f64 * 2.0-1.0, 1.0), 0.0)),
    ];
    objects[0].impulse(&Vec2::new(0.0, 1000.0));
    objects[1].impulse(&Vec2::new(1000.0, 0.0));
    objects[2].impulse(&Vec2::new(-1000.0, 0.0));
    objects[3].impulse(&Vec2::new(0.0, -1000.0));
    objects[4].impulse(&Vec2::new(-1000.0, -1000.0));
    objects[5].impulse(&Vec2::new(1000.0, -1000.0));
    objects[6].impulse(&Vec2::new(-1000.0, 1000.0));
    objects[7].impulse(&Vec2::new(1000.0, 1000.0));
    
    'running: loop {
        let mouse_state = event_pump.mouse_state();
        let mouse_pos = Vec2::from(Point::new(mouse_state.x(), mouse_state.y()));
        let shift = event_pump.keyboard_state().is_scancode_pressed(sdl2::keyboard::Scancode::LShift)||event_pump.keyboard_state().is_scancode_pressed(sdl2::keyboard::Scancode::RShift);
        let ctrl = event_pump.keyboard_state().is_scancode_pressed(sdl2::keyboard::Scancode::LCtrl)||event_pump.keyboard_state().is_scancode_pressed(sdl2::keyboard::Scancode::RCtrl);
        
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..} => break 'running,
                Event::MouseButtonDown{x, y, mouse_btn, ..} => {
                    for button in &mut buttons {
                        if button.in_bounds(Point::new(x, y)) {
                            button.click_down(&mut mode, &objects);
                        }
                    }
                    match mode {
                        Mode::Paused(ManipMode::Move) => {
                            let click = Vec2::from(Point::new(x, y));
                            let mut carried: Option<(usize, Vec2)> = None;
                            for (i, shape) in objects.iter().enumerate() {
                                if shape.pointIn(&click) {
                                    carried = Some((i, click-shape.position()));
                                    break;
                                }
                            }
                            if let Some((i, position)) = carried {
                                match mouse_btn {
                                    MouseButton::Left if shift => {
                                        let object = objects.remove(i);
                                        let archive = object.clone();
                                        mode = Mode::Paused(ManipMode::Resizing(object, position, archive));
                                    },
                                    MouseButton::Left if ctrl => {
                                        let object = objects.remove(i);
                                        mode = Mode::Paused(ManipMode::VelSetting(object, position));
                                    },
                                    MouseButton::Left => {
                                        let object = objects.remove(i);
                                        mode = Mode::Paused(ManipMode::Carrying(object, position));
                                    },
                                    MouseButton::Right if shift => {
                                        objects.remove(i);
                                    },
                                    MouseButton::Right if ctrl => {
                                        let object = objects.remove(i);
                                        mode = Mode::Paused(ManipMode::AngVelSetting(object, position));
                                    },
                                    MouseButton::Right => {
                                        let object = objects.remove(i);
                                        let archive = object.clone();
                                        mode = Mode::Paused(ManipMode::Rotating(object, position, archive));
                                    },
                                    _ => (),
                                }
                            }
                        },

                        _ => (),
                    }
                },

                Event::MouseButtonUp{..} => {
                    for button in &mut buttons {
                        if button.clicked() {
                            button.click_up(&objects);
                        }
                    }

                    match mode {
                        Mode::Paused(ManipMode::Carrying(shape, _)) => {
                            objects.push(shape);
                            mode = Mode::Paused(ManipMode::Move);
                        },

                        Mode::Paused(ManipMode::VelSetting(shape, _)) => {
                            objects.push(shape);
                            mode = Mode::Paused(ManipMode::Move);
                        },

                        Mode::Paused(ManipMode::AngVelSetting(shape, _)) => {
                            objects.push(shape);
                            mode = Mode::Paused(ManipMode::Move);
                        },

                        Mode::Paused(ManipMode::Resizing(shape, _, _)) => {
                            objects.push(shape);
                            mode = Mode::Paused(ManipMode::Move);
                        },

                        Mode::Paused(ManipMode::Rotating(shape, _, _)) => {
                            objects.push(shape);
                            mode = Mode::Paused(ManipMode::Move);
                        },

                        _ => (),
                    }
                },

                _ => {}
            }
        }

        let points = Arc::new(Mutex::new(Vec::new()));

        
        match mode {
            Mode::Paused(ManipMode::Carrying(ref mut shape, ref grabPos)) => {
                let p = &mut shape.display();
                points.lock().unwrap().append(p);
    
                //println!("{:?} {:?}", grabPos, mouse_pos);
                shape.translateTo(&(mouse_pos-*grabPos));
            },

            Mode::Paused(ManipMode::VelSetting(ref mut shape, ref grabPos)) => {
                let p = &mut shape.display();
                points.lock().unwrap().append(p);
    
                //println!("{:?} {:?}", grabPos, mouse_pos);
                shape.impulse(&(((mouse_pos-shape.position())-*grabPos)-shape.velocity()));
            },
    
            Mode::Paused(ManipMode::AngVelSetting(ref mut shape, ref grabPos)) => {
                let p = &mut shape.display();
                points.lock().unwrap().append(p);
    
                //println!("{:?} {:?}", grabPos, mouse_pos);
                let (_, angle) = (mouse_pos-shape.position()).polar();
                let (_, newAngle) = grabPos.polar();
                shape.angular_impulse((newAngle-angle)-shape.angular_velocity());
            },

            Mode::Paused(ManipMode::Resizing(ref mut shape, ref click, ref archive)) => {
                let p = &mut shape.display();
                points.lock().unwrap().append(p);
    
                shape.resize(click, &(mouse_pos-shape.position()), archive);
            },

            Mode::Paused(ManipMode::Rotating(ref mut shape, ref click, ref archive)) => {
                let p = &mut shape.display();
                points.lock().unwrap().append(p);
    
                shape.rotate(click, &(mouse_pos-shape.position()), archive);
            },
            _ => (),
        }

        if let Mode::Unpaused = mode {
            let mut archive = objects.clone();
            thread::scope( |s| {
                for object in objects.iter_mut() {
                    s.spawn(|_| {
                        object.integrate();
                    });
                }
            }).unwrap();
            let intersecting = detectIntersections(&objects);
            if intersecting.len() > 0 {
                for (i1, i2) in intersecting {
                    let (a, b) = archive.split_at_mut(i2);
                    let shape1 = &mut a[i1];
                    let archiveShape1 = shape1.clone();
                    let shape2 = &mut b[0];
                    if let Shape::Circle(_) = shape1 {
                        shape1.bounce(&shape2);
                    }
                    if let Shape::Circle(_) = shape2 {
                        shape2.bounce(&archiveShape1);
                    }
                }
                objects = archive;
            }
        }

        thread::scope( |s| {
            for object in &objects {
                s.spawn(|_| {
                    let p = &mut object.display();
                    points.lock().unwrap().append(p);
                });
            }
            for UI in &buttons {
                s.spawn(|_| {
                    let p = &mut UI.display();
                    points.lock().unwrap().append(p);
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
        std::thread::sleep(DELTA_TIME);
    }
}
