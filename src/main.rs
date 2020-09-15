use hound::WavReader;
use fft;

fn main() {
    let wav_reader = WavReader::open("sin_440Hz.wav").unwrap();
    let pcm: Vec<i16> = wav_reader.into_samples::<i16>().map(|x|{x.unwrap()}).collect();
    let max = *pcm.iter().max().unwrap();
    let pcm: Vec<f64> = pcm.iter().map(|x| {*x as f64 / max as f64}).collect();
    //let output = fft::stft(&pcm[..], 256, 128);
    //for x in output[..3].iter() {
    //    for y in x {
    //        print!("{} ", y.abs());
    //    }
    //for x in &pcm[..2048] { println!("{}", x); }
    let output = fft::fft(&pcm[..2048]);
    for x in output { println!("{}", x); }

}
