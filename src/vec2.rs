use std::ops::*;

use sdl2::rect::Point;
use crate::WINDOW_DIMENSIONS;

#[derive(Clone, Copy, Debug)]
pub struct Vec2
{
    x: f64,
    y: f64,
}

impl Vec2
{
    pub fn new(x: f64, y: f64) -> Vec2
    {
        Vec2{x, y}
    }

    pub fn x(&self) -> f64
    {
        self.x
    }

    pub fn y(&self) -> f64
    {
        self.y
    }

    pub fn len_squared(&self) -> f64
    {
        ((self.x as f64).powf(2.0_f64)+(self.y as f64).powf(2.0_f64)) as f64
    }
    pub fn len(&self) -> f64
    {
        (self.len_squared() as f64).powf(0.5_f64)
    }
    pub fn dot(fst: &Vec2, snd: &Vec2) -> f64
    {
        fst.x*snd.x+fst.y*snd.y
    }
    pub fn polar(&self) -> (f64, f64) {
        //Some trickery goes on here. I use the absolute value of x and y, putting the result in
        //the first quadrant, then I adjust the angle back to the correct quadrant using the
        //angleManip closure

        let mag = self.len();
        let angleManip = match (self.x(), self.y()) {
            (x, y) if x > 0.0 && y > 0.0 => |x| {x},
            (x, y) if x < 0.0 && y > 0.0 => |x| {std::f64::consts::PI-x},
            (x, y) if x < 0.0 && y < 0.0 => |x| {std::f64::consts::PI+x},
            (x, y) if x > 0.0 && y < 0.0 => |x| {std::f64::consts::TAU-x},
            _ => { if self.x() < 0.0 {|_| std::f64::consts::PI} else {|_| 0.0}},
        };
        let angle = 
            if self.x().abs() < 0.001 {
                match self.y() {
                    i if i > 0.0 => std::f64::consts::FRAC_PI_2,
                    i if i < 0.0 => 3.0*std::f64::consts::FRAC_PI_2,
                    _ => 0.0,
                }
            }
            else {
                angleManip((self.y()/self.x()).abs().atan())
            };
        (mag, angle)
    }
    pub fn from_polar(mag: f64, angle: f64) -> Vec2 {
        Vec2{ x: angle.cos()*mag, y: angle.sin()*mag }
    }
    pub fn rotate(&self, axis: &Vec2, angle: f64) -> Vec2 {
        let relativePos = *self-*axis;
        let (myMag, myAngle) = relativePos.polar();
        let rotated = Vec2::from_polar(myMag, myAngle+angle);
        rotated + *axis

    }
}

impl Add for Vec2
{
    type Output = Vec2;

    fn add(self, other: Self) -> Self::Output
    {
        Vec2{x: self.x+other.x, y: self.y+other.y}
    }
}

impl Sub for Vec2
{
    type Output = Vec2;

    fn sub(self, other: Self) -> Self::Output
    {
        Vec2{x: self.x-other.x, y: self.y-other.y}
    }
}

impl Mul<f64> for Vec2
{
    type Output = Vec2;

    fn mul(self, other: f64) -> Self::Output
    {
        Vec2{x: self.x*other, y: self.y*other}
    }
}

impl Div<f64> for Vec2
{
    type Output = Vec2;

    fn div(self, other: f64) -> Self::Output
    {
        Vec2{x: self.x/other, y: self.y/other}
    }
}

impl AddAssign for Vec2
{
    fn add_assign(&mut self, other: Self)
    {
        *self = *self + other
    }
}

impl PartialEq for Vec2
{
    fn eq(&self, other: &Self) -> bool
    {
        ((self.x-other.x).abs() < 1.0e-6) && ((self.y-other.y).abs() < 1.0e-6)
    }
}

impl From<Vec2> for Point
{
    fn from(vec2: Vec2) -> Point
    {
        let x = vec2.x;
        let y = vec2.y;
        let x = (x + (WINDOW_DIMENSIONS.0/2) as f64) as i32;
        let y = (-y + (WINDOW_DIMENSIONS.1/2) as f64) as i32;
        Point::new(x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_approx_eq;
    #[test]
    fn len_squared()
    {
        assert_approx_eq!(Vec2{x: 5.4, y: 6.3}.len_squared(), 68.85);
    }
    #[test]
    fn len()
    {
        assert_approx_eq!(Vec2{x: 3.0, y: 4.0}.len(), 5.0);
    }
    #[test]
    fn dot()
    {
        assert_approx_eq!(Vec2::dot(&Vec2{x: 1.0, y: 2.0}, &Vec2{x: 3.0, y: 4.0}), 11.0);
    }
    #[test]
    fn ops()
    {
        let vecA = Vec2{x: 3.0, y: 4.0};
        let vecB = Vec2{x: 4.0, y: 1.5};
        assert_eq!(vecA/2.0, Vec2{x: 1.5, y: 2.0});
        assert_eq!(vecA*2.0, Vec2{x: 6.0, y: 8.0});
        assert_eq!(vecA+vecB, Vec2{x: 7.0, y: 5.5});
        assert_eq!(vecA-vecB, Vec2{x: -1.0, y: 2.5});
    }
    #[test]
    fn polar()
    {
        assert_approx_eq!(Vec2{x: 1.0, y: 0.0}.polar().0, 1.0);
        assert_approx_eq!(Vec2{x: 1.0, y: 0.0}.polar().1, 0.0);

        assert_approx_eq!(Vec2{x: 1.0, y: 1.0}.polar().0, 2.0_f64.powf(0.5_f64));
        assert_approx_eq!(Vec2{x: 1.0, y: 1.0}.polar().1, std::f64::consts::FRAC_PI_4);

        assert_approx_eq!(Vec2{x: -1.0, y: 1.0}.polar().1, 3.0*std::f64::consts::FRAC_PI_4);
        assert_approx_eq!(Vec2{x: -1.0, y: -1.0}.polar().1, 5.0*std::f64::consts::FRAC_PI_4);
        assert_approx_eq!(Vec2{x: 1.0, y: -1.0}.polar().1, 7.0*std::f64::consts::FRAC_PI_4);

        assert_approx_eq!(Vec2{x: 0.0, y: 1.0}.polar().1, std::f64::consts::FRAC_PI_2);

        assert_approx_eq!(Vec2{x: 3.0, y: 2.0}.polar().0, 13.0_f64.powf(0.5_f64));
        assert_approx_eq!(Vec2{x: 3.0, y: 2.0}.polar().1, 0.5880026035);
    }
    #[test]
    fn from_polar()
    {
        assert_approx_eq!(Vec2::from_polar(1.0, 0.0).x(), 1.0);
        assert_approx_eq!(Vec2::from_polar(1.0, 0.0).y(), 0.0);

        assert_approx_eq!(Vec2::from_polar(2.0_f64.powf(0.5_f64), std::f64::consts::FRAC_PI_4).x(), 1.0);
        assert_approx_eq!(Vec2::from_polar(2.0_f64.powf(0.5_f64), std::f64::consts::FRAC_PI_4).y(), 1.0);

        assert_approx_eq!(Vec2::from_polar(1.0, std::f64::consts::FRAC_PI_2).x(), 0.0);
        assert_approx_eq!(Vec2::from_polar(1.0, std::f64::consts::FRAC_PI_2).y(), 1.0);

        assert_approx_eq!(Vec2::from_polar(13.0_f64.powf(0.5_f64), 0.5880026035).x(), 3.0);
        assert_approx_eq!(Vec2::from_polar(13.0_f64.powf(0.5_f64), 0.5880026035).y(), 2.0);
    }
    #[test]
    fn rotate()
    {
        let origin = Vec2{x: 0.0, y: 0.0};
        assert_approx_eq!(Vec2{x: 3.0, y: 2.0}.rotate(&origin, 0.0).x(), 3.0);
        assert_approx_eq!(Vec2{x: 3.0, y: 2.0}.rotate(&origin, 0.0).y(), 2.0);

        assert_approx_eq!(Vec2{x: 3.0, y: 2.0}.rotate(&origin, std::f64::consts::FRAC_PI_2).x(), -2.0);
        assert_approx_eq!(Vec2{x: 3.0, y: 2.0}.rotate(&origin, std::f64::consts::FRAC_PI_2).y(), 3.0);
    }
}
