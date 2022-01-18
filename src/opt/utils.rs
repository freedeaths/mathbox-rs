/*
pub fn local_minmax(signal: &[f64], closed: bool) -> (Vec<usize>, Vec<usize>) {
    let mut minima = vec![];
    let mut maxima = vec![];
    let mut prev = signal[0];

    (minima, maxima)
} */

/// Find argmax of slice.
///
/// Returns index of first occuring maximum.
///
/// # Examples
///
/// ```
/// use mathbox::opt::utils::argmax;
/// let a = vec![1.0,2.0,3.0,4.0];
/// assert_eq!(argmax(&a), 3);
/// ```
pub fn argmax<T: Copy + PartialOrd>(x: &[T]) -> usize {
    assert!(!x.is_empty());

    let mut max_index = 0;
    let mut max = x[max_index];

    for (i, v) in (x.iter()).enumerate() {
        if max < *v {
            max_index = i;
            max = *v;
        }
    }
    max_index
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_argmax() {
        let a = vec![1, 2, 3, 4];
        assert_eq!(argmax(&a), 3);
        let a = vec![1.0, 2.0, 3.0, 0.4];
        assert_eq!(argmax(&a), 2);
    }
}
