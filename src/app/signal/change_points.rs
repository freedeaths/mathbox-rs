use crate::opt::utils::argmax;
use crate::stats::estimator::energy_distance;
use crate::stats::hypothesis_testing::permutation_test;

fn calculate_t_stats<X: Into<f64> + Copy, Y: Into<f64> + Copy>(
    signal: &[X],
    jump: usize,
    p_value: Y,
    permutations: usize,
) -> Option<(usize, f64)> {
    if signal.len() < jump {
        return None;
    }
    let mut t_values = vec![];
    let t_stats = |x: &[f64], y: &[f64]| {
        energy_distance(x, y, true) * x.len() as f64 * y.len() as f64
            / (x.len() as f64 + y.len() as f64)
    };
    for i in 0..(signal.len() - 1) {
        let a = signal[..(i + 1)].iter().map(|&x| x.into()).collect::<Vec<f64>>(); //.to_vec();
        let b = signal[(i + 1)..].iter().map(|&x| x.into()).collect::<Vec<f64>>(); //.to_vec();
        let t = t_stats(&a, &b);
        t_values.push(t);
    }
    let idx = argmax(&t_values);
    let a = signal[..(idx + 1)].iter().map(|&x| x.into()).collect::<Vec<f64>>(); //.to_vec();
    let b = signal[(idx + 1)..].iter().map(|&x| x.into()).collect::<Vec<f64>>(); //.to_vec();
    if permutation_test(&a, &b, &t_stats, p_value.into(), permutations) {
        Some((idx, t_values[idx]))
    } else {
        None
    }
}

/// Detect change points from a &[<Into<f64> + Copy>] vector.
///
/// References: [Energy distance](https://doi.org/10.1002/wics.1375) & [MongoDB paper](https://dl.acm.org/doi/10.1145/3358960.3375791)
///
/// # Examples
///
/// ```
/// use mathbox::app::signal::change_points::e_divisive;
/// let data_1 = [0., 0., 0., 0., 0., 0., 1., 1., 1., 1., 1., 5., 5., 5., 5., 5.];
/// let mut cp = e_divisive(&data_1, 3, 0.05, 100);
/// cp.sort();
/// assert_eq!(cp, vec![5, 10]);
/// ```
//#[no_mangle]
pub fn e_divisive<X: Into<f64> + Copy, Y: Into<f64> + Copy>(
    signal: &[X],
    jump: usize,
    p_value: Y,
    permutations: usize,
) -> Vec<usize> {
    let mut change_points = vec![];
    let mut window = vec![0, signal.len() - 1];
    while window.len() > 1 {
        match calculate_t_stats(
            &signal[window[window.len() - 2]..window[window.len() - 1]],
            jump,
            p_value,
            permutations,
        ) {
            Some((idx, _)) => {
                if change_points.contains(&(window[window.len() - 2] + idx))
                    || window[window.len() - 2] + idx == window[0]
                    || window[window.len() - 2] + idx == window[window.len() - 1]
                {
                    window.pop();
                    continue;
                } else {
                    change_points.push(window[window.len() - 2] + idx);
                    window.pop();
                    window.push(window[window.len() - 1] + idx);
                    window.push(signal.len() - 1)
                }
            }
            None => {
                window.pop();
            }
        }
    }
    change_points
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_t_stats() {
        let data_1 = [0., 0., 0., 0., 0., 0., 1., 1., 1., 1., 1., 5., 5., 5., 5., 5.];
        let (idx, _) = calculate_t_stats(&data_1, 3, 0.05, 100).unwrap();
        assert_eq!(idx, 10); // maybe 5
        let data_1 = [0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 5, 5, 5, 5, 5];
        let (idx, _) = calculate_t_stats(&data_1, 3, 0.05, 100).unwrap();
        assert_eq!(idx, 10); // maybe 5
    }

    #[test]
    fn test_e_divisive() {
        let data_1 = [0., 0., 0., 0., 0., 0., 1., 1., 1., 1., 1., 5., 5., 5., 5., 5.];
        let _cp = e_divisive(&data_1, 3, 0.05, 100);
        let mut cp = _cp.clone();
        cp.sort();
        assert_eq!(cp, vec![5, 10]);
        let data_1 = [0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 5, 5, 5, 5, 5];
        let _cp = e_divisive(&data_1, 3, 0.05, 100);
        let mut cp = _cp.clone();
        cp.sort();
        assert_eq!(cp, vec![5, 10]);
    }
}
