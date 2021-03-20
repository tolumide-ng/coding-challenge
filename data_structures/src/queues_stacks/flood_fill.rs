use std::{collections::VecDeque};

#[derive(Debug, Default)]
pub struct Position {
    x: usize,
    y: usize,
}


pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
    let mut the_image: Vec<Vec<i32>> = image.clone();
    let mut queue: VecDeque<Position> = VecDeque::new();

    if sc < image.len() as i32 && sr < image[0].len() as i32 {
        let prev_color = &image[sr as usize][sc as usize];

        queue.push_back(Position{x: sr as usize, y: sc as usize});

        while !queue.is_empty() {
            let current = queue.pop_front().unwrap();

            let neighbours = get_4_connected(current.x, current.y, &the_image, prev_color, new_color);

            for point in neighbours {
                queue.push_back(point);
            }

            the_image[current.x][current.y] = new_color;
        }

    }

    return the_image;
}

pub fn get_4_connected(x: usize, y: usize, image: &Vec<Vec<i32>>, prev_color: &i32, new_color: i32) -> Vec<Position> {
    let mut points: Vec<Position> = vec![];
    if x  != 0 && image[x-1][y] == *prev_color && image[x-1][y] != new_color  {
            points.push(Position {x: x-1, y});
    }


    if x < image.len()-1 && image[x+1][y] == *prev_color && image[x+1][y] != new_color {
            points.push(Position{x: x+1, y});
    }

    if y > 0 && image[x][y-1] == *prev_color && image[x][y-1] != new_color  {
            points.push(Position{x, y: y-1});

    }

    if y < image[0].len()-1 && image[x][y+1] == *prev_color &&  image[x][y+1] != new_color {
            points.push(Position{x, y: y+1});
    }

    return points;
    // println!("THE REAL POINTS {:#?}", points);
    // return vec![];
}

#[cfg(test)]
mod test_flood_fill_cont {
    use super::*;

    #[test]
    fn test_flood_fill() {
        let image: Vec<Vec<i32>> = vec![vec![1,1,1],vec![1,1,0],vec![1,0,1]];
        let expected = vec![[2,2,2],[2,2,0],[2,0,1]];

        assert_eq!(flood_fill(image, 1, 1, 2), expected);

        let image: Vec<Vec<i32>> = vec![vec![0,0,0],vec![0,0,0]];
        let expected = vec![[2,2,2],[2,2,2]];


        assert_eq!(flood_fill(image, 0, 0, 2), expected);


        let image = vec![vec![0,0,0],vec![0,1,1]];
        let expected = vec![vec![0, 0, 0], vec![0, 1, 1]];

        assert_eq!(flood_fill(image, 1, 1, 1), expected);
        // assert_eq!(3, 4);
    }
}