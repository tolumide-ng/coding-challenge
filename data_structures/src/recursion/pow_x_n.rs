pub fn my_pow(x: f64, n: i32) -> f64 {
    if n == 0 {
        return 1_f64;
    }

    if n == 1 {
        return x as f64;
    }

    if n < 0 {
        return my_pow((1_f64 / x) as f64, -n);
    }

    let mut result = my_pow(x * x, n / 2);
    if n % 2 == 1 {
        result *= x;
    }

    return result;
}

#[cfg(test)]
mod test_pow_x_n_mod {
    use crate::recursion::pow_x_n::my_pow;

    #[test]
    fn test_pow_x_n() {
        assert_eq!(my_pow(2_f64, 2), 4_f64);
        assert_eq!(my_pow(2_f64, -2), 0.25_f64);
        assert_eq!(my_pow(2_f64, 0), 1_f64);
        // assert_eq!(my_pow(0.00001_f64, 2147483647), 2147483647_f64);
    }
}
