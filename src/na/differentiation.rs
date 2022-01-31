pub fn diff<T: Into<f64> + Copy>(signal: &[T]) -> Vec<f64> {
    let mut result = vec![0.0; signal.len() - 1];
    for i in 0..signal.len() - 1 {
        result[i] = signal[i + 1].into() - signal[i].into();
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_diff() {
        let signal = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let result = diff(&signal);
        let expected = vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
        for i in 0..signal.len() - 1 {
            assert_relative_eq!(result[i], expected[i]);
        }
        let signal = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let result = diff(&signal);
        let expected = vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
        for i in 0..signal.len() - 1 {
            assert_relative_eq!(result[i], expected[i]);
        }
    }
}
