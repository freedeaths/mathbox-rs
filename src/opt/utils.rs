use find_peaks::PeakFinder;

pub fn local_max(signal: &[f64], prominence: f64) -> Vec<usize> {
    let mut fp = PeakFinder::new(signal);
    fp.with_min_prominence(prominence);
    let peaks = fp.find_peaks();
    peaks.iter().map(|x| x.middle_position()).collect::<Vec<usize>>()
}

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

pub fn argsort<T: PartialOrd>(data: &[T], reverse: bool) -> Vec<usize> {
    let mut indices = (0..data.len()).collect::<Vec<_>>();
    //indices.sort_by_key(|&i| &data[i]);  // not work for float
    indices.sort_by(|&i, &j| data[i].partial_cmp(&data[j]).unwrap());
    if reverse {
        indices.reverse();
    }
    indices
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_local_max() {
        let a = vec![1.0, 3.0, 2.0, 4.0, 1.0, 3.0, 2.0, 4.0, 1.0];
        let b = local_max(&a, 0.);
        let expected = vec![1, 3, 5, 7];
        assert_eq!(b, expected);
    }
    #[test]
    fn test_argmax() {
        let a = vec![1, 2, 3, 4];
        assert_eq!(argmax(&a), 3);
        let a = vec![1.0, 2.0, 3.0, 0.4];
        assert_eq!(argmax(&a), 2);
    }
    #[test]
    fn test_argsort() {
        let data = vec![-5, 4, 1, -3, 2];
        let expected = vec![0, 3, 2, 4, 1];
        assert_eq!(argsort(&data, false), expected);
        let data = vec![-5., 4., 1., -3., 2.];
        let expected = vec![0, 3, 2, 4, 1];
        let ex_reverse = vec![1, 4, 2, 3, 0];
        assert_eq!(argsort(&data, false), expected);
        assert_eq!(argsort(&data, true), ex_reverse);
    }
}
