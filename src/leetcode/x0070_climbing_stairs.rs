#[allow(dead_code)]
pub fn climb_stairs(n: i32) -> i32 {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn climb_stairs_2() {
        assert_eq!(climb_stairs(2), 2)
    }

    #[test]
    fn climb_stairs_3() {
        // 1 + 1 + 1
        // 1 + 2
        // 2 + 1
        assert_eq!(climb_stairs(3), 3)
    }

    #[test]
    fn climb_stairs_4() {
        // 4 / 4 = 1 0
        // 4 / 3 = 1 1
        // 4 / 2 = 2 0
        // 4 / 1 = 4 0

        // 1 + 1 + 1 + 1
        // 1 + 1 + 2
        // 1 + 2 + 1
        // 2 + 1 + 1
        // 2 + 2
        assert_eq!(climb_stairs(4), 5)
    }

    #[test]
    fn climb_stairs_5() {
        // 5 / 5 = 1 0
        // 5 / 4 = 1 1
        // 5 / 3 = 1 2
        // 5 / 2 = 2 1
        // 5 / 1 = 5 0

        // 1 + 1 + 1 + 1 + 1
        // 1 + 1 + 1 + 2
        // 1 + 1 + 2 + 1
        // 1 + 2 + 1 + 1
        // 2 + 1 + 1 + 1
        // 2 + 1 + 2
        // 2 + 2 + 1
        assert_eq!(climb_stairs(5), 7)
    }

    #[test]
    fn climb_stairs_6() {
        // 6 / 6 = 1 0
        // 6 / 5 = 1 1
        // 6 / 4 = 1 2
        // 6 / 3 = 2 0
        // 6 / 2 = 3 0
        // 6 / 1 = 6 0

        // 1 + 1 + 1 + 1 + 1 + 1
        // 1 + 1 + 1 + 1 + 2
        // 1 + 1 + 1 + 2 + 1
        // 1 + 1 + 2 + 1 + 1
        // 1 + 2 + 1 + 1 + 1
        // 2 + 1 + 1 + 1 + 1
        // 2 + 1 + 1 + 2
        // 2 + 1 + 2 + 1
        // 2 + 2 + 1 + 1
        // 2 + 2 + 2
        assert_eq!(climb_stairs(6), 10)
    }
}
