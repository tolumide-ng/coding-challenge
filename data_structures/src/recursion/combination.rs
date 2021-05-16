pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    let mut list: Vec<Vec<i32>> = vec![];

    combine_dfs(n, k, 1, &mut list, &mut vec![]);

    return list;
}

pub fn combine_dfs(n: i32, k: i32, start: i32, list: &mut Vec<Vec<i32>>, current: &mut Vec<i32>) {
    // time 0(2^n)
    // space 0(n)

    if current.len() == k as usize {
        list.push((&current).to_vec());

        return;
    }

    for index in start..=n {
        current.push(index);
        combine_dfs(n, k, index + 1, list, current);
        current.pop();
    }
}

pub fn combine_recursion(n: i32, k: i32) -> Vec<Vec<i32>> {
    let mut list = vec![];

    // Base case 1: where k = 0 -> vec![vec![]]
    if k == 0 {
        list.push(vec![]);
        return list;
    }

    // Base case 2: were k = 1 -> vec![vec![1, 2, ..., n]]
    if k == 1 {
        let mut current: Vec<i32> = vec![];
        for index in 1..n {
            current.push(index);
        }
        list.push(current);
        return list;
    }

    // for index in

    return list;
}

#[cfg(test)]
mod test_combination_mod {
    use crate::recursion::combination::combine;

    #[test]
    fn test_combination() {
        assert_eq!(combine(4, 2).len(), 6);
        assert_eq!(combine(6, 3).len(), 20);
    }
}
