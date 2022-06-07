use std::fs;

enum State {
    ON,
    OFF,
    TOGGLE,
}

struct Range {
    x: u16,
    y: u16,
}

impl Range {
    fn from(input: &str) -> Range {
        let splits: Vec<&str> = input.split(',').collect();
        Range {
            x: splits[0].parse::<u16>().unwrap(),
            y: splits[1].parse::<u16>().unwrap(),
        }
    }
}

struct Command {
    from: Range,
    to: Range,
    action: State,
}

impl Command {
    fn from(input: &str) -> Command {
        let splits: Vec<&str> = input.split(' ').collect();
        if splits[1] == "off" {
            Command {
                from: Range::from(splits[2]),
                to: Range::from(splits[4]),
                action: State::OFF,
            }
        } else if splits[1] == "on" {
            Command {
                from: Range::from(splits[2]),
                to: Range::from(splits[4]),
                action: State::ON,
            }
        } else {
            Command {
                from: Range::from(splits[1]),
                to: Range::from(splits[3]),
                action: State::TOGGLE,
            }
        }
    }
}

fn update_grid(grid: &mut Vec<Vec<i32>>, command: &Command) {
    for r in command.from.x..=command.to.x {
        for c in command.from.y..=command.to.y {
            match command.action {
                State::ON => grid[r as usize][c as usize] = 1,
                State::OFF => grid[r as usize][c as usize] = 0,
                State::TOGGLE => {
                    grid[r as usize][c as usize] = if grid[r as usize][c as usize] == 1 {
                        0
                    } else {
                        1
                    }
                }
            }
        }
    }
}

fn update_grid_with_brightness(grid: &mut Vec<Vec<i32>>, command: &Command) {
    for r in command.from.x..=command.to.x {
        for c in command.from.y..=command.to.y {
            match command.action {
                State::ON => grid[r as usize][c as usize] += 1,
                State::OFF => {
                    grid[r as usize][c as usize] = if grid[r as usize][c as usize] == 0 {
                        0
                    } else {
                        grid[r as usize][c as usize] - 1
                    }
                }
                State::TOGGLE => grid[r as usize][c as usize] += 2,
            }
        }
    }
}

fn main() {
    let mut grid = vec![vec![0; 1000]; 1000];
    let content = fs::read_to_string("input.txt").unwrap();
    for line in content.lines() {
        let com = Command::from(line);
        update_grid(&mut grid, &com);
    }
    let mut count = 0;
    for i in 0..1000 {
        for j in 0..1000 {
            if grid[i as usize][j as usize] == 1 {
                count += 1;
            }
        }
    }

    println!("Lights on: {}", count);

    let mut grid = vec![vec![0; 1000]; 1000];
    for line in content.lines() {
        let com = Command::from(line);
        update_grid_with_brightness(&mut grid, &com);
    }
    let mut total_brightness = 0;
    for i in 0..1000 {
        for j in 0..1000 {
            total_brightness += grid[i as usize][j as usize];
        }
    }
    println!("Total brightness: {}", total_brightness);
}

#[cfg(test)]
mod tests {
    use crate::{update_grid, Command, State};

    #[test]
    fn command_parsing() {
        let com = Command::from("turn off 301,3 through 808,453");
        matches!(com.action, State::OFF);
        assert_eq!(com.from.x, 301);
        assert_eq!(com.from.y, 3);
        assert_eq!(com.to.x, 808);
        assert_eq!(com.to.y, 453);
    }

    #[test]
    fn update_grid_works() {
        let mut grid = vec![vec![0; 10]; 10];
        let com = Command::from("turn on 2,2 through 3,3");
        update_grid(&mut grid, &com);

        let com = Command::from("turn off 1,1 through 2,2");
        update_grid(&mut grid, &com);

        let mut count = 0;
        for i in 0..10 {
            for j in 0..10 {
                if grid[i as usize][j as usize] == 1 {
                    count += 1;
                }
            }
        }
        assert_eq!(count, 3);
    }
}
