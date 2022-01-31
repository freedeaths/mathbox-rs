pub enum Exterpolation {
    Linear,
    Saturation,
}

fn point_linear_interp<T: Into<f64> + Copy, S: Into<f64> + Copy>(
    x: T,
    x0: T,
    y0: S,
    x1: T,
    y1: S,
) -> f64 {
    y0.into() + (x.into() - x0.into()) * (y1.into() - y0.into()) / (x1.into() - x0.into())
}

pub fn series_linear_interp<X: Into<f64> + Copy, Y: Into<f64> + Copy, Z: Into<f64> + Copy>(
    orig_time: &[X],
    orig_signal: &[Y],
    desired_time: &[Z],
    exterp: Exterpolation,
) -> Vec<f64> {
    let mut result: Vec<f64> = vec![];
    for &item in desired_time {
        if item.into() < orig_time[0].into() {
            match exterp {
                Exterpolation::Linear => result.push(point_linear_interp(
                    item.into(),
                    orig_time[0].into(),
                    orig_signal[0].into(),
                    orig_time[1].into(),
                    orig_signal[1].into(),
                )),
                Exterpolation::Saturation => result.push(orig_signal[0].into()),
            }
        }
        for i in 0..orig_time.len() - 1 {
            if item.into() >= orig_time[i].into() && item.into() <= orig_time[i + 1].into() {
                result.push(point_linear_interp(
                    item.into(),
                    orig_time[i].into(),
                    orig_signal[i].into(),
                    orig_time[i + 1].into(),
                    orig_signal[i + 1].into(),
                ));
                break;
            }
        }
        if item.into() > orig_time[orig_time.len() - 1].into() {
            match exterp {
                Exterpolation::Linear => result.push(point_linear_interp(
                    item.into(),
                    orig_time[orig_time.len() - 2].into(),
                    orig_signal[orig_signal.len() - 2].into(),
                    orig_time[orig_time.len() - 1].into(),
                    orig_signal[orig_signal.len() - 1].into(),
                )),
                Exterpolation::Saturation => result.push(orig_signal[orig_signal.len() - 1].into()),
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
        assert_relative_eq!(point_linear_interp(0, 0, 0, 1, 1), 0.0);
        assert_relative_eq!(point_linear_interp(0.5, 0.0, 0.0, 1.0, 1.0), 0.5);
        assert_relative_eq!(point_linear_interp(1, 0, 0, 1, 1), 1.0);
        assert_relative_eq!(point_linear_interp(0, 0, 1, 1, 0), 1.0);
        assert_relative_eq!(point_linear_interp(0.5, 0.0, 1.0, 1.0, 0.0), 0.5);
        assert_relative_eq!(point_linear_interp(1, 0, 1, 1, 0), 0.0);
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
        let orig_time = vec![1, 2, 3, 4, 5];
        let orig_signal = vec![1, 2, 3, 4, 5];
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
