/// https://leetcode.com/problems/pascals-triangle
#[allow(dead_code)]
pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![vec![1]];
    if num_rows < 1 {
        return result;
    }
    for i in 1..num_rows as usize {
        if i == 1 {
            result.push(vec![1, 1]);
        } else {
            let last = result.last().unwrap();
            let mut new: Vec<i32> = vec![1];
            for i in 1..last.len() {
                let prev = last.get(i - 1).unwrap();
                let current = last.get(i).unwrap();
                let sum = current + prev;
                new.push(sum);
            }
            new.push(1);
            result.push(new);
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn generate_1() {
        assert_eq!(
            generate(5),
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1]
            ]
        )
    }

    #[test]
    #[ignore]
    fn generate_2() {
        assert_eq!(generate(1), vec![vec![1]])
    }
}
