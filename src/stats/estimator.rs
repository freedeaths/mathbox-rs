pub fn mean(series: &[f64]) -> f64 {
    if series.len() == 0 {
        panic!("Cannot calculate mean of empty series");
    }
    series.iter().sum::<f64>() / series.len() as f64
}

pub fn var(series: &[f64], biased: bool) -> f64 {
    let mean = mean(series);
    let mut sum = 0.0;
    for value in series {
        sum += (value - mean).powi(2);
    }
    if biased {
        sum / series.len() as f64
    } else {
        if series.len() == 1 {
            panic!("Cannot calculate unbiased var of 1 number");
        }
        sum / (series.len() - 1) as f64
    }
}

pub fn std(series: &[f64], biased: bool) -> f64 {
        var(series, biased).sqrt()
}

pub fn median(series: &[f64]) -> f64 {
    let mut series = series.to_vec();
    series.sort_by(|a, b| a.partial_cmp(b).unwrap());
    if series.len() % 2 == 0 {
        (series[series.len() / 2 - 1] + series[series.len() / 2]) / 2.0
    } else {
        series[series.len() / 2]
    }
}

pub fn pcc(x: &[f64], y: &[f64], lag_max: usize) -> Vec<(isize, f64)> {
    let i = x.len();
    if i != y.len() {
        panic!("Cannot calculate pearson correlation coefficient with different length series");
    }
    if x.iter().copied().fold(f64::NAN, f64::max) == x.iter().copied().fold(f64::NAN, f64::min) || y.iter().copied().fold(f64::NAN, f64::max) == y.iter().copied().fold(f64::NAN, f64::min) {
        panic!("Cannot calculate pearson correlation coefficient with constant series");
    }
    let mut lag_max = lag_max;
    if lag_max >= i {
        lag_max = i - 1;
    }
    let mut result: Vec<(isize, f64)> = vec![];
    let mut new_x = vec![];
    for _ in 0..(i - 1) {
        new_x.push(0.0);
    }
    new_x.append(&mut x.to_vec());
    for _ in 0..(i - 1) {
        new_x.push(0.0);
    }
    for k in i -lag_max -1.. i + lag_max {
        let x_lag = new_x[k..(k + i)].to_vec();
        if x_lag.iter().copied().fold(f64::NAN, f64::max) == x_lag.iter().copied().fold(f64::NAN, f64::min) {
            continue;
        }
        result.push((k as isize + 1 - i as isize, pearson_correlation_coefficient(y, &new_x[k..(k + i)])));
    }
    result
}

pub fn pearson_correlation_coefficient(x: &[f64], y: &[f64]) -> f64 {
    if x.len() != y.len() {
        panic!("Cannot calculate pearson correlation coefficient with different length series");
    }
    if x.iter().copied().fold(f64::NAN, f64::max) == x.iter().copied().fold(f64::NAN, f64::min) || y.iter().copied().fold(f64::NAN, f64::max) == y.iter().copied().fold(f64::NAN, f64::min) {
        panic!("Cannot calculate pearson correlation coefficient with constant series");
    }
    let x_mean = mean(x);
    let y_mean = mean(y);
    let mut x_qsum = 0.0;
    let mut y_qsum = 0.0;
    let mut xy_qsum = 0.0;
    for i in 0..x.len() {
        x_qsum += (x[i] - x_mean).powi(2);
        y_qsum += (y[i] - y_mean).powi(2);
        xy_qsum += (x[i] - x_mean) * (y[i] - y_mean);
    }
    xy_qsum / (x_qsum * y_qsum).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_mean() {
        let series = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(mean(&series), 3.0);
    }

    #[test]
    #[should_panic]
    fn test_mean_panic() {
        let series: Vec<f64> = vec![];
        mean(&series);
    }

    #[test]
    fn test_var() {
        let series = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(var(&series, false), 2.5);
        assert_eq!(var(&series, true), 2.0);
    }

    #[test]
    #[should_panic]
    fn test_var_panic() {
        let series: Vec<f64> = vec![1.];
        assert_eq!(var(&series, false), 2.5);
    }

    #[test]
    fn test_std() {
        let series = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_relative_eq!(std(&series, false), 1.58113883008, epsilon = 1e-6);
        assert_eq!(std(&series, true), 1.4142135623730951);
    }

    #[test]
    fn test_median() {
        let series = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(median(&series), 3.0);
    }

    #[test]
    fn test_pearson_correlation_coefficient() {
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(pearson_correlation_coefficient(&x, &y), 1.0);
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y = vec![5.0, 4.0, 3.0, 2.0, 1.0];
        assert_eq!(pearson_correlation_coefficient(&x, &y), -1.0);
        let a = [1.,2.,3.,4.,0.,1.,2.,3.,4.,0.,1.,2.,3.,4.,0.,1.,2.,3.,4.,0.];
        let b = [1.,2.,3.,3.,0.,1.,2.,3.,4.,0.,1.,1.,4.,4.,0.,1.,2.,3.,4.,0.];
        assert_relative_eq!(pearson_correlation_coefficient(&a, &b), 0.964, epsilon = 1e-3);
    }

    #[test]
    fn test_pcc() {
        let a = [0.,1.,2.,3.,4.,0.,1.,2.,3.,4.,0.,1.,2.,3.,4.,0.,1.,2.,3.,4.];
        let b = [1.,2.,3.,3.,0.,1.,2.,3.,4.,0.,1.,1.,4.,4.,0.,1.,2.,3.,4.,0.];
        let result = pcc(&a, &b, 4);
        let expected = [
        (-4, 0.75709),
        (-3, 0.013114),
        (-2, -0.499392),
        (-1, -0.3793792),
        (0, 0.),
        (1, 0.96362),
        (2, 0.11803),
        (3, -0.484421),
        (4, -0.411908)];
        for i in 0..result.len() {
            assert_eq!(result[i].0, expected[i].0);
            assert_relative_eq!(result[i].1, expected[i].1, epsilon = 1e-3);
        }
    }
}