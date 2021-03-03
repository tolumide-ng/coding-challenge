/// Question: https://www.hackerrank.com/challenges/chief-hopper/problem?utm_campaign=challenge-recommendation&utm_medium=email&utm_source=24-hour-campaign

pub fn get_energy(buildings: Vec<i32>) -> i32 {
    let mut bot_energy = 0;
    if buildings.len() > 0 {
        // use the first building as tbe lowest energy to start with
        // bot_energy = 1;

        let mut current_energy = bot_energy;

        let mut restart = true;

        while restart {
            for space in 0..buildings.len() {
                let new_energy = calc_new_energy(buildings[space], current_energy);

                if new_energy < 0 {
                    bot_energy += 1;
                    current_energy = bot_energy;

                    break;
                }

                current_energy = new_energy;

                if space + 1 == buildings.len() && current_energy >= 0 {
                    restart = false;
                }
            }
        }
    }

    return bot_energy;
}

pub fn calc_new_energy(height: i32, energy: i32) -> i32 {
    if height > energy {
        return energy - (height - energy);
    }

    return energy + (energy - height);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bot_energy() {
        let buildings_results = vec![
            (vec![3, 4, 3, 2, 4], 4),
            (vec![4, 4, 4], 4),
            (vec![1, 6, 4], 3),
            (vec![4, 3, 2], 3),
        ];

        for test in buildings_results {
            assert_eq!(get_energy(test.0), test.1);
        }
    }
}
