pub fn climb_stairs(n: i32) -> i32 {
    if n < 2 {
        return n;
    }
    let mut times = vec![1, 2];

    for _i in 2..n {
        let new_times = times[1] + times[0];
        times[0] = times[1];
        times[1] = new_times;
    }

    return times[1];
}

#[cfg(test)]
mod test_climb_stairs_mod {
    use super::climb_stairs;

    #[test]
    fn test_climb_stairs() {
        assert_eq!(climb_stairs(1), 1);
        assert_eq!(climb_stairs(2), 2);
        assert_eq!(climb_stairs(3), 3);
        assert_eq!(climb_stairs(4), 5);
    }
}
