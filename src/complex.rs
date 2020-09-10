use std::ops::{Add, Sub, Mul};
use std::fmt;

#[derive(Copy, Clone)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.re, self.im)
    }
}

impl Add for Complex {
    type Output = Complex;

    #[inline]
    fn add(self, rhs: Complex) -> Complex {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}
    
impl Sub for Complex {
    type Output = Complex;

    #[inline]
    fn sub(self, rhs: Complex) -> Complex {
        Complex {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl Mul for Complex {
    type Output = Complex;

    #[inline]
    fn mul(self, rhs: Complex) -> Complex {
        Complex {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.im * rhs.re + self.re * rhs.im,
        }
    }
}

impl Complex {
    pub fn new(re: f64, im: f64) -> Complex {
        Complex { re, im }
    }

    #[inline]
    pub fn from_theta(theta: f64) -> Complex {
        Complex {
            re: theta.cos(),
            im: theta.sin(),
        }
    }

    #[inline]
    pub fn abs(&self) -> f64 {
        self.re * self.re + self.im * self.im
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
