use rustfft::FFTplanner;
use rustfft::num_complex::Complex;
use rustfft::num_traits::{Zero,One};
use std::env;

fn read_input() -> (Vec<u32>, Vec<u32>){
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

fn to_polynom(distances: Vec<u32>) -> Vec<Complex<f64>> {
    let max_distance = distances.iter().fold(0, |acc, distance| acc.max(*distance));
    let mut x = vec![Complex::zero(); (2 * max_distance + 1) as usize];

    x[0] = Complex::one();

    for distance in distances {
        x[distance as usize] = Complex::one()
    }

    return x;
}

fn main() {
    let (distance_to_test,shot_distances) = read_input();
    let mut samples = to_polynom(shot_distances);
    let mut convolued_samples: Vec<Complex<f64>> = samples.iter().map(|_| Complex::zero()).collect();

    // FFT
    let mut planner = FFTplanner::new(false);
    let fft = planner.plan_fft(samples.len());
    fft.process(&mut samples, &mut convolued_samples);

    convolued_samples = convolued_samples.iter().map(|c| c * c).collect();

    // IFFT
    let mut i_planner = FFTplanner::new(true);
    let i_fft = i_planner.plan_fft(samples.len());
    i_fft.process(&mut convolued_samples, &mut samples);

    // let is_reachable: Vec<bool> = sample
    //     .iter()
    //     .enumerate()
    //     .collect();

    println!("Can shoot at {:?}", samples
             .iter()
             .enumerate()
             .map(|(i, x)| if x.re > 0.5 { i as i32 } else { -1 })
             .filter(|x| *x != -1)
             .collect::<Vec<i32>>());




    // println!("{} shot distances", shot_distances.len());
    println!("{} distances to test", distance_to_test.len());
}
