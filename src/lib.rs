mod complex;
use complex::Complex;
use std::f64::consts::PI;

#[inline]
fn apply_hann(signal: &Vec<f64>) -> Vec<f64> {
    let n = signal.len();
    let n = n as f64;

    signal
        .iter()
        .enumerate()
        .map(|(t, s)| {
            let tmp = (PI * t as f64 / n).cos();
            s * tmp * tmp
        })
        .collect::<Vec<f64>>()
}

fn dft(signal: &Vec<f64>) -> Vec<Complex> {
    let mut output = Vec::new();
    let mut sum = Complex::new(0.0, 0.0);

    for k in 0..signal.len() {
        sum.re = 0.0;
        sum.im = 0.0;
        for n in 0..signal.len() {
            let theta = -2.0 * PI * k as f64 * n as f64 / signal.len() as f64;
            sum = sum + Complex::new(signal[n], 0.0) * Complex::from_theta(theta);
        }
        output.push(sum);
    }
    output
}

fn fft(signal: &Vec<f64>) -> Vec<Complex> {
    let mut output = Vec::new();

    if signal.len() == 1 {
        output.push(Complex::new(signal[0], 0.0));
    } else {
        let mut even_elems: Vec<f64> = Vec::new();
        let mut odd_elems: Vec<f64> = Vec::new();
    
        let mut counter = 0;
        for elem in signal.iter() {
            if counter & 1 == 0 {
                even_elems.push(*elem);
            } else {
                odd_elems.push(*elem);
            }
            counter += 1;
        }
        let left = fft(&even_elems);
        let right = fft(&odd_elems);
        for k in 0..signal.len()/2 {
            let twiddle = Complex::from_theta(-2.0 * PI * k as f64 / signal.len() as f64);
            output.push(left[k] + twiddle * right[k]);
        }
        for k in 0..signal.len()/2 {
            let twiddle = Complex::from_theta(-2.0 * PI * k as f64 / signal.len() as f64);
            output.push(left[k] - twiddle * right[k]);
        }
    }
    output
}

//fn sfft
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_fft() {
        let test_signal = vec![0.2, 0.4, 1.05, 3.0, 2.0, 2.0, -1.4, -4.3];
        let fft_res = fft(&test_signal);
        let dft_res = dft(&test_signal);
        println!("{}", "start");
        for i in 0..fft_res.len() {
            println!("fft[{}], {}",i, fft_res[i]);
            println!("dft[{}], {}",i, dft_res[i]);
            assert!((fft_res[i] - dft_res[i]).abs() < 1e-12);
        }
    }
}
