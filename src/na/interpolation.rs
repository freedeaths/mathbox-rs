pub enum Exterpolation {
    Linear,
    Saturation,
}

fn point_linear_interp(x: f64, x0: f64, y0: f64, x1: f64, y1: f64) -> f64 {
    y0 + (x - x0) * (y1 - y0) / (x1 - x0)
}

pub fn series_linear_interp(
    orig_time: &[f64],
    orig_signal: &[f64],
    desired_time: &[f64],
    exterp: Exterpolation,
) -> Vec<f64> {
    let mut result: Vec<f64> = vec![];
    for item in desired_time {
        if item < &orig_time[0] {
            match exterp {
                Exterpolation::Linear => result.push(point_linear_interp(
                    *item,
                    orig_time[0],
                    orig_signal[0],
                    orig_time[1],
                    orig_signal[1],
                )),
                Exterpolation::Saturation => result.push(orig_signal[0]),
            }
        }
        for i in 0..orig_time.len() - 1 {
            if item >= &orig_time[i] && item <= &orig_time[i + 1] {
                result.push(point_linear_interp(
                    *item,
                    orig_time[i],
                    orig_signal[i],
                    orig_time[i + 1],
                    orig_signal[i + 1],
                ));
                break;
            }
        }
        if item > &orig_time[orig_time.len() - 1] {
            match exterp {
                Exterpolation::Linear => result.push(point_linear_interp(
                    *item,
                    orig_time[orig_time.len() - 2],
                    orig_signal[orig_signal.len() - 2],
                    orig_time[orig_time.len() - 1],
                    orig_signal[orig_signal.len() - 1],
                )),
                Exterpolation::Saturation => result.push(orig_signal[orig_signal.len() - 1]),
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_point_linear_interp() {
        assert_relative_eq!(point_linear_interp(0.0, 0.0, 0.0, 1.0, 1.0), 0.0);
        assert_relative_eq!(point_linear_interp(0.5, 0.0, 0.0, 1.0, 1.0), 0.5);
        assert_relative_eq!(point_linear_interp(1.0, 0.0, 0.0, 1.0, 1.0), 1.0);
        assert_relative_eq!(point_linear_interp(0.0, 0.0, 1.0, 1.0, 0.0), 1.0);
        assert_relative_eq!(point_linear_interp(0.5, 0.0, 1.0, 1.0, 0.0), 0.5);
        assert_relative_eq!(point_linear_interp(1.0, 0.0, 1.0, 1.0, 0.0), 0.0);
    }

    #[test]
    fn test_series_linear_interp() {
        let orig_time = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let orig_signal = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let desired_time = vec![0.0, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0, 4.5, 5.0, 6.0];
        let result =
            series_linear_interp(&orig_time, &orig_signal, &desired_time, Exterpolation::Linear);
        for i in 0..result.len() {
            assert_relative_eq!(result[i], desired_time[i]);
        }
        let result_sat = series_linear_interp(
            &orig_time,
            &orig_signal,
            &desired_time,
            Exterpolation::Saturation,
        );
        let expected = vec![1.0, 1.0, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0, 4.5, 5.0, 5.0];
        for i in 0..result_sat.len() {
            assert_relative_eq!(result_sat[i], expected[i]);
        }
    }
}
