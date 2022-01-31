use num::Complex;

/// Discrete Fourier transform 
pub fn dft<T: Into<f64> + Copy>(signal: &[T]) -> Vec<Complex<f64>> {
    let n = signal.len();
    let mut dft: Vec<Complex<f64>> = Vec::with_capacity(n);
    for k in 0..n {
        let mut sum = Complex::new(0.0, 0.0);
        //for t in 0..n {
        //    let theta = -(k as f64) * (2.0 * std::f64::consts::PI) * (t as f64) / (n as f64);
        //    let c = Complex::new(theta.cos(), theta.sin());
        //    sum += signal[t] * c;
        //}
        for (t, &item) in signal.iter().enumerate().take(n) {
            let theta = -(k as f64) * (2.0 * std::f64::consts::PI) * (t as f64) / (n as f64);
            let c = Complex::new(theta.cos(), theta.sin());
            sum += item.into() * c;
        }
        dft.push(sum);
    }
    dft
}

/// Real discrete Fourier transform 
pub fn rdft<T: Into<f64> + Copy>(signal: &[T]) -> Vec<Complex<f64>> {
    dft(signal)[..(signal.len() / 2) + 1].to_vec()
}

/// Inverse discrete Fourier transform 
pub fn idft<T: Into<f64> + Copy>(signal: &[Complex<T>]) -> Vec<f64> {
    let n = signal.len();
    let mut idft: Vec<f64> = Vec::with_capacity(n);
    for k in 0..n {
        let mut sum = 0.0;
        //for t in 0..n {
        //    let theta = (k as f64) * (2.0 * std::f64::consts::PI) * (t as f64) / (n as f64);
        //    let c = Complex::new(theta.cos(), theta.sin());
        //    sum += signal[t].re * c.re - signal[t].im * c.im;
        //}
        for (t, &item) in signal.iter().enumerate().take(n) {
            let theta = (k as f64) * (2.0 * std::f64::consts::PI) * (t as f64) / (n as f64);
            let c = Complex::new(theta.cos(), theta.sin());
            sum += item.re.into() * c.re - item.im.into() * c.im;
        }
        idft.push(sum / (n as f64));
    }
    idft
}

/// Inverse real discrete Fourier transform 
pub fn irdft<T: Into<f64> + Copy>(rfourier: &[Complex<T>]) -> Vec<f64> {
    let n = rfourier.len();
    let mut fourier =
        rfourier.iter().map(|&x| Complex::new(x.re.into(), x.im.into())).collect::<Vec<_>>(); //rfourier.to_vec();
    let mut conjugate: Vec<Complex<f64>> =
        rfourier[1..].iter().map(|&x| Complex::new(x.re.into(), x.im.into())).collect::<Vec<_>>(); //.to_vec();
    if rfourier[n - 1].im.into().abs() < 1e-10 {
        conjugate = rfourier[1..n - 1]
            .iter()
            .map(|&x| Complex::new(x.re.into(), x.im.into()))
            .collect::<Vec<_>>(); //.to_vec();
                                  //fourier.pop();
    }
    fourier.extend(conjugate.iter().map(|x| x.conj()).rev().collect::<Vec<_>>());
    idft(&fourier)
}

/// Same as fftfreq from Python Numpy.
pub fn dftfreq<T: Into<f64> + Copy>(n: usize, dt: T) -> Vec<f64> {
    let val = 1.0 / (n as f64 * dt.into());
    let mut result: Vec<f64> = vec![];
    let mid = (n - 1) / 2 + 1;
    result.append(&mut (0..mid).map(|x| val * x as f64).collect::<Vec<f64>>());
    result.append(&mut (-(n as isize / 2)..0).map(|x| val * x as f64).collect::<Vec<f64>>());
    result
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
    fn test_dft1() {
        let signal = vec![1, 2, 1, -1, 5];
        let dft = dft(&signal);
        let expected: Vec<Complex<f64>> = vec![
            Complex::new(8., 0.0),
            Complex::new(3.16311896, 1.67759904),
            Complex::new(-4.66311896, 3.66546879),
            Complex::new(-4.66311896, -3.66546879),
            Complex::new(3.16311896, -1.67759904),
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

    #[test]
    fn test_rdft() {
        let signal = vec![1, 2, 1, -1, 5];
        let dft = dft(&signal);
        let expected: Vec<Complex<f64>> = vec![
            Complex::new(8., 0.0),
            Complex::new(3.16311896, 1.67759904),
            Complex::new(-4.66311896, 3.66546879),
        ];
        assert_eq!(dft.len(), signal.len());
        for i in 0..expected.len() {
            assert_relative_eq!(dft[i].re, expected[i].re, epsilon = 1e-6);
            assert_relative_eq!(dft[i].im, expected[i].im, epsilon = 1e-6);
        }
    }

    #[test]
    fn test_irdft() {
        let signal = vec![1.0, 2.0, 1.0, -1.0, 1.5, 5.];
        let rdft_signal = rdft(&signal);
        let res = irdft(&rdft_signal);
        for i in 0..signal.len() {
            assert_relative_eq!(res[i], signal[i], epsilon = 1e-6);
        }
        let signal = vec![1.0, 2.0, 1.0, -1.0, 1.5];
        let rdft_signal = rdft(&signal);
        let res = irdft(&rdft_signal);
        for i in 0..signal.len() {
            assert_relative_eq!(res[i], signal[i], epsilon = 1e-6);
        }
        let signal = vec![
            Complex::new(15., 0.),
            Complex::new(-2.5, 3.4409548),
            Complex::new(-2.5, 0.81229924),
        ];
        let res = irdft(&signal);
        let expected = [1., 2., 3., 4., 5.];
        for i in 0..res.len() {
            assert_relative_eq!(res[i], expected[i], epsilon = 1e-6);
        }
        let signal = vec![
            Complex::new(21., 0.),
            Complex::new(-3., 5.19615242),
            Complex::new(-3., 1.73205081),
            Complex::new(-3., 0.),
        ];
        let res = irdft(&signal);
        let expected = [1., 2., 3., 4., 5., 6.];
        for i in 0..res.len() {
            assert_relative_eq!(res[i], expected[i], epsilon = 1e-6);
        }
    }

    #[test]
    fn test_dftfreq() {
        let signal_len = 8;
        let dt = 0.1;
        let res = dftfreq(signal_len, dt);
        let expected = [0., 1.25, 2.5, 3.75, -5., -3.75, -2.5, -1.25];
        for i in 0..expected.len() {
            assert_relative_eq!(res[i], expected[i], epsilon = 1e-6);
        }
    }
}
