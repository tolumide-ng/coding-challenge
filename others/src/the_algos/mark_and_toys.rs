pub fn max_toys(prices: &mut Vec<u32>, k: u32) -> usize {
    let mut total = 0;
    let mut pocket = k;
    prices.sort();

    for index in 0..prices.len() {
        if pocket >= prices[index] {
            total += 1;
            pocket -= prices[index]
        }
    }

    return total;
}

#[cfg(test)]
mod test_max_toys_cont {
    use super::*;

    #[test]
    pub fn test_max_toys() {
        let mut toys = vec![1, 12, 5, 111, 200, 1000, 10];

        assert_eq!(max_toys(&mut toys, 50), 4);
    }
}
