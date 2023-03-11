use num_traits::bounds::Bounded;
use num_traits::Zero;

fn find_max_crossing_subarray<T>(
    array: &[T],
    low: usize,
    mid: usize,
    high: usize,
) -> (usize, usize, T)
where
    T: Zero + Bounded + std::ops::AddAssign + std::cmp::PartialOrd + Copy + std::fmt::Debug,
{
    let mut left_sum = T::min_value();
    let mut sum = T::zero();
    let mut max_left = 0usize;
    // Note: 一定要这样使用不能写成 mid..=low
    for i in (low..=mid).rev() {
        if let Some(v) = array.get(i) {
            sum = sum + *v;
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
            sum = sum + *v;
            if sum > right_sum {
                right_sum = sum;
                max_right = i;
            }
        }
    }
    (max_left, max_right, left_sum + right_sum)
}

pub fn find_maximum_suarray<T>(array: &[T], low: usize, hight: usize) -> (usize, usize, T)
where
    T: Zero
        + Bounded
        + std::ops::AddAssign
        + std::cmp::PartialOrd
        + Default
        + Clone
        + std::cmp::PartialOrd
        + Copy
        + std::fmt::Debug,
{
    if hight == low {
        let sum = array.get(low).map(|v| v.clone()).unwrap_or_default();
        return (low, hight, sum);
    } else {
        let mid = ((low as f64 + hight as f64) / 2f64).floor() as usize;

        let (left_low, left_heigh, left_sum) = find_maximum_suarray(array, low, mid);
        let (right_low, right_high, right_sum) = find_maximum_suarray(array, mid + 1, hight);
        let (cross_low, cross_hight, cross_sum) =
            find_max_crossing_subarray(array, low, mid, hight);

        if left_sum >= right_sum && left_sum >= cross_sum {
            return (left_low, left_heigh, left_sum);
        } else if right_sum >= left_sum && right_sum >= cross_sum {
            return (right_low, right_high, right_sum);
        } else {
            return (cross_low, cross_hight, cross_sum);
        }
    }
}

#[test]
fn test_find_maximum_suarray() {
    let array = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    let result = find_maximum_suarray(&array, 0, array.len() - 1);
    println!(
        "start is {}, end is {}, sum is {}",
        result.0, result.1, result.2
    );
}

#[test]
fn test_array_range() {
    let array = vec![0, 1, 2];
    let low = 0;
    let high = array.len();
    for idx in low..high {
        println!("{}", array[idx]);
    }
    println!("*************");
    for idx in (high - 1)..=low {
        println!("{}", array[idx]);
    }
    println!("*******----******");
}
