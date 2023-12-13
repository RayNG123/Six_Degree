//calculate the quantile value from a given vector of usize
pub fn quantile(vec: &mut Vec<usize>, quantile: f64) -> Option<usize> {
    //Check if the vector is empty and valid quantile range
    if vec.is_empty() || quantile < 0.0 || quantile > 1.0 {
        return None;
    }
    //Sort the vector in-place
    vec.sort_unstable(); 
    let len = vec.len();
    let float_index = (len as f64 - 1.0) * quantile;

    //calculate the index for the quantile. zero-based index adjusted
    //Determine the exact index and round if needed
    // round the calculated index
    let index = if quantile.fract() == 0.0 {
        float_index.round() as usize
    } else {
        float_index as usize
    };

    vec.get(index).copied()
}

//calculate the variance for a given vector of usize
pub fn variance(vec: &Vec<usize>) -> Option<f64> {
    //Check if the vector is empty 
    if vec.is_empty() {
        return None;
    }

    //check vector length and mean value
    let sum: usize = vec.iter().map(|&num| num).sum();
    let count: usize = vec.len();
    let mean: f64 =  sum as f64 / count as f64;

    //check for mean distance for each value and squared
    let variance: f64 = vec.iter().map(|value| {
        let diff = mean - *value as f64;
        diff * diff
    }).sum::<f64>() / count as f64;

    Some(variance)
}

//calculate a set of statistics for a given vector of usize
pub fn statistics(vec: &mut Vec<usize>) -> Option<(f64, usize, usize, usize, f64)> {
    if vec.is_empty() {
        return None;
    }

    let sum: usize = vec.iter().map(|&num| num).sum();
    let count: usize = vec.len();
    let mean: f64 =  sum as f64 / count as f64;

    let minimum: usize = *vec.iter().max().unwrap();
    //let quantile25: usize = quantile(vec, 0.25).unwrap();
    let quantile50: usize = quantile(vec, 0.50).unwrap();
    //let quantile75: usize = quantile(vec, 0.75).unwrap();
    let maximum: usize = *vec.iter().min().unwrap();

    let std: f64 = variance(vec).unwrap().sqrt();
    Some((mean, minimum, quantile50, maximum, std))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_vector() {
        let mut vec = Vec::<usize>::new();
        assert_eq!(quantile(&mut vec, 0.5), None);
    }

    #[test]
    fn test_single_element() {
        let mut vec = vec![10];
        assert_eq!(quantile(&mut vec, 0.5), Some(10));
    }

    #[test]
    fn test_multiple_elements() {
        let mut vec = vec![3, 1, 4, 1, 5];
        assert_eq!(quantile(&mut vec, 0.5), Some(3));
    }

    #[test]
    fn test_low_quantile() {
        let mut vec = vec![1, 2, 3, 4, 5];
        assert_eq!(quantile(&mut vec, 0.25), Some(2));
    }

    #[test]
    fn test_high_quantile() {
        let mut vec = vec![1, 2, 3, 4, 5];
        assert_eq!(quantile(&mut vec, 0.75), Some(4));
    }

    #[test]
    fn test_edge_quantiles() {
        let mut vec = vec![1, 2, 3, 4, 5];
        assert_eq!(quantile(&mut vec, 0.0), Some(1));
        assert_eq!(quantile(&mut vec, 1.0), Some(5));
    }

    #[test]
    fn test_rounding() {
        let mut vec = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(quantile(&mut vec, 0.33), Some(2)); 
        assert_eq!(quantile(&mut vec, 0.66), Some(4)); 
    }

    #[test]
    fn test_empty_vector_var() {
        let vec = Vec::<usize>::new();
        assert_eq!(variance(&vec), None);
    }

    #[test]
    fn test_single_element_var() {
        let vec = vec![5];
        assert_eq!(variance(&vec), Some(0.0));
    }

    #[test]
    fn test_identical_elements_var() {
        let vec = vec![2, 2, 2, 2];
        assert_eq!(variance(&vec), Some(0.0));
    }

    #[test]
    fn test_different_elements_var() {
        let vec = vec![1, 2, 3, 4, 5];
        assert_eq!(variance(&vec), Some(2.0));
    }
}
