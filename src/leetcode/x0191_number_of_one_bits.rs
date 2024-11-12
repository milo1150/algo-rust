/// https://leetcode.com/problems/number-of-1-bits/description/
#[allow(dead_code)]
pub fn hamming_weight(n: i32) -> i32 {
    let binary = format!("{:b}", n);
    let count = binary.chars().filter(|v| *v == '1').count();
    count.try_into().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn hamming_weight_1() {
        assert_eq!(hamming_weight(11), 3)
    }

    #[test]
    #[ignore]
    fn hamming_weight_2() {
        assert_eq!(hamming_weight(128), 1)
    }

    #[test]
    #[ignore]
    fn hamming_weight_3() {
        assert_eq!(hamming_weight(2147483645), 30)
    }
}
