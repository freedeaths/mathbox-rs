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

pub fn ncc(x: &[f64], y: &[f64], lag_max: usize) -> Vec<(isize, f64)> {
    let i = x.len();
    let j = y.len();
    let mut lag_max = lag_max;
    if lag_max >= i.min(j) {
        lag_max = i.min(j) - 1;
    }
    let mut ncc_list: Vec<(isize, f64)> = vec![];
    let mut result: Vec<(isize, f64)> = vec![];
    let mut new_x = vec![];
    for _ in 0..(j - 1) {
        new_x.push(0.0);
    }
    new_x.append(&mut x.to_vec());
    for _ in 0..(j - 1) {
        new_x.push(0.0);
    }
    for k in 0..(i + j - 1) {
        ncc_list.push((k as isize + 1 - i as isize, _ncc(y, &new_x[k..(k + j)])));
    }
    for lag in i -lag_max -1.. i + lag_max {
        result.push(ncc_list[lag]);
    }
    result
}

fn _ncc(x: &[f64], y: &[f64]) -> f64 {
    let x_mean = mean(x);
    let y_mean = mean(y);
    let mut x_qsum = 0.0;
    let mut y_qsum = 0.0;
    let mut xy_qsum = 0.0;
    for i in 0..x.len() {
        x_qsum += (x[i] - x_mean).powi(2) + std::f64::EPSILON;
        y_qsum += (y[i] - y_mean).powi(2) + std::f64::EPSILON;
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
    fn test_private_ncc() {
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(_ncc(&x, &y), 1.0);
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y = vec![5.0, 4.0, 3.0, 2.0, 1.0];
        assert_eq!(_ncc(&x, &y), -1.0);
        let a = [1.,2.,3.,4.,0.,1.,2.,3.,4.,0.,1.,2.,3.,4.,0.,1.,2.,3.,4.,0.];
        let b = [1.,2.,3.,3.,0.,1.,2.,3.,4.,0.,1.,1.,4.,4.,0.,1.,2.,3.,4.,0.];
        assert_relative_eq!(_ncc(&a, &b), 0.964, epsilon = 1e-3);
    }

    #[test]
    fn test_ncc() {
        let a = [0.,1.,2.,3.,4.,0.,1.,2.,3.,4.,0.,1.,2.,3.,4.,0.,1.,2.,3.,4.];
        let b = [1.,2.,3.,3.,0.,1.,2.,3.,4.,0.,1.,1.,4.,4.,0.,1.,2.,3.,4.,0.];
        let result = ncc(&a, &b, 4);
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
        let a = [0.,1.,2.,3.,4.,0.,1.,2.,3.,4.,0.,1.,2.,3.,4.,0.,1.,2.,3.];
        let b = [1.,2.,3.,3.,0.,1.,2.,3.,4.,0.,1.,1.,4.,4.,0.,1.,2.,3.,4.,0.];
        let result = ncc(&a, &b, 20);
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
        //for i in 0..result.len() {
            //assert_eq!(result[i].0, expected[i].0);
            //assert_relative_eq!(result[i].1, expected[i].1, epsilon = 1e-3);
        //}
        assert_ne!(result[0].0, expected[0].0);
    }
}