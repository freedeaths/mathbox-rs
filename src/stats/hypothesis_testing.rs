use rand::seq::SliceRandom;
use rand::thread_rng;


pub fn permutation_test(
    alist: &[f64],
    blist: &[f64],
    hypothesis: &dyn Fn(&[f64], &[f64]) -> f64,
    p_value: f64,
    n_permutations: usize,
) -> bool {
    let mut rng = thread_rng();

    let mut total = vec![];
    total.extend_from_slice(alist);
    total.extend_from_slice(blist);
    let len_a = alist.len();
    let t_stats = hypothesis(alist, blist);
    let mut better_num = 0;

    for _ in 0..n_permutations {
        total.shuffle(&mut rng);
        let new_a = &total[0..len_a];
        let new_b = &total[len_a..];
        let new_t_stats = hypothesis(new_a, new_b);
        if new_t_stats > t_stats {
            better_num += 1;
        }
    }
    let probability = better_num as f64 / (n_permutations + 1) as f64;
    println!("p: {}", probability);
    if probability <= p_value {
        return true;
    } else {
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stats::estimator::mean;
    //use approx::assert_relative_eq;

    #[test]
    fn test_permutation_test() {
        let a = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let b = vec![11.0, 12.0, 13.0, 14.0, 15.0, 11.0, 12.0, 13.0, 14.0, 15.0];
        let hypothesis = |a: &[f64], b: &[f64]| mean(a) - mean(b);
        let p_value = 0.05;
        let n_permutations = 1000;
        assert_eq!(
            permutation_test(&a, &b, &hypothesis, p_value, n_permutations),
            false
        );
        let a = vec![1.01, 1.02, 1.03, 1.04, 1.05];
        let b = vec![0.99, 1.0, 1.01, 1.0, 1.01];
        let hypothesis = |a: &[f64], b: &[f64]| mean(a) - mean(b);
        let p_value = 0.05;
        let n_permutations = 1000;
        assert_eq!(
            permutation_test(&a, &b, &hypothesis, p_value, n_permutations),
            true
        );
    }
}