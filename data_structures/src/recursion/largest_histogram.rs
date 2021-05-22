struct LargestRectangle {
    stack: Vec<usize>,
    heights: Vec<i32>,
    area: i32,
    max_area: i32,
}

impl LargestRectangle {
    pub fn new(heights: Vec<i32>) -> Self {
        LargestRectangle {
            stack: vec![],
            heights,
            area: -1, 
            max_area: -1,
        }
    }

    pub fn get_rectangle(&mut self) {
        for index in 0..self.heights.len() {
            if self.stack.is_empty() || self.heights[index] < self.heights[self.stack.len() -1] {
                self.update_area(index);
            }

            self.stack.push(index);
        }

        while self.stack.last().is_some() {
            self.update_area(self.heights.len())
        }
    }

    pub fn update_area(&mut self, index: usize){
        if let Some(active_index) = self.stack.pop() {
            if let Some(index_at_top) = self.stack.last() {
                self.area = self.heights[active_index] * (index - index_at_top -1) as i32;
            } else {
                self.area = self.heights[active_index] * index as i32;
            }
        };

        self.max_area = std::cmp::max(self.area, self.max_area);
    }
}


pub fn get_largest_rectangle(heights: Vec<i32>) -> i32 {
    let mut histogram = LargestRectangle::new(heights);
    histogram.get_rectangle();

    return histogram.max_area;
}



#[cfg(test)]
mod test_largest_histogram_mod {
    use crate::recursion::largest_histogram::get_largest_rectangle;


    #[test]
    fn test_largest_histogram() {
        let rectangles = vec![2, 4, 3, 6, 4];

        assert_eq!(get_largest_rectangle(rectangles), 12);
    }
}