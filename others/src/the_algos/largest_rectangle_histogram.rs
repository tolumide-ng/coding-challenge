mod largest_rectangle_histogram {
    pub fn histogram_brute_force(heights: Vec<i32>) -> i32 {
        let mut max_area = 0;
        for index in 0..heights.len() {
            let mut curr_index = 0;
            let mut local_area = 0;
            let local_min = heights[index];
            while curr_index < heights.len() {
                let local_current = heights[curr_index];
                if local_current < local_min && curr_index < index {
                    local_area = 0;
                    curr_index += 1;
                    continue;
                }
                if local_current < local_min && curr_index > index {
                    break;
                }
                local_area += local_min;
                curr_index += 1;
            }
            if local_area > max_area {
                max_area = local_area;
            }
        }
        return max_area;
    }

    pub fn calculate_area(
        store: &Vec<usize>,
        value_at_top: i32,
        current_index: usize,
        stack_at_top: Option<&usize>,
    ) -> i32 {
        if store.is_empty() {
            return value_at_top * current_index as i32;
        } else {
            return value_at_top * (current_index - stack_at_top.unwrap() - 1) as i32;
        }
    }

    pub fn histogram_stack_approach(heights: Vec<i32>) -> i32 {
        let mut max_area: i32 = -1;
        let mut top: Option<usize> = None;

        let mut store: Vec<usize> = vec![];

        for index in 0..heights.len() {
            let current = heights[index];

            match store.is_empty() {
                true => store.push(index),
                false => {
                    while !store.is_empty() && current < heights[*store.last().unwrap()] {
                        std::mem::swap(&mut top, &mut store.pop());
                        let new_area =
                            calculate_area(&store, heights[top.unwrap()], index, store.last());
                        if new_area > max_area {
                            max_area = new_area;
                        }
                    }
                    store.push(index);
                }
            }
        }

        while !store.is_empty() {
            std::mem::swap(&mut top, &mut store.pop());
            let new_area =
                calculate_area(&store, heights[top.unwrap()], heights.len(), store.last());
            if new_area > max_area {
                max_area = new_area;
            }
        }

        return max_area;
    }
}

#[cfg(test)]
mod test_largest_rectangle_histogram_mod {
    use crate::the_algos::largest_rectangle_histogram::largest_rectangle_histogram::{
        histogram_brute_force, histogram_stack_approach,
    };

    #[test]
    fn test_lrh() {
        assert_eq!(histogram_brute_force(vec![2, 1, 5, 6, 2, 3]), 10);
        assert_eq!(histogram_brute_force(vec![2, 4]), 4);
        assert_eq!(histogram_brute_force(vec![2, 1, 2]), 3);
        assert_eq!(histogram_brute_force(vec![1, 1, 1, 1, 1, 1, 6]), 7);
        assert_eq!(histogram_brute_force(vec![3, 6, 5, 7, 4, 8, 1, 0]), 20);
    }

    #[test]
    fn test_stack_approach() {
        assert_eq!(histogram_stack_approach(vec![1, 3, 2, 1, 2]), 5);
        assert_eq!(histogram_stack_approach(vec![2, 1, 2, 3, 1]), 5);
        assert_eq!(histogram_stack_approach(vec![2, 1, 5, 6, 2, 3]), 10);
        assert_eq!(histogram_stack_approach(vec![2, 4]), 4);
        assert_eq!(histogram_stack_approach(vec![2, 1, 2]), 3);
        assert_eq!(histogram_stack_approach(vec![1, 1, 1, 1, 1, 1, 6]), 7);
        assert_eq!(histogram_stack_approach(vec![3, 6, 5, 7, 4, 8, 1, 0]), 20);
    }
}
