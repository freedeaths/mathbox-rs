use num::Complex;

pub fn dft(signal: &[f64]) -> Vec<Complex<f64>> {
    let n = signal.len();
    let mut dft: Vec<Complex<f64>> = Vec::with_capacity(n);
    for k in 0..n {
        let mut sum = Complex::new(0.0, 0.0);
        //for t in 0..n {
        //    let theta = -(k as f64) * (2.0 * std::f64::consts::PI) * (t as f64) / (n as f64);
        //    let c = Complex::new(theta.cos(), theta.sin());
        //    sum += signal[t] * c;
        //}
        for (t, item) in signal.iter().enumerate().take(n) {
            let theta = -(k as f64) * (2.0 * std::f64::consts::PI) * (t as f64) / (n as f64);
            let c = Complex::new(theta.cos(), theta.sin());
            sum += item * c;
        }
        dft.push(sum);
    }
    dft
}

pub fn rdft(signal: &[f64]) -> Vec<Complex<f64>> {
    dft(signal)[..(signal.len() / 2) + 1].to_vec()
}

pub fn idft(signal: &[Complex<f64>]) -> Vec<f64> {
    let n = signal.len();
    let mut idft: Vec<f64> = Vec::with_capacity(n);
    for k in 0..n {
        let mut sum = 0.0;
        //for t in 0..n {
        //    let theta = (k as f64) * (2.0 * std::f64::consts::PI) * (t as f64) / (n as f64);
        //    let c = Complex::new(theta.cos(), theta.sin());
        //    sum += signal[t].re * c.re - signal[t].im * c.im;
        //}
        for (t, item) in signal.iter().enumerate().take(n) {
            let theta = (k as f64) * (2.0 * std::f64::consts::PI) * (t as f64) / (n as f64);
            let c = Complex::new(theta.cos(), theta.sin());
            sum += item.re * c.re - item.im * c.im;
        }
        idft.push(sum / (n as f64));
    }
    idft
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_dft() {
        let signal = vec![1.0, 2.0, 1.0, -1.0, 1.5];
        let dft = dft(&signal);
        let expected: Vec<Complex<f64>> = vec![
            Complex::new(4.5, 0.0),
            Complex::new(2.08155948, -1.65109876),
            Complex::new(-1.83155948, 1.60822041),
            Complex::new(-1.83155948, -1.60822041),
            Complex::new(2.08155948, 1.65109876),
        ];
        assert_eq!(dft.len(), signal.len());
        for i in 0..expected.len() {
            assert_relative_eq!(dft[i].re, expected[i].re, epsilon = 1e-6);
            assert_relative_eq!(dft[i].im, expected[i].im, epsilon = 1e-6);
        }
    }
    #[test]
    fn test_idft() {
        let signal = vec![1.0, 2.0, 1.0, -1.0, 1.5];
        let dft_signal = dft(&signal);
        let res = idft(&dft_signal);
        for i in 0..signal.len() {
            assert_relative_eq!(res[i], signal[i], epsilon = 1e-6);
        }
    }
}
