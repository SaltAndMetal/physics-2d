use sdl2::pixels::Color;
use sdl2::rect::Point;
use crate::Shape;
use super::Displayable;
use super::UI;

use super::{invert, display, in_bounds};
use super::{ManipMode, Mode};
use super::Button;

use super::super::super::physics::detectIntersections;

#[derive(Debug)]
//Undefined behaviour if bottomRight is not below and to the right of topLeft
pub struct PauseButton
{
    topLeft: Point,
    bottomRight: Point,
    texture: bmp::Image,
    clicked: bool,
}

impl Button for PauseButton
{
    fn topLeft(&self) -> Point {self.topLeft}
    fn bottomRight(&self) -> Point {self.bottomRight}
    fn texture(&self) -> &bmp::Image {&self.texture}
    fn mutTexture(&mut self) -> &mut bmp::Image {&mut self.texture}
    fn clicked(&self) -> bool {self.clicked}
    
    fn clickDown(&mut self, state: &mut Mode, objects: &Vec<Shape>)
    {
        invert(&mut self.texture);
        let new = match *state {
            Mode::Paused(_) => {
                if detectIntersections(objects).len() == 0 {
                    Some(Mode::Unpaused)
                }
                else {
                    println!("Cannot unpause while there are intersecting objects!");
                    None
                }
            },
            Mode::Unpaused => Some(Mode::Paused(ManipMode::Move)),
        };
        if let Some(newState) = new {
            *state = newState;
        }
    }

}

impl Displayable for PauseButton
{
    //Scales the input image
    fn display(&self) -> Vec<(Point, Color)>
    {
        display(self)
    }
}

impl UI for PauseButton
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

impl PauseButton
{
    pub fn new(topLeft: Point, bottomRight: Point, texture: &str) -> PauseButton
    {
        PauseButton{topLeft, bottomRight, texture: bmp::open(texture).expect(&format!("Error opening texture: {}", texture)), clicked: false}
    }
}
