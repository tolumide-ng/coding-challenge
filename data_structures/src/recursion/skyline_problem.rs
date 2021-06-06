#[derive(Debug)]
struct Building {
    start: i32,
    end: i32,
    height: i32,
}

impl Building {
    pub fn new(start: i32, end: i32, height: i32) -> Self {
        Building { start, end, height }
    }

    pub fn change_start(&mut self, start: i32) {
        self.start = start;
    }
}

pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut store: Vec<Building> = vec![];

    for index in 0..buildings.len() {
        match index {
            0 =>{ 

                store.push(Building::new(
                    buildings[index][0],
                    buildings[index][1],
                    buildings[index][2],
            ))
        },
            _ => {
                let prev = &store[store.len() - 1];
                let mut curr = Building::new(
                    buildings[index][0],
                    buildings[index][1],
                    buildings[index][2],
                );

                if prev.end > curr.start && prev.end > curr.end && prev.height > curr.height {
                    continue;
                }

                // if it is intersecting but current's with extends beyond previous
                if prev.height > curr.height && prev.end > curr.start && curr.end > prev.end {

                    curr.change_start(prev.end);


                    store.push(curr);
                    continue;
                }

                // if prev.height > curr.height && prev.end > curr.start && prev.end > prev.end {}

                if prev.end >= curr.start && prev.height < curr.height {
                    store.push(curr);
                    continue;
                }

                // if there is space between them
                if curr.start > prev.end {
                    store.push(Building::new(prev.end, prev.end, 0));
                    store.push(curr);
                    continue;
                }

                if prev.height == curr.height {

                    println!("______________________________________");

                    let the_prev = store.pop();

                    if let Some(new_curr) = the_prev {
                        store.push(Building::new(new_curr.start, curr.end, curr.height));
                    }

                    continue;
                }
            }
        }
    }

    let last = &store[store.len() -1];


    store.push(Building::new(last.end, last.end, 0));



    return  store.iter().map(|x| vec![x.start, x.height]).collect::<Vec<Vec<i32>>>();

}

#[cfg(test)]
mod test_skyline_mod {
    use crate::recursion::skyline_problem::get_skyline;

    #[test]
    fn test_skyline() {
        // let buildings = [
        //     vec![2, 9, 10],
        //     vec![3, 7, 15],
        //     vec![5, 12, 12],
        //     vec![15, 20, 10],
        //     vec![19, 24, 8],
        // ]
        // .to_vec();

        // let result = get_skyline(buildings);

        // println!("the buildings {:#?}", result);




        // let buildings = vec![vec![0,2,3],vec![2,5,3]];

        // let result = get_skyline(buildings);

        // println!("the buildings {:#?}", result);


        // assert_eq!(3, 4);


        // let buildings = vec![vec![2,9,10], vec![9,12,15]];

        // let result = get_skyline(buildings);

        // println!("the buildings {:#?}", result);


        // assert_eq!(3, 4);


        let buildings = vec![vec![1,2,1],vec![1,2,2],vec![1,2,3]];

         let result = get_skyline(buildings);

         println!("the buildings {:#?}", result);
         assert_eq!(3, 4);



        // [[1,2,1],[1,2,2],[1,2,3]]

    }
}
