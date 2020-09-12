mod complex;
use complex::Complex;
use std::f64::consts::PI;
use std::time::Instant;

#[inline]
fn apply_hann(signal: &[f64]) -> Vec<f64> {
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

fn dft(signal: &[f64]) -> Vec<Complex> {
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

fn fft(signal: &[f64]) -> Vec<Complex> {
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
        let left = fft(&even_elems[..]);
        let right = fft(&odd_elems[..]);
        let mut twiddles = Vec::with_capacity(signal.len() / 2);
        for k in 0..signal.len() / 2 {
            let twid = Complex::from_theta(-2.0 * PI * k as f64 / signal.len() as f64);
            output.push(left[k] + twid * right[k]);
            twiddles.push(twid);
        }
        for k in 0..signal.len() / 2 {
            output.push(left[k] - twiddles[k] * right[k]);
        }
    }
    output
}

fn stft(signal: &[f64], window: u32, shift: u32) -> Vec<Vec<Complex>> {
    let mut output = Vec::new();
    for i in 0..(signal.len() as u32 - window) / shift {
        output.push(fft(
            &apply_hann(&signal[i as usize..(i + window) as usize])[..]
        ));
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_fft() {
        let n = 2048;
        let test_signal = (0..n)
            .map(|x| {
                let theta = (2.0 * PI * (x * x) as f64 / n as f64);
                (theta / 2.0).sin() + theta.cos()
            })
            .collect::<Vec<f64>>();

        let now = Instant::now();
        let fft_res = fft(&test_signal);
        println!(
            "fft time: {} seconds",
            now.elapsed().as_micros() as f32 / 1_000_000.0
        );

        let now = Instant::now();
        let dft_res = dft(&test_signal);
        println!(
            "dft time: {} seconds",
            now.elapsed().as_micros() as f32 / 1_000_000.0
        );

        for i in 0..fft_res.len() {
            assert!((fft_res[i] - dft_res[i]).abs() < 1e-12);
        }
    }
    #[test]
    fn call_stft() {
        let window = 2048;
        let shift = 1024;
        let n = 32 * shift;
        let test_signal = (0..n)
            .map(|x| (2.0 * PI * (x * x) as f64 / n as f64).cos())
            .collect::<Vec<f64>>();

        let now = Instant::now();
        let fft_res = stft(&test_signal[..], window, shift);
        println!(
            "stft time: {} seconds",
            now.elapsed().as_micros() as f32 / 1_000_000.0
        );
    }
}
