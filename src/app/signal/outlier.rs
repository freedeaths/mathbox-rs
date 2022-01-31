use crate::stats::estimator::{mean, std};

pub fn iqr_outlier<X: Into<f64> + Copy, Y: Into<f64> + Copy>(
    signal: &[X],
    iqr_factor: Y,
) -> (Vec<usize>, Vec<usize>) {
    let mut outlier_lo = vec![];
    let mut outlier_hi = vec![];
    let mut sorted = signal.iter().map(|&x| x.into()).collect::<Vec<f64>>(); //signal.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let q1 = sorted[(sorted.len() as f64 * 0.25) as usize];
    let q3 = sorted[(sorted.len() as f64 * 0.75) as usize];
    let iqr = q3 - q1;
    for (i, &item) in signal.iter().enumerate() {
        if item.into() < q1 - iqr_factor.into() * iqr {
            outlier_lo.push(i);
        }
        if item.into() > q3 + iqr_factor.into() * iqr {
            outlier_hi.push(i);
        }
    }
    (outlier_lo, outlier_hi)
}

pub fn normal_outlier<X: Into<f64> + Copy, Y: Into<f64> + Copy>(
    normal: &[X],
    std_factor: Y,
) -> (Vec<usize>, Vec<usize>) {
    let mut outlier_lo = vec![];
    let mut outlier_hi = vec![];
    let mean = mean(normal);
    let std = std(normal, false);
    for (i, &item) in normal.iter().enumerate() {
        if item.into() > mean + std_factor.into() * std {
            outlier_hi.push(i);
        } else if item.into() < mean - std_factor.into() * std {
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
        let signal =
            vec![1, 20000, 20000, 20000, 20000, 20000, 20000, 20000, 20000, 20000, 20000, 1000000];
        let (lower, upper) = iqr_outlier(&signal, 3);
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
        let signal: Vec<i32> =
            vec![-100000000, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 100000000];
        let (lower, upper) = normal_outlier(&signal, 3);
        assert_eq!(lower, [0]);
        assert_eq!(upper, [21]);
    }
}
