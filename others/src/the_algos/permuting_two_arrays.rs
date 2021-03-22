#[derive(Debug, PartialEq)]
pub enum Answer {
    YES,
    NO,
}

pub fn two_arrays(k: i32, a: &mut Vec<i32>, b: &mut Vec<i32>) -> Answer {
    let mut possible = Answer::YES;

    a.sort();

    b.sort();
    b.reverse();

    for index in 0..a.len() {
        println!("RESULT {}", a[index] + b[index]);
        if a[index] + b[index] < k {
            possible = Answer::NO;
        }
    }

    return possible;
}

#[cfg(test)]
mod test_two_arrays_cont {
    use super::*;

    #[test]
    fn test_two_arrays() {
        let mut a = vec![2, 1, 3];
        let mut b = vec![7, 8, 9];

        assert_eq!(two_arrays(10, &mut a, &mut b), Answer::YES);

        let mut a = vec![1, 2, 2, 1];
        let mut b = vec![3, 3, 3, 4];

        assert_eq!(two_arrays(5, &mut a, &mut b), Answer::NO);
    }
}
