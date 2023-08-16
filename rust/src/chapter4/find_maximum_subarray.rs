use num_traits::bounds::Bounded;
use num_traits::Zero;
use std::cmp::PartialOrd;
use std::fmt::Debug;
use std::ops::AddAssign;

fn find_max_crossing_subarray<T>(
    array: &[T],
    low: usize,
    mid: usize,
    high: usize,
) -> (usize, usize, T)
where
    T: Zero + Bounded + AddAssign + PartialOrd + Copy + Debug,
{
    let mut left_sum = T::min_value();
    let mut sum = T::zero();
    let mut max_left = 0usize;
    // Note: 一定要这样使用不能写成 mid..=low
    for i in (low..=mid).rev() {
        if let Some(v) = array.get(i) {
            sum += *v;
            if sum > left_sum {
                left_sum = sum;
                max_left = i;
            }
        }
    }
    let mut right_sum = T::min_value();
    let mut sum = T::zero();
    let mut max_right = 0usize;
    for i in (mid + 1)..=high {
        if let Some(v) = array.get(i) {
            sum += *v;
            if sum > right_sum {
                right_sum = sum;
                max_right = i;
            }
        }
    }
    (max_left, max_right, left_sum + right_sum)
}

/// find maximum sub array
pub fn find_maximum_subarray<T>(array: &[T], low: usize, hight: usize) -> (usize, usize, T)
where
    T: Zero + Bounded + AddAssign + PartialOrd + Default + Clone + Copy + Debug,
{
    if hight == low {
        let sum = array.get(low).copied().unwrap_or_default();
        (low, hight, sum)
    } else {
        let mid = ((low as f64 + hight as f64) / 2f64).floor() as usize;

        let (left_low, left_heigh, left_sum) = find_maximum_subarray(array, low, mid);
        let (right_low, right_high, right_sum) = find_maximum_subarray(array, mid + 1, hight);
        let (cross_low, cross_hight, cross_sum) =
            find_max_crossing_subarray(array, low, mid, hight);

        if left_sum >= right_sum && left_sum >= cross_sum {
            (left_low, left_heigh, left_sum)
        } else if right_sum >= left_sum && right_sum >= cross_sum {
            (right_low, right_high, right_sum)
        } else {
            (cross_low, cross_hight, cross_sum)
        }
    }
}

#[test]
fn test_find_maximum_subarray() {
    let array = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    let result = find_maximum_subarray(&array, 0, array.len() - 1);
    println!(
        "start is {}, end is {}, sum is {}",
        result.0, result.1, result.2
    );
}
