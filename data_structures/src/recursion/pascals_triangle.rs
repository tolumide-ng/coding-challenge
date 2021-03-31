pub fn get_row(row_index: i32) -> Vec<i32> {
    let mut triangle: Vec<Vec<i32>> = vec![];

    if row_index < 0 {
        return vec![];
    }

    for index in 0..=row_index {
        for subindex in 0..=index {
            triangle.push(vec![]);

            if subindex == 0 {
                triangle[index as usize].push(1);
            }

            if index > 0 && subindex > 0 && subindex < index {
                if subindex == 0 {}
                let value = triangle[(index - 1) as usize][(subindex - 1) as usize]
                    + triangle[(index - 1) as usize][(subindex) as usize];

                triangle[index as usize].push(value);
            }

            if subindex == index && subindex > 0 {
                triangle[index as usize].push(1);
            }
        }
    }

    triangle[row_index as usize].clone()
}

pub fn get_row_two(row_index: i32) -> Vec<i32> {
    let row_index = row_index as usize + 1;
    let mut row = vec![1; row_index];

    for i in 2..row_index {
        for j in (1..i).rev() {
            row[j] += row[j - 1];
        }
    }

    row
}

#[cfg(test)]
mod test_get_row_mod {
    #[test]
    fn test_get_row() {
        use super::get_row;

        assert_eq!(get_row(3), vec![1, 3, 3, 1]);
    }

    #[test]
    fn test_get_row_zero() {
        use super::get_row;

        assert_eq!(get_row(0), vec![1]);
    }

    #[test]
    fn test_get_row_one() {
        use super::get_row;

        assert_eq!(get_row(1), vec![1, 1]);
    }

    #[test]
    fn test_get_row_inexistent() {
        use super::get_row;

        assert_eq!(get_row(-1), vec![]);
    }
}
