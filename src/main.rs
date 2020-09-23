use fft;
use std::io::Write;
use std::fs::File;
use hound::WavReader;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    in_file: String,

    #[structopt(short)]
    window: u32,

    #[structopt(short)]
    shift: u32,

    #[structopt(short)]
    out_file: Option<String>,
}

fn main() {
    let opt = Cli::from_args();
    let wav_reader = WavReader::open(&opt.in_file).unwrap();
    let pcm: Vec<i16> = wav_reader
        .into_samples::<i16>()
        .map(|x| x.unwrap())
        .collect();
    let max = *pcm.iter().max().unwrap();
    let pcm: Vec<f64> = pcm.iter().map(|x| *x as f64 / max as f64).collect();
    let out_path = opt.out_file.unwrap_or("output.csv".to_string());
    let mut out_file = match File::create(&out_path) {
        Err(why) => panic!("couldn't create {}: {}", out_path, why),
        Ok(file) => file,
    };
    let window = opt.window;
    let shift = opt.shift;
    let output = fft::stft(&pcm[..], window, shift);
    let line = (0..window).map(|x| { x.to_string() }).collect::<Vec<String>>().join(",");
    writeln!(out_file, "{}", line).unwrap();
    for x in output.iter() {
        let line = x.iter().map(|x| { x.abs().to_string() }).collect::<Vec<String>>().join(",");
        writeln!(out_file, "{}", line).unwrap();
    }
}
