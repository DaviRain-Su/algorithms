/// 计算两个非负数p和q的最大公约数： 若q是0，则最大公约数为p。
/// 否则，将p除以q得到余数r，p和q的最大公约数即为q和r的最大公约数。
pub fn gcd(p: i32, q: i32) -> i32 {
    if q == 0 {
        return p;
    }
    let r = p % q;
    gcd(q, r)
}
