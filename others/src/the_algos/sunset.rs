///This problem was asked by Mailchimp.
/// You are given an array representing the heights of neighboring buildings on a city street,
/// from east to west. The city assessor would like you to write an algorithm that returns how
/// many of these buildings have a view of the setting sun, in order to properly value the street.
/// For example, given the array [3, 7, 8, 3, 6, 1], you should return 3, since the top floors
/// of the buildings with heights 8, 6, and 1 all have an unobstructed view to the west.
/// Can you do this using just one forward pass through the array?

pub struct Buildings {
    total: usize,
    buildings: Vec<usize>,
}

pub fn east_to_west(list: Vec<usize>) -> Buildings {
    let mut buildings: Vec<usize> = vec![];

    for build in list {
        buildings.push(build);

        for index in 0..buildings.len() {
            if index < buildings.len() {
                if buildings[index] < build {
                    buildings.remove(index);
                }
            }
        }
    }

    Buildings {
        total: buildings.len(),
        buildings,
    }
}

/// There is a much simpler approach where all that is needed is only one loop
///
pub fn east_to_west_revised(list: Vec<usize>) -> usize {
    let mut buildings = vec![];
    let total_buildings = list.len() - 1; // 6

    // buildings.push(list[total_buildings]);

    if list.len() > 0 {
        let mut min_height = usize::MIN;

        for height in 0..total_buildings {
            let curr_building = list[total_buildings - height];
            if curr_building > min_height {
                min_height = curr_building;
                buildings.push(curr_building);
            }
        }
    }

    return buildings.len();
}

#[cfg(test)]
mod east_west_tests {
    use super::*;

    #[test]
    fn test_east_to_west() {
        let all_buildings = east_to_west(vec![3, 7, 8, 3, 6, 1]);

        assert_eq!(all_buildings.total, 3);
        assert!(all_buildings.buildings.contains(&8));
        assert!(all_buildings.buildings.contains(&6));
        assert!(all_buildings.buildings.contains(&1));
    }

    #[test]
    fn test_empty_east_to_west() {
        let all_buildings = east_to_west(vec![]);

        assert_eq!(all_buildings.total, 0);
    }

    #[test]
    fn test_east_to_west_rev() {
        let all_buildings = east_to_west_revised(vec![3, 7, 8, 3, 6, 1]);

        assert_eq!(all_buildings, 3);
    }
}
