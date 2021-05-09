use std::{collections::HashSet, fmt::format};

/// Given a robot cleaner in a room modeled as a grid.
/// Each cell in the grid can be empty or blocked.
/// The robot cleaner with 4 given APIs can move forward, turn left or turn right. Each turn it made is 90 degrees.
/// When it tries to move into a blocked cell, its bumper sensor detects the obstacle and it stays on the current cell.
/// design an algorithm to clean the entire room using only the 4 given APIs shown below.
/// 1. The input is only given to initialize the room and the robot's position internally. You must solve this problem "blindfolded". In other words, you must control the robot using only the mentioned 4 APIs, without knowing the room layout and the initial robot's position.
/// 2. The robot's initial position will always be in an accessible cell.
/// 3. The initial direction of the robot will be facing up.
/// 4. All accessible cells are connected, which means the all cells marked as 1 will be accessible by the robot.
/// 5. Assume all four edges of the grid are all surrounded by wall.

// #[derive(Default)]
pub struct Position {
    row: u16,
    col: u16,
    direction: u16,
}

impl Position {
    fn new(row: u16, col: u16, direction: u16) -> Self {
        Position {
            row,
            col,
            direction,
        }
    }
}

// #[derive(Default)]
pub struct Robot {
    current: Position,
    total_rows: u16,
    total_col: u16,
    set: HashSet<String>,
}

impl Robot {
    pub fn new(rows: u16, cols: u16, row: u16, col: u16) -> Self {
        Robot {
            current: Position::new(row, col, 0),
            total_col: cols,
            total_rows: rows,
            set: HashSet::new(),
        }
    }

    pub fn turnLeft(&mut self) -> u16 {
        if self.current.direction == 270 {
            self.current.direction = 0;
        } else {
            self.current.direction += 90;
        }
        return self.current.direction;
    }

    pub fn turnRight(&mut self) -> u16 {
        if self.current.direction == 0 {
            self.current.direction = 270;
        } else {
            self.current.direction -= 90;
        }

        return self.current.direction;
    }

    pub fn move_robot(&mut self) {
        match self.current.direction {
            0 => self.current.row -= 1,
            90 => self.current.col += 1,
            180 => self.current.row += 1,
            270 => self.current.col -= 1,
            _ => {}
        }
    }

    pub fn clean_room(&mut self, row: u16, col: u16) {
        let current = format!("{}-{}", row, col);

        if !self.set.contains(&current) {
            self.set.insert(current);
        }
    }
}

pub fn clean_room(rows: u16, cols: u16, col: u16, row: u16, room: Vec<Vec<u8>>) {
    let robot = Robot::new(rows, cols, row, col);

    for room_row in room {
        for room_cell in room_row {}
    }
}
