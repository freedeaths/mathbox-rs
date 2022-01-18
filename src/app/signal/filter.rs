use crate::app::signal::transform::{dftfreq, irdft, rdft};
use crate::stats::estimator::median;
//use crate::opt::utils::local_minimax;

pub fn moving_average(signal: &[f64], window: usize) -> Vec<f64> {
    if window > signal.len() {
        panic!("Window size must be smaller than signal length");
    }
    let mut sum = 0.0;
    let mut result = vec![0.0; signal.len()];
    for i in 0..window {
        sum += signal[i];
        result[i] = sum / (i + 1) as f64;
    }
    for i in window..signal.len() {
        sum += signal[i] - signal[i - window];
        result[i] = sum / window as f64;
    }
    result
}

pub fn moving_median(signal: &[f64], window: usize) -> Vec<f64> {
    if window > signal.len() {
        panic!("Window size must be smaller than signal length");
    }
    let mut result = vec![0.0; signal.len()];
    for i in 0..signal.len() {
        if i < window {
            result[i] = median(&signal[0..i + 1]);
        } else {
            result[i] = median(&signal[i - window + 1..i + 1]);
        }
    }
    result
}

//pub fn fft_filter_lowpass(signal: &[f64], dt: f64, cutoff_frequency: f64, top_n: usize) -> Vec<f64> {
//
//}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_moving_average() {
        let signal = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let result = moving_average(&signal, 3);
        let expected = vec![1.0, 1.5, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        for i in 0..signal.len() {
            assert_relative_eq!(result[i], expected[i]);
        }
    }

    #[test]
    fn test_moving_median() {
        let signal = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let result = moving_median(&signal, 3);
        let expected = vec![1.0, 1.5, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        for i in 0..signal.len() {
            assert_relative_eq!(result[i], expected[i]);
        }
    }
}
