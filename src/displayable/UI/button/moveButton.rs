use sdl2::pixels::Color;
use sdl2::rect::Point;

use super::Displayable;
use super::UI;
use crate::Shape;

use super::{invert, display, in_bounds};
use super::{ManipMode, Mode};
use super::Button;

#[derive(Debug)]
//Undefined behaviour if bottomRight is not below and to the right of topLeft
pub struct MoveButton
{
    topLeft: Point,
    bottomRight: Point,
    texture: bmp::Image,
    clicked: bool,
}

impl Button for MoveButton
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
            Mode::Paused(_) => Mode::Paused(ManipMode::Move),
            Mode::Unpaused => Mode::Unpaused,
        }
    }

}

impl Displayable for MoveButton
{
    //Scales the input image
    fn display(&self) -> Vec<(Point, Color)>
    {
        display(self)
    }
}

impl UI for MoveButton
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

impl MoveButton
{
    pub fn new(topLeft: Point, bottomRight: Point, texture: &str) -> MoveButton
    {
        MoveButton{topLeft, bottomRight, texture: bmp::open(texture).expect(&format!("Error opening texture: {}", texture)), clicked: false}
    }
}
