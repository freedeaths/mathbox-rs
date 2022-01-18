fn point_linear_interp(x: f64, x0: f64, x1: f64, y0: f64, y1: f64) -> f64 {
    y0 + (x - x0) * (y1 - y0) / (x1 - x0)
}

//pub fn series_linear_interp(orig_time: &[f64], orig_signal: &[f64], desired_time: &[f64], exterp: &str) -> Vec<f64> {
//
//}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_point_linear_interp() {
        assert_relative_eq!(point_linear_interp(0.0, 0.0, 1.0, 0.0, 1.0), 0.0);
        assert_relative_eq!(point_linear_interp(0.5, 0.0, 1.0, 0.0, 1.0), 0.5);
        assert_relative_eq!(point_linear_interp(1.0, 0.0, 1.0, 0.0, 1.0), 1.0);
        assert_relative_eq!(point_linear_interp(0.0, 0.0, 1.0, 1.0, 0.0), 1.0);
        assert_relative_eq!(point_linear_interp(0.5, 0.0, 1.0, 1.0, 0.0), 0.5);
        assert_relative_eq!(point_linear_interp(1.0, 0.0, 1.0, 1.0, 0.0), 0.0);
    }
}
