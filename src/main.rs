use rustfft::FFTplanner;
use rustfft::num_complex::Complex;
use rustfft::num_traits::{Zero,One};
use std::env;

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

fn to_samples(distances: Vec<u32>) -> Vec<Complex<f64>> {
    let max_distance = distances.iter().fold(0, |acc, distance| acc.max(*distance));
    let mut x = vec![Complex::zero(); (2 * max_distance + 1) as usize];

    x[0] = Complex::one();

    for distance in distances {
        x[distance as usize] = Complex::one()
    }

    return x;
}

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

fn main() {
    let (
        distance_to_test,
        shot_distances
    ) = read_input();
    let mut samples = to_samples(shot_distances);

    samples = fft(samples, false);
    samples = samples
        .iter()
        .map(|c| c * c)
        .collect();
    samples = fft(samples, true);

    println!("Can shoot at {:?}", samples
             .iter()
             .enumerate()
             .map(|(i, x)| if x.re > 0.5 { i as i32 } else { -1 })
             .filter(|x| *x != -1)
             .collect::<Vec<i32>>());

    println!("{} distances to test", distance_to_test.len());
}
