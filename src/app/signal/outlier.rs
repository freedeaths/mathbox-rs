use crate::stats::estimator::{mean, std};

pub fn iqr_outlier(signal: &[f64], iqr_factor: f64) -> (Vec<usize>, Vec<usize>) {
    let mut outlier_lo = vec![];
    let mut outlier_hi = vec![];
    let mut sorted = signal.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let q1 = sorted[(sorted.len() as f64 * 0.25) as usize];
    let q3 = sorted[(sorted.len() as f64 * 0.75) as usize];
    let iqr = q3 - q1;
    for (i, item) in signal.iter().enumerate() {
        if *item < q1 - iqr_factor * iqr {
            outlier_lo.push(i);
        }
        if *item > q3 + iqr_factor * iqr {
            outlier_hi.push(i);
        }
    }
    (outlier_lo, outlier_hi)
}

pub fn normal_outlier(normal: &[f64], std_factor: f64) -> (Vec<usize>, Vec<usize>) {
    let mut outlier_lo = vec![];
    let mut outlier_hi = vec![];
    let mean = mean(normal);
    let std = std(normal, false);
    for (i, item) in normal.iter().enumerate() {
        if *item > mean + std_factor * std {
            outlier_hi.push(i);
        } else if *item < mean - std_factor * std {
            outlier_lo.push(i);
        }
    }
    (outlier_lo, outlier_hi)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iqr_outlier() {
        let signal = vec![
            1.0, 20000.0, 20000.0, 20000.0, 20000.0, 20000.0, 20000.0, 20000.0, 20000.0, 20000.0,
            20000.0, 1000000.,
        ];
        let (lower, upper) = iqr_outlier(&signal, 1.5);
        assert_eq!(lower, [0]);
        assert_eq!(upper, [11]);
    }

    #[test]
    fn test_normal_outlier() {
        let signal = vec![
            -100000000000.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            100000000000.,
        ];
        let (lower, upper) = normal_outlier(&signal, 3.);
        assert_eq!(lower, [0]);
        assert_eq!(upper, [21]);
    }
}
