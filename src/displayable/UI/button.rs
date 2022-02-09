use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::Point;

use crate::Vec2;
use super::Displayable;
use super::UI;

use bmp::Pixel;
use bmp::px;

#[derive(Debug)]

//Undefined behaviour if bottomRight is not below and to the right of topLeft
pub struct Button
{
    topLeft: Point,
    bottomRight: Point,
    texture: bmp::Image,
    clicked: bool,
}

impl Displayable for Button
{
    //Scales the input image
    fn display(&self) -> Vec<(Point, Color)> {
        let size = self.bottomRight-self.topLeft;
        let scale = Vec2::new(size.x() as f64/self.texture.get_width() as f64, size.y() as f64/self.texture.get_height() as f64);
        
        let mut points = Vec::new();

        for (x, y) in self.texture.coordinates() {
            let px = self.texture.get_pixel(x, y);
            
            points.push((Point::from(self.topLeft)+Point::new((x as f64*scale.x()) as i32, (y as f64*scale.y() as f64) as i32), Color::RGB(px.r, px.g, px.b)));

        }
        points
    }
}

impl UI for Button
{
    fn in_bounds(&self, click: Point) -> bool
    {
        click.x()>self.topLeft.x() as i32 &&click.y()>self.topLeft.y() as i32 &&click.x()<self.bottomRight.x() as i32 &&click.y()<self.bottomRight.y() as i32 
    }

    fn click_down(&mut self)
    {
        self.clicked = true;
        for (x, y) in self.texture.coordinates() {
            let px = self.texture.get_pixel(x, y);
            self.texture.set_pixel(x, y, px![255-px.b, 255-px.g, 255-px.b])
        }
    }
    fn click_up(&mut self)
    {
        self.click_down();
        self.clicked = false;
    }
    fn clicked(&self) -> bool {
        self.clicked
    }
}

impl Button
{
    pub fn new(topLeft: Point, bottomRight: Point, texture: &str) -> Button
    {
        Button{topLeft, bottomRight, texture: bmp::open(texture).expect(&format!("Error opening texture: {}", texture)), clicked: false}
    }
}
