use rustfft::FFTplanner;
use rustfft::num_complex::Complex;
use rustfft::num_traits::{Zero,One};
use std::env;
use std::f64::consts::PI;

// I/O
fn read_input() -> (Vec<u32>, Vec<u32>) {
    let mut middle_flag_seen = false;
    let mut shot_distances: Vec<u32> = vec![];

    let mut distance_to_test: Vec<u32> = vec![];
    for arg in env::args().skip(1) {
        match arg.as_ref() {
            "/" => middle_flag_seen = true,
            _ => {

                match arg.parse::<u32>() {
                    Ok(val) => {
                        if middle_flag_seen {
                            distance_to_test.push(val)
                        } else {
                            shot_distances.push(val)
                        }
                    },
                    Err(_) => {}
                }
            }
        }
    }

    return (distance_to_test, shot_distances);
}
fn print_successful_shots(samples: &Vec<Complex<f64>>, shots: &Vec<u32>) {
    let successful_shots = shots
        .iter()
        .filter(|&x| samples[(*x as usize)].re > 0.5)
        .map(|&x| x.to_string())
        .collect::<Vec<String>>();

    print!("{} shots are possible: {}", successful_shots.len(), successful_shots.join(", "));
}
fn print_possible_shot(samples: &Vec<Complex<f64>>) {
    println!("Can shoot at {:?}", samples
             .iter()
             .enumerate()
             .map(|(i, x)| if x.re > 0.5 { i as i32 } else { -1 })
             .filter(|x| *x != -1)
             .collect::<Vec<i32>>());
}

// Pick samples
fn to_samples(distances: &Vec<u32>) -> Vec<Complex<f64>> {
    let max_distance = distances.iter().fold(0, |acc, &distance| acc.max(distance));
    let size = (2 * max_distance + 1).next_power_of_two();

    let mut x = vec![Complex::zero(); size as usize];

    x[0] = Complex::one();
    for distance in distances {
        x[*distance as usize] = Complex::one()
    }

    return x;
}

// Custom FFT
fn rec_fft(samples: &Vec<Complex<f64>>, step: usize, start: usize, sign: &f64) -> Vec<Complex<f64>> {
    let n = samples.len() / step;
    if n > 1 {
        let mut out: Vec<Complex<f64>> = vec![];
        out.extend(rec_fft(samples, 2*step, start       , sign));
        out.extend(rec_fft(samples, 2*step, start + step, sign));

        for k in 0..n/2 {
            let c = Complex::from_polar(&1., &(-2. * (*sign) * PI * (k as f64) / (n as f64)));

            let sample1 = out[k];
            let sample2 = out[k+n/2];

            out[k]     = sample1 + c * sample2;
            out[k+n/2] = sample1 - c * sample2;
        }
        return out;
    } else {
        return vec![samples[start]];
    }
}
fn custom_fft(samples: Vec<Complex<f64>>, inverse: bool) -> Vec<Complex<f64>> {
    let sign = if inverse { -1. } else { 1. };
    return rec_fft(&samples, 1, 0, &sign);
}

// Lib FFT
fn fft(mut input: Vec<Complex<f64>>, inverse: bool) -> Vec<Complex<f64>> {
    let mut output: Vec<Complex<f64>> = input
        .iter()
        .map(|_| Complex::zero())
        .collect();
    let mut planner = FFTplanner::new(inverse);
    let fft = planner.plan_fft(input.len());
    fft.process(&mut input, &mut output);
    output
}

// Main
fn main() {
    let ( distance_to_test, shot_distances ) = read_input();

    println!("{} distances to test", distance_to_test.len());

    // FFT from lib
    println!("\nFrom rustfft lib");
    let mut samples = to_samples(&shot_distances);
    samples = fft(samples, false);
    samples = samples
        .iter()
        .map(|c| c * c)
        .collect();
    samples = fft(samples, true);
    print_possible_shot(&samples);
    print_successful_shots(&samples, &distance_to_test);

    // Custom FFT
    println!("\nFrom custom fft");
    let mut custom_samples = to_samples(&shot_distances);
    custom_samples = custom_fft(custom_samples, false);
    custom_samples = custom_samples
        .iter()
        .map(|c| c * c)
        .collect();
    custom_samples = custom_fft(custom_samples, true);
    print_possible_shot(&custom_samples);
    print_successful_shots(&custom_samples, &distance_to_test);
}
