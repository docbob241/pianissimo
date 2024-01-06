use hound::{SampleFormat, WavReader};
use rustfft::num_complex::Complex;
use rustfft::FftPlanner;
use wasm_bindgen::prelude::*;
mod utils;

#[wasm_bindgen(js_name = processBytes)]
pub fn process_bytes(file_data: &[u8]) -> Result<f32, JsError> {
    if file_data.len() == 0 {
        return Err(JsError::new("Can't get first byte of an empty file"));
    }

    let mut reader = WavReader::new(file_data).unwrap();

    // Extract audio samples as f32
    let samples: Vec<f32> = match reader.spec().sample_format {
        SampleFormat::Int => {
            let integer_samples: Vec<i32> = reader.samples::<i32>().map(|s| s.unwrap()).collect();
            integer_samples.iter().map(|&x| x as f32).collect()
        }
        SampleFormat::Float => reader.samples::<f32>().map(|s| s.unwrap()).collect(),
    };

    let fft_size = 1024; // Choose an appropriate FFT size based on your requirements
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(fft_size);

    // Zero-pad the input buffer to the nearest multiple of FFT size
    let input_len = samples.len();
    let pad_len = fft_size - input_len % fft_size;
    let mut input: Vec<Complex<f32>> = samples
        .iter()
        .map(|&x| Complex::new(x, 0.0))
        .chain(std::iter::repeat(Complex::new(0.0, 0.0)).take(pad_len))
        .collect();

    fft.process(&mut input);

    let sample_rate = reader.spec().sample_rate as f32;
    let bin_width = sample_rate / fft_size as f32;

    // Find the index of the maximum magnitude in the FFT result
    let max_index = input
        .iter()
        .enumerate()
        .max_by(|&(_, a), &(_, b)| a.norm().partial_cmp(&b.norm()).unwrap())
        .map(|(index, _)| index)
        .unwrap();

    // Calculate the corresponding frequency of the dominant note
    Ok(max_index as f32 * bin_width)
}
