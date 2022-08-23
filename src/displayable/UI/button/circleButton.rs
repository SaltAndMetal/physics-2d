use sdl2::pixels::Color;
use sdl2::rect::Point;

use super::Displayable;
use super::UI;
use crate::Shape;
use crate::Circle;
use crate::Vec2;

use super::{invert, display, in_bounds};
use super::{ManipMode, Mode};
use super::Button;

const DEFAULT_RADIUS: f64 = 50.0;

#[derive(Debug)]
//Undefined behaviour if bottomRight is not below and to the right of topLeft
pub struct CircleButton
{
    topLeft: Point,
    bottomRight: Point,
    texture: bmp::Image,
    clicked: bool,
}

impl Button for CircleButton
{
    fn topLeft(&self) -> Point {self.topLeft}
    fn bottomRight(&self) -> Point {self.bottomRight}
    fn texture(&self) -> &bmp::Image {&self.texture}
    fn mutTexture(&mut self) -> &mut bmp::Image {&mut self.texture}
    fn clicked(&self) -> bool {self.clicked}
    
    fn clickDown(&mut self, state: &mut Mode, _objects: &Vec<Shape>)
    {
        invert(&mut self.texture);

        *state = match *state {
            Mode::Paused(_) =>
            {
                let circle = Shape::Circle(Circle::new(Vec2::from(Point::new(-1000, -1000)), DEFAULT_RADIUS));
                Mode::Paused(ManipMode::Carrying(circle, Vec2::zero()))
            },
            Mode::Unpaused => 
            {
                println!("Pause first!");
                Mode::Unpaused
            },
        }
    }

}

impl Displayable for CircleButton
{
    //Scales the input image
    fn display(&self) -> Vec<(Point, Color)>
    {
        display(self)
    }
}

impl UI for CircleButton
{
    fn in_bounds(&self, click: Point) -> bool
    {
        in_bounds(self, click)
    }

    fn click_down(&mut self, state: &mut Mode, objects: &Vec<Shape>)
    {
        self.clickDown(state, objects);
        self.clicked = true;
    }
    fn click_up(&mut self, _objects: &Vec<Shape>)
    {
        invert(&mut self.texture);
        self.clicked = false;
    }
}

impl CircleButton
{
    pub fn new(topLeft: Point, bottomRight: Point, texture: &str) -> CircleButton
    {
        CircleButton{topLeft, bottomRight, texture: bmp::open(texture).expect(&format!("Error opening texture: {}", texture)), clicked: false}
    }
}
