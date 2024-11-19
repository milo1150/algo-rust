#[allow(dead_code)]
pub fn add_digits(num: i32) -> i32 {
    let num_str = num.to_string();
    let mut acc = sum(&num_str);
    while acc.to_string().len() >= 2 {
        acc = sum(&acc.to_string());
    }
    acc
}

fn sum(s: &String) -> i32 {
    return s
        .chars()
        .into_iter()
        .map(|chr| chr.to_string().parse::<i32>().unwrap())
        .sum();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn add_digits_1() {
        assert_eq!(add_digits(38), 2)
    }
    #[test]
    #[ignore]
    fn add_digits_2() {
        assert_eq!(add_digits(0), 0)
    }
}
