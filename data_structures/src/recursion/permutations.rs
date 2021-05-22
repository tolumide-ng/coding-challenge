pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    if nums.len() <= 1 {
        return vec![nums];
    }

    let mut store: Vec<Vec<i32>> = vec![];

    for index in 0..nums.len() {
        let mut new_nums = nums.clone();

        let current = new_nums.remove(index);

        for possibles in &mut permute(new_nums) {
            possibles.insert(0, current);

            store.push(possibles.to_vec());
        }
    }

    return store;
}

#[cfg(test)]
mod test_permute_mod {
    use crate::recursion::permutations::permute;

    #[test]
    fn test_permute() {
        let nums = vec![1, 2, 3];

        let result = permute(nums);

        assert_eq!(result.len(), 6);
    }

    #[test]
    fn test_emoty_permute() {
        let nums = vec![1];
        assert_eq!(permute(nums).len(), 1);
    }
}
