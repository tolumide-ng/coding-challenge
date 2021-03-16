// https://www.hackerrank.com/challenges/angry-children/problem

pub fn max_min(arr: &mut Vec<u32>, target: usize) -> u32 {
    let mut result = u32::MAX;

    // for x in 0..arr.len() {}

    arr.sort();
    let mut i: usize = 0;

    while i + target - 1 < arr.len() {
        let diff = arr[i + target - 1] - arr[i];

        result = std::cmp::min(result as u32, diff);
        i += 1;
    }

    return result;
}

#[cfg(test)]
mod test_max_min_cont {
    use super::*;

    #[test]
    fn test_max_min() {
        let mut a = vec![10, 100, 300, 200, 1000, 20, 30];
        let mut b = vec![1, 2, 3, 4, 10, 20, 30, 40, 100, 200];
        let mut c = vec![1, 2, 1, 2, 1];
        let mut d = vec![
            4504, 1520, 5857, 4094, 4157, 3902, 822, 6643, 2422, 7288, 8245, 9948, 2822, 1784,
            7802, 3142, 9739, 5629, 5413, 7232,
        ];

        let all_test = vec![
            (&mut a, 3, 20),
            (&mut b, 4, 3),
            (&mut c, 2, 0),
            (&mut d, 5, 1335),
        ];
        // assert_eq!(eval_rpn(vec!))

        for test in all_test {
            assert_eq!(max_min(test.0, test.1), test.2);
        }
    }
}
