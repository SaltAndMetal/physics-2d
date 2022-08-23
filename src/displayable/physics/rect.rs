use crate::vec2::Vec2;
use crate::DELTA_TIME;
use super::{Physics, Intersect, Shape, circle::Circle};
use super::super::Displayable;

use sdl2::pixels::Color;
use sdl2::rect::Point;

extern crate bresenham;
use bresenham::Bresenham;

#[derive(Debug, Clone, Copy)]
pub struct Rect
{
    points: [Vec2; 4],
    rotation: f64,
    velocity: Vec2,
    angular_velocity: f64,
    mass: f64,
}

impl Displayable for Rect
{
    fn display(&self) -> Vec<(Point, Color)>
    {
        let extr = |a: Vec2| {let a = Point::from(a); (a.x() as isize, a.y() as isize)};
        let revExtr = |a: (isize, isize)| {(Point::new(a.0 as i32, a.1 as i32), self.color())};

        let points: Vec<(Point, Color)> = Bresenham::new(extr(self.points[0]), extr(self.points[1]))
            .chain(Bresenham::new(extr(self.points[1]), extr(self.points[2])))
            .chain(Bresenham::new(extr(self.points[2]), extr(self.points[3])))
            .chain(Bresenham::new(extr(self.points[3]), extr(self.points[0])))
            .map(|a| revExtr(a))
            .collect();
            
        points
    }
}

impl Physics for Rect
{
    #[inline]
    fn position(&self) -> Vec2
    {
        (self.points[0]+self.points[2])/2.0
    }
    #[inline]
    fn translateTo(&mut self, point: &Vec2)
    {
        let difference = *point - self.position();
        for i in  0..4 {
            self.points[i] += difference;
        }
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
        self.velocity += *impulse
    }
    #[inline]
    fn angular_impulse(&mut self, impulse: f64)
    {
        self.angular_velocity += impulse;
    }
    fn integrate(&mut self, gravity: &Vec2)
    {
        let ang = self.angular_velocity;
        let vel = self.velocity;
        for i in 0..self.points.len() {
            self.points[i]+=self.velocity*DELTA_TIME.as_millis() as f64/1000.0;
        }
        *self = Rect::from_centre(self.position(), self.size(), self.rotation()+self.angular_velocity()*DELTA_TIME.as_millis() as f64/1000.0);
        self.impulse(&vel);
        self.angular_impulse(ang);
    }
    fn pointIn(&self, point: &Vec2) -> bool
    {
        let points = self.points();

        //crossing number
        let mut cn = 0;

        for i in 0..4 {
            let i1 = 
                if i == 3 { 0 }
                else {i+1};
            let t = (point.y()-points[i1].y())/(points[i].y()-points[i1].y());
            if t > 0.0 && t < 1.0 {
                let x = points[i].x()*t + points[i1].x()*(1.0-t);
                if x > point.x() {
                    cn += 1;
                }
            }
        }
        cn%2 == 1
    }
    fn resize(&mut self, point: &Vec2, newPoint: &Vec2, archive: &Self) {
        let rotPoint = point.rotate(&Vec2::zero(), -self.rotation());
        let rotNewPoint = newPoint.rotate(&Vec2::zero(), -self.rotation());
        let x = match rotPoint.x() {
            x if x != 0.0 => rotPoint.x().abs()/rotPoint.x(),
            _ => 0.0,
        };
        let y = match rotPoint.y() {
            y if y != 0.0 => rotPoint.y().abs()/rotPoint.y(),
            _ => 0.0,
        };
        let quadrant = Vec2::new(x, y);
        let newSize = (rotNewPoint-rotPoint)*2.0;
        let newSize = Vec2::new(newSize.x()*quadrant.x(), newSize.y()*quadrant.y());
        let newSize = archive.size() + newSize;
        let newSize = Vec2::new(newSize.x().abs(), newSize.y().abs());
        //println!("{:?}", quadrant);

        //println!("{:?} {:?} {:?}", newSize, archive.size(), rotNewPoint-rotPoint);

        *self = Rect::from_centre(self.position(), newSize, self.rotation());
    }
    fn rotate(&mut self, point: &Vec2, newPoint: &Vec2, archive: &Self) {
        let (_, angle) = point.polar();
        let (_, newAngle) = newPoint.polar();
        let newRotation = archive.rotation() + (newAngle-angle);
        *self = Rect::from_centre(self.position(), self.size(), newRotation);
    }
    fn bounce(&mut self, other: &Shape) {
        match other {
            &Shape::Circle(circle) => {
            },
            &Shape::Rect(rect) => {
            }
        }
    }
}

impl Intersect for Rect
{
    fn intersect(&self, other: &Shape) -> bool
    {
        match other {
            Shape::Circle(circle) => {
                let rotCirc = Circle::new(circle.centre().rotate(&self.position(), -self.rotation()), circle.radius());

                //println!("{:?}", circle.centre());
                //println!("{:?}", rotCirc.centre());

                let rotRect = Rect::from_centre(self.position(), self.size(), 0.0);
                let points = rotRect.points();

                //println!("{:?}", points);
                //println!("{:?}", self.points());
                //println!("{:?}", rotRect.size());
                #[derive(Debug)]
                enum XDirection {
                    Left,
                    Middle,
                    Right,
                }
                
                #[derive(Debug)]
                enum YDirection {
                    Above,
                    Middle,
                    Below,
                }

                let xDirection = match rotCirc.position().x() {
                    x if x < points[0].x() => XDirection::Left,
                    x if x > points[2].x() => XDirection::Right,
                    _ => XDirection::Middle,
                };
                //println!("{:?}", xDirection);

                let yDirection = match rotCirc.position().y() {
                    y if y > points[2].y() => YDirection::Above,
                    y if y < points[0].y() => YDirection::Below,
                    _ => YDirection::Middle,
                };
                //println!("{:?}", yDirection);

                let closestPoint = match (xDirection, yDirection) {
                    (XDirection::Left, YDirection::Above) => points[1],
                    (XDirection::Left, YDirection::Middle) => Vec2::new(points[0].x(), rotCirc.position().y()),
                    (XDirection::Left, YDirection::Below) => points[0],
                    (XDirection::Middle, YDirection::Above) =>  Vec2::new(rotCirc.position().x(), points[1].y()),
                    (XDirection::Middle, YDirection::Middle) => return true,
                    (XDirection::Middle, YDirection::Below) =>  Vec2::new(rotCirc.position().x(), points[0].y()),
                    (XDirection::Right, YDirection::Above) => points[2],
                    (XDirection::Right, YDirection::Middle) => Vec2::new(points[3].x(), rotCirc.position().y()),
                    (XDirection::Right, YDirection::Below) => points[3],
                };
                if (closestPoint-rotCirc.position()).len_squared() <= rotCirc.radius().powf(2.0_f64) {
                    true
                }
                else {
                    false
                }
            },
            //Seperating axis test
            Shape::Rect(rect) => {
                let sPoints = self.points();
                let oPoints = rect.points();
                 
                let normalA = (sPoints[0]-sPoints[1]).perpendicular();
                let normalB = (sPoints[1]-sPoints[2]).perpendicular();
                let normalC = (oPoints[0]-oPoints[1]).perpendicular();
                let normalD = (oPoints[1]-oPoints[2]).perpendicular();
                let normals: [Vec2; 4] = [normalA, normalB, normalC, normalD];
                
                for normal in normals {
                    let mut sProjections: Vec<f64> = Vec::with_capacity(4);
                    for point in sPoints {
                        sProjections.push(Vec2::dot(&normal, &point));
                    }

                    let mut oProjections: Vec<f64> = Vec::with_capacity(4);
                    for point in oPoints {
                        oProjections.push(Vec2::dot(&normal, &point));
                    }

                    sProjections.sort_by(|a, b| a.partial_cmp(b).unwrap());
                    oProjections.sort_by(|a, b| a.partial_cmp(b).unwrap());

                    //Detects if the 2 regions are not intersecting
                    if !((sProjections[0] < oProjections[3] && sProjections[0] > oProjections[0]) ||
                    (oProjections[0] < sProjections[3] && oProjections[0] > sProjections[0])) {
                        return false;
                    }
                }

                true
            }
        }
    }
}

impl Rect
{
    pub fn from_centre(centre: Vec2, size: Vec2, rotation: f64) -> Rect
    {
        let pointA = (centre - size/2.0).rotate(&centre, rotation);
        let pointB = (centre - Vec2::new(size.x(), -size.y())/2.0).rotate(&centre, rotation);
        let pointC = (centre + size/2.0).rotate(&centre, rotation);
        let pointD = (centre - Vec2::new(-size.x(), size.y())/2.0).rotate(&centre, rotation);
        
        let points = [pointA, pointB, pointC, pointD];
        Rect{points, rotation, velocity: Vec2::zero(), angular_velocity: 0.0, mass: size.x()*size.y()}
    }
       
    pub fn from_centre_with_mass(centre: Vec2, size: Vec2, rotation: f64, mass: f64) -> Rect
    {
        let pointA = (centre - size/2.0).rotate(&centre, rotation);
        let pointB = (centre - Vec2::new(size.x(), -size.y())/2.0).rotate(&centre, rotation);
        let pointC = (centre + size/2.0).rotate(&centre, rotation);
        let pointD = (centre - Vec2::new(-size.x(), size.y())/2.0).rotate(&centre, rotation);
        
        let points = [pointA, pointB, pointC, pointD];
        Rect{points, rotation, velocity: Vec2::zero(), angular_velocity: 0.0, mass}
    }

    #[inline]
    pub fn bottomLeft(&self) -> Vec2
    {
        self.points[0]
    }

    #[inline]
    pub fn size(&self) -> Vec2
    {
        self.points[2].rotate(&self.position(), -self.rotation)-self.points[0].rotate(&self.position(), -self.rotation)
    }

    #[inline]
    pub fn points(&self) -> [Vec2; 4]
    {
        self.points
    }

    #[inline]
    pub fn rotation(&self) -> f64 
    {
        self.rotation
    }
}


