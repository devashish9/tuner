use cpal::{traits::{DeviceTrait, HostTrait, StreamTrait}, StreamConfig};
use rustfft::{num_complex::Complex, FftPlanner};

fn main() {
    let stream = build_stream();
    loop {
    stream.play().expect("stream couldn't play?");
    }
}

fn build_stream() -> cpal::Stream {
    let host = cpal::default_host();
    let my_dev = host.default_input_device().expect("no default input device found");
    let config: StreamConfig = my_dev.default_input_config().expect("No default input config").into();
    let sample_rate = config.sample_rate.0;
    println!("Sample rate: {}", sample_rate);
    let mut stream_acc: Vec<f32> = Vec::new();
    let stream = my_dev.build_input_stream(&config,
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            stream_acc.extend(data.to_vec());
            if stream_acc.len() > sample_rate as usize {
                apply_fft(&stream_acc, sample_rate);
                stream_acc.clear();
            }
        }, 
        move |err| {
            print!("error: {}", err)
        }, None).expect("Could not build stream");
    
    stream
}

fn apply_fft(sample: &[f32], sample_rate: u32) {
    let mut datapoints: Vec<Complex<f32>> = sample.iter().map(|&x| Complex::new(x, 0.0)).collect();
    let mut planner = FftPlanner::<f32>::new();
    let fft = planner.plan_fft_forward(datapoints.len());
    fft.process(&mut datapoints);
    let mut frequency_magnitude_pairs: Vec<(usize, f32)> = datapoints[..1000]
        .iter()
        .enumerate()
        .map(|(i, &c)| {
            let magnitude = (c.re * c.re + c.im * c.im).sqrt();
            let frequency = i * (sample_rate as usize) / datapoints.len();
            (frequency, magnitude)
        })
        .collect();

    frequency_magnitude_pairs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    for &(frequency, magnitude) in frequency_magnitude_pairs.iter().take(5) {
        println!("Frequency {}, Magnitude: {}", frequency, magnitude);
    }
} 
