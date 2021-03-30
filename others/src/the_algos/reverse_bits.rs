// Reverse Bits
// Given an input integer input, return the integer value of input's bits reversed. You will only be reversing the "significant portion" of the binary representation (ignoring leading zeros).
// Example 1:
// Input: 1
// Output: 1
// Explanation: Under 8 bits 1 can be represented as 00000001. If we only reverse the significant protion of this we yield 00000001 which is 1 in binary.
// Example 2:
// Input: 10
// Output: 5
// Explanation: 10 is 1010 under base 2, 00001010 under 8 bits (notice how leading 0's do not add value). The reverse of 1010 is 0101, yielding 00000101 which is 5.
// Example 3:
// Input: 9090
// Output: 4209
// Explanation:
// 9090
// ---------
// Under base 10: 9090
// Under base 2: 10001110000010 (we just need 14 bits to represent this)
// Under base 2 (fit into 16 bits aka 2 bytes): 0010001110000010
// Under base 2 reversed: 01000001110001 (still 14 bits)
// Under base 2 (fit into 16 bits): 0001000001110001
// Constraints:
// input will always be strictly greater than or equal to 0

pub fn reverse_bits(input: usize) -> usize {
    let mut value = 0 as usize;

    let mut the_input = input;
    let mut the_bit = vec![];

    while the_input != 0 {
        let result = the_input % 2;
        the_bit.push(result);

        the_input = the_input / 2 as usize;
    }

    let bit_len = the_bit.len();

    for index in 0..bit_len {
        the_bit[index] = the_bit[index] * 2usize.pow((bit_len - index - 1) as u32);
    }
    println!("{}", value);

    value = the_bit.iter().sum();

    return value;
}

#[cfg(test)]
mod mod_reverse_bits {
    use super::*;

    #[test]
    fn test_one() {
        let result = reverse_bits(1);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_two_digits() {
        let result = reverse_bits(10);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_four_digits() {
        let result = reverse_bits(9090);
        assert_eq!(result, 4209);
    }

    #[test]
    fn test_zero() {
        let result = reverse_bits(0);
        assert_eq!(result, 0);
    }
}
