use crate::app::signal::transform::{dftfreq, idft, rdft};
use crate::opt::utils::{argsort, local_max};
use crate::stats::estimator::median;
use num::Complex;

//use crate::opt::utils::local_minimax;

pub fn moving_average<T: Into<f64> + Copy>(signal: &[T], window: usize) -> Vec<f64> {
    if window > signal.len() {
        panic!("Window size must be smaller than signal length");
    }
    let mut sum = 0.0;
    let mut result = vec![0.0; signal.len()];
    for i in 0..window {
        sum += signal[i].into();
        result[i] = sum / (i + 1) as f64;
    }
    for i in window..signal.len() {
        sum += signal[i].into() - signal[i - window].into();
        result[i] = sum / window as f64;
    }
    result
}

pub fn moving_median<T: Into<f64> + Copy>(signal: &[T], window: usize) -> Vec<f64> {
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

/// Fourier filter for `signal`.
/// 
/// `dt`: sample time step
/// 
/// `cutoff_frequency`: cut off low frequency components
/// 
/// `top_n`: output the top n frequency components of the signal
pub fn dft_filter_lowpass<X: Into<f64> + Copy, Y: Into<f64> + Copy, Z: Into<f64> + Copy>(
    signal: &[X],
    dt: Y,
    cutoff_frequency: Z,
    top_n: usize,
) -> (Vec<f64>, Vec<f64>) {
    let mut f_sig = rdft(signal); // real fourier transform
    let mut fix_flag = true;
    if f_sig[f_sig.len() - 1].im < 1e-10 {
        fix_flag = false;
    }
    let f_amp = f_sig.iter().map(|x| x.norm()).collect::<Vec<_>>(); // calculate amplitude

    // find top n amplitudes
    let prominence = (f_amp.iter().copied().fold(f64::NAN, f64::max)
        - f_amp.iter().copied().fold(f64::NAN, f64::min))
        * 0.001;
    let amp_max_pos = local_max(&f_amp, prominence); // open interval without dc component and the last point
    let amp_max = amp_max_pos.iter().map(|x| f_amp[*x]).collect::<Vec<_>>();
    let mut top_amp_pos = argsort(&amp_max, false);
    if top_amp_pos.len() > top_n {
        top_amp_pos.truncate(top_n);
    }
    //let top_n_amp_pos = top_amp_pos.iter().map(|x| amp_max_pos[*x]).collect::<Vec<_>>();  // use line 60
    // cut small amplitude
    for (i, item) in f_sig.iter_mut().enumerate() {
        //if !top_n_amp_pos.contains(&i) {  // use line 60
        if !top_amp_pos.iter().map(|x| amp_max_pos[*x]).any(|x| x == i) {
            *item = Complex::new(0., 0.);
        }
    }

    let sample_num = signal.len();
    let offset = f_amp[0] / sample_num as f64; // dc component
    let f = dftfreq(sample_num, dt);
    let f_half = f[..((sample_num - 1) / 2 + 1)].to_vec();
    let freq = top_amp_pos.iter().map(|x| f_half[*x]).collect::<Vec<_>>();

    // cut low frequency
    for i in 0..f_half.len() {
        if f_half[i] < cutoff_frequency.into() {
            f_sig[i] = Complex::new(0.0, 0.0);
        }
    }

    let mut conjugate: Vec<Complex<f64>> = f_sig[1..].to_vec();
    if fix_flag {
        conjugate = f_sig[1..f_sig.len() - 1].to_vec();
    }
    f_sig.extend(conjugate.iter().map(|x| x.conj()).rev().collect::<Vec<_>>());

    let mut filterd_sig = idft(&f_sig);
    filterd_sig = filterd_sig.iter().map(|x| x + offset).collect::<Vec<_>>(); // add dc component cuz it is not in top_n_amp_pos
                                                                              // is it possible that the last point is not in top_n_amp_pos?
    (freq, filterd_sig)
}

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
        let signal = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
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
        let signal = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let result = moving_median(&signal, 3);
        let expected = vec![1.0, 1.5, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        for i in 0..signal.len() {
            assert_relative_eq!(result[i], expected[i]);
        }
    }

    #[test]
    fn test_dft_filter_lowpass() {
        let signal = vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
        let (_, result) = dft_filter_lowpass(&signal, 1.0, 0.0, 10000);
        for i in 0..signal.len() {
            assert_relative_eq!(result[i], signal[i]);
        }
        let signal = vec![1.0, 2.0, 3.0, 4.0, 3.0, 2.0, 1.0, 2.0, 3.0, 4.0, 3.0, 2.0, 1.0];
        let (_, result) = dft_filter_lowpass(&signal, 1.0, 0.0, 10000);
        let expected = vec![
            1.177546665236456,
            2.220297731272131,
            3.4049979717345273,
            3.708219789800072,
            2.8680187999032034,
            1.610219856779601,
            1.021398370548015,
            1.6102198567796013,
            2.8680187999032127,
            3.708219789800072,
            3.4049979717345247,
            2.2202977312721237,
            1.1775466652364555,
        ];
        for i in 0..signal.len() {
            assert_relative_eq!(result[i], expected[i]);
        }
        let signal = vec![1, 1, 1, 1, 1, 1, 1, 1, 1];
        let (_, result) = dft_filter_lowpass(&signal, 1.0, 0.0, 10000);
        for i in 0..signal.len() {
            assert_relative_eq!(result[i], signal[i] as f64);
        }
        let signal = vec![1, 2, 3, 4, 3, 2, 1, 2, 3, 4, 3, 2, 1];
        let (_, result) = dft_filter_lowpass(&signal, 1, 0.0, 10000);
        let expected = vec![
            1.177546665236456,
            2.220297731272131,
            3.4049979717345273,
            3.708219789800072,
            2.8680187999032034,
            1.610219856779601,
            1.021398370548015,
            1.6102198567796013,
            2.8680187999032127,
            3.708219789800072,
            3.4049979717345247,
            2.2202977312721237,
            1.1775466652364555,
        ];
        for i in 0..signal.len() {
            assert_relative_eq!(result[i], expected[i]);
        }
    }
}
