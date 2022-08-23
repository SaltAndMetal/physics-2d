use crate::vec2::Vec2;
use crate::WINDOW_DIMENSIONS;
use crate::DELTA_TIME;
use super::{Physics, Intersect, Shape};

use sdl2::rect::Point;
use sdl2::pixels::Color;

use super::super::Displayable;

#[derive(Debug, Clone, Copy)]
pub struct Circle
{
    centre: Vec2,
    radius: f64,
    velocity: Vec2,
    angular_velocity: f64,
    mass: f64,
}

impl Displayable for Circle
{
    fn display(&self) -> Vec<(Point, Color)>
    {
        let r2 = self.radius.powf(2_f64);
        
        //Computes how far away the centre of a pixel would be from the idealised point on the circle
        //The point given is relative to the centre of the circle
        let RE = |point: Vec2|
        {
            let d2 = point.len_squared();
            (d2-r2).abs()
        };

        let mut points = Vec::new();
        
        //Start at the top of the circle, draws one eighth
        let mut x = 0;
        let mut y = self.radius as i32;
        while x <= y {
            points.push(Vec2::new(x.into(), y.into()));
            let pointA = Vec2::new((x+1).into(), y.into());
            let pointB = Vec2::new((x+1).into(), (y-1).into());
            x += 1;
            if RE(pointB) < RE(pointA) {
                y -= 1;
            }
        }
        let points: Vec<(Point, Color)> = points.iter()
            .map(|a| vec![*a, Vec2::new(-a.x(), a.y()), Vec2::new(a.x(), -a.y()), Vec2::new(-a.x(), -a.y()), Vec2::new(a.y(), a.x()), Vec2::new(-a.y(), a.x()), Vec2::new(a.y(), -a.x()), Vec2::new(-a.y(), -a.x())]).flatten()
            .map(|a| a + self.centre)
            .map(|x| (Point::from(x), self.color())).collect();
        points
    }
}
impl Physics for Circle
{
    #[inline]
    fn position(&self) -> Vec2
    {
        self.centre
    }
    #[inline]
    fn translateTo(&mut self, point: &Vec2)
    {
        self.centre = *point;
    }
    #[inline]
    fn velocity(&self) -> Vec2
    {
        self.velocity
    }
    #[inline]
    fn angular_velocity(&self) -> f64
    {
        self.angular_velocity
    }
    #[inline]
    fn mass(&self) -> f64
    {
        self.mass
    }
    #[inline]
    fn impulse(&mut self, impulse: &Vec2)
    {
        self.velocity += *impulse;
    }
    #[inline]
    fn angular_impulse(&mut self, impulse: f64)
    {
        self.angular_velocity += impulse;
    }
    #[inline]
    fn integrate(&mut self)
    {
        self.centre += self.velocity*DELTA_TIME.as_millis() as f64/1000.0;
    }
    fn pointIn(&self, point: &Vec2) -> bool
    {
        (*point-self.centre).len_squared() <= self.radius.powf(2.0_f64)
    }
    fn resize(&mut self, point: &Vec2, newPoint: &Vec2, archive: &Self) {
        self.radius = (archive.radius - point.len()) + newPoint.len();
    }
    //Empty since a sphere cannot be rotated
    fn rotate(&mut self, _: &Vec2, _: &Vec2, _: &Self) {}

    fn bounce(&mut self, other: &Shape) {
        match other {
            Shape::Circle(circle) => {
                let v1 = self.velocity().len();
                let v2 = circle.velocity().len();
                let (_, theta1) = self.velocity().polar();
                let (_, theta2) = circle.velocity().polar();
                let (_, epsilon) = (circle.position()-self.position()).polar();
                let m1 = self.mass();
                let m2 = circle.mass();

                let initial = (v1*(theta1-epsilon).cos()*(m1-m2)+v2*2.0*m2*(theta2-epsilon).cos())/(m1+m2);
                let x = initial*epsilon.cos()+v1*(theta1-epsilon).sin()*(epsilon+std::f64::consts::FRAC_PI_2).cos();
                let y = initial*epsilon.sin()+v1*(theta1-epsilon).sin()*(epsilon+std::f64::consts::FRAC_PI_2).sin();
                println!("{:?}, {:?}", x, y);
                self.velocity = Vec2::new(x, y);
            },

            Shape::Rect(rect) => {
                let left = -(WINDOW_DIMENSIONS.0 as f64)/2.0+self.radius()+1.0;
                let right = -left;
                let bottom = -(WINDOW_DIMENSIONS.1 as f64)/2.0+self.radius()+1.0;
                let top = -bottom;
                println!("{}", left);
                println!("{}", self.centre().x());
                let mut new = self.clone();
                new.integrate();
                match (new.centre().x(), new.centre().y()) {
                    (x, y) if x < left => {
                        //self.centre = Vec2::new(left+1.0, y);
                        self.velocity = Vec2::new(self.velocity().x().abs(), self.velocity.y());
                    },
                    (x, y) if x > right => {
                        //self.centre = Vec2::new(right, y);
                        self.velocity = Vec2::new(-(self.velocity().x().abs()), self.velocity.y());
                    },
                    (x, y) if y < bottom => {
                        //self.centre = Vec2::new(x, bottom);
                        self.velocity = Vec2::new(self.velocity().x(), self.velocity.y().abs());
                    },
                    (x, y) if y > top => {
                        //self.centre = Vec2::new(x, top);
                        self.velocity = Vec2::new(self.velocity().x(), -(self.velocity.y().abs()));
                    },
                    _ => (),
                }
                //let angle = (self.position()-rect.position()).polar().1;
                //let relativeRot = rect.rotation()-angle;
            },
        }
    }
}

impl Intersect for Circle
{
    fn intersect(&self, other: &Shape) -> bool
    {
        match other {
            Shape::Circle(circle) => {
                (self.position()-circle.position()).len_squared() < (self.radius+circle.radius).powf(2.0_f64)
            }
            Shape::Rect(rect) => {
                rect.intersect(&Shape::Circle(*self))
            },
        }
    }
}

impl Circle
{
    #[inline]
    pub fn new(centre: Vec2, radius: f64) -> Circle
    {
        Circle{centre, radius, velocity: Vec2::zero(), angular_velocity: 0.0, mass: std::f64::consts::PI*radius.powf(2_f64)}
    }

    #[inline]
    pub fn new_with_mass(centre: Vec2, radius: f64, mass: f64) -> Circle
    {
        Circle{centre, radius, velocity: Vec2::zero(), angular_velocity: 0.0, mass}
    }

    #[inline]
    pub fn centre(&self) -> Vec2
    {
        self.centre
    }

    #[inline]
    pub fn radius(&self) -> f64
    {
        self.radius
    }
}
