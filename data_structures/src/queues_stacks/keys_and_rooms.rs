use std::usize;

pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
    let mut possible = false;

    if rooms.len() > 0 {
        let mut visited: Vec<i32> = vec![];
        let mut current_index = 0;

        let mut store = Vec::new();

        add_to_store(&mut current_index, &mut store, 0, &mut visited, &rooms);

        while !store.is_empty() {
            let key = store.pop().unwrap();

            add_to_store(&mut current_index, &mut store, key, &mut visited, &rooms);
        }

        if current_index == rooms.len() || rooms.len() == 1 {
            possible = true;
        }
    }

    return possible;
}

pub fn add_to_store(
    current_index: &mut usize,
    store: &mut Vec<i32>,
    current_room: i32,
    visited: &mut Vec<i32>,
    rooms: &Vec<Vec<i32>>,
) {
    visited.push(current_room);
    for index in 0..rooms[current_room as usize].len() {
        let key = rooms[current_room as usize][index];

        if !visited.contains(&key) && !store.contains(&key) && key < rooms.len() as i32 {
            store.push(key);
        }
    }
    store.sort();
    store.reverse();

    if *current_index != rooms[current_room as usize].len() || *current_index > 0 {
        *current_index += 1;
    }
}

#[cfg(test)]
mod can_visit_all_rooms_cont {
    use super::*;

    #[test]
    fn test_keys_and_rooms() {
        assert_eq!(
            can_visit_all_rooms(vec![vec![1], vec![2], vec![3], vec![]]),
            true
        );

        assert_eq!(
            can_visit_all_rooms(vec![vec![1, 3], vec![3, 0, 1], vec![2], vec![0]]),
            false
        );

        assert_eq!(can_visit_all_rooms(vec![vec![2], vec![], vec![1]]), true);

        assert_eq!(
            can_visit_all_rooms(vec![vec![1], vec![2], vec![3], vec![]]),
            true
        );

        assert_eq!(can_visit_all_rooms(vec![vec![]]), true);
    }
}
