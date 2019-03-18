// use rustfft::FFTplanner;
use rustfft::num_complex::Complex;
use std::env;

fn read_input() -> (Vec<Complex<f64>>, Vec<Complex<f64>>){
    let mut middle_flag_seen = false;
    let mut shot_distances: Vec<Complex<f64>> = vec![];

    let mut distance_to_test: Vec<Complex<f64>> = vec![];
    for arg in env::args().skip(1) {
        match arg.as_ref() {
            "/" => middle_flag_seen = true,
            _ => {

                match arg.parse::<f64>() {
                    Ok(val) => {
                        if middle_flag_seen {
                            distance_to_test.push(Complex::new(val, 0.))
                        } else {
                            shot_distances.push(Complex::new(val, 0.))
                        }
                    },
                    Err(_) => {}
                }
            }
        }
    }

    return (distance_to_test, shot_distances);
}

fn main() {
    let (distance_to_test, shot_distances) = read_input();
    println!("{} shot distances", shot_distances.len());
    println!("{} distances to test", distance_to_test.len());
}
