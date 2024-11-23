#[allow(dead_code)]
pub fn count_bits(n: i32) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];

    for n in 0..=n {
        let binary = format!("{:b}", n);
        let sum: Vec<char> = binary.chars().filter(|v| *v != '0').collect();
        result.push(sum.len() as i32);
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn count_bits_1() {
        assert_eq!(count_bits(2), vec![0, 1, 1])
    }

    #[test]
    #[ignore]
    fn count_bits_2() {
        assert_eq!(count_bits(5), vec![0, 1, 1, 2, 1, 2])
    }
}
