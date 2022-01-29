/*
pub struct Decomposor {
    pub n: usize,
    pub dt: f64,
    pub freq: Vec<f64>,
    pub dft: Vec<Complex<f64>>,
    pub rdft: Vec<Complex<f64>>,
}

impl Default for Decomposor {
    fn default() -> Self {
        Decomposor {
            n: 0,
            dt: 0.0,
            freq: vec![],
            dft: vec![],
            rdft: vec![],
        }
    }
}

pub fn decompose(signal: &[f64], dt: f64, config: Decomposor) -> (Vec<f64>, Vec<f64>) {
    let n = signal.len();
    let val = 1.0 / (n as f64 * dt);
    let mut result: Vec<f64> = vec![];
    let mid = (n - 1) / 2 + 1;
    result.append(&mut (0..mid).map(|x| val * x as f64).collect::<Vec<f64>>());
    result.append(&mut (-(n as isize / 2)..0).map(|x| val * x as f64).collect::<Vec<f64>>());
    let mut dft_signal: Vec<Complex<f64>> = vec![];
    for i in 0..signal.len() {
        dft_signal.push(Complex::new(signal[i], 0.0));
    }
    let dft_signal = dft(&dft_signal);
    let mut dft_result: Vec<f64> = vec![];
    for i in 0..dft_signal.len() {
        dft_result.push(dft_signal[i].re);
    }
    (result, dft_result)
}
*/