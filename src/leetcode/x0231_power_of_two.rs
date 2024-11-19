#[allow(dead_code)]
pub fn is_power_of_two(n: i32) -> bool {
    if n.is_negative() {
        return false;
    }
    (n as u32).is_power_of_two()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn is_power_of_two_1() {
        assert_eq!(is_power_of_two(1), true)
    }

    #[test]
    #[ignore]
    fn is_power_of_two_2() {
        assert_eq!(is_power_of_two(16), true)
    }

    #[test]
    #[ignore]
    fn is_power_of_two_3() {
        assert_eq!(is_power_of_two(3), false)
    }

    #[test]
    #[ignore]
    fn is_power_of_two_4() {
        assert_eq!(is_power_of_two(1073741825), false)
    }
}
