use std::f64::consts::PI;

struct Complex {
    re: f64,
    im: f64,
}

impl Complex {
    #[inline]
    fn add(&self, rhs: &Complex) -> Complex {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
    #[inline]
    fn mul(&self, rhs: &Complex) -> Complex {
        Complex {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.im * rhs.re + self.re * rhs.im,
        }
    }
}

fn hann(t: f64, tau: f64, n: u32) -> f64 {
    let n = n as f64;
    let dt = t - tau;

    if dt >= n || dt <= -n {
        return 0.0;
    }
    let tmp = (PI * dt / n).cos();
    tmp * tmp
}   
    
//fn sfft
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
