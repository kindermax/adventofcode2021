use std::str::FromStr;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::io;

#[derive(Debug)]
enum Command {
    Forward(usize),
    Down(usize),
    Up(usize),
}


impl FromStr for Command {
    type Err = String;
    fn from_str(s: &str) -> Result<Command, Self::Err> {
        use self::Command::*;

        let parts: Vec<&str> = s.split(' ').collect();
        let (raw_command, value) = (parts[0], parts[1]);

        let value = match value.parse::<usize>() {
            Ok(n) => n,
            Err(err) => {
                panic!("failed to parse command value {}: {}", value, err)
            }
        };

        match raw_command {
            "forward" => Ok(Forward(value)),
            "down" => Ok(Down(value)),
            "up" => Ok(Up(value)),
            _ => panic!("Invalid command {}", raw_command)
        }
    }
}

fn read_input(path: &str) -> io::Result<Vec<Command>> {
    let file = File::open(path)?;
    let buf = BufReader::new(file);

    let commands = buf
    .lines()
    .map(|line|
        line
        .expect("invalid line")
        .parse::<Command>()
        .expect("invalid command")
    )
    .collect();

    Ok(commands)

}

fn determine_position_and_depth(commands: &Vec<Command>) -> (usize, usize) {
    let mut position = 0;
    let mut depth = 0;

    for command in commands.iter() {
        match command {
            Command::Forward(value) => position += value,
            Command::Down(value) => depth += value,
            Command::Up(value) => depth -= value,
        };
    }

    (position, depth)
}

fn determine_position_and_depth_with_aim(commands: &Vec<Command>) -> (usize, usize) {
    let mut position = 0;
    let mut depth = 0;
    let mut aim = 0;

    for command in commands.iter() {
        match command {
            Command::Forward(value) => {
                position += value;
                depth += aim * value;
            },
            Command::Down(value) => aim += value,
            Command::Up(value) => aim -= value,
        };
    }

    (position, depth)
}

fn main() {
    let commands = read_input("input.txt").unwrap();
    let (pos, depth) = determine_position_and_depth(&commands);
    let (pos_with_aim, depth_with_aim) = determine_position_and_depth_with_aim(&commands);

    println!("Got pos {} and depth {}, total {}", pos, depth, pos * depth);
    println!("Got pos {} and depth {} with aim, total {}", pos_with_aim, depth_with_aim, pos_with_aim * depth_with_aim);
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_determine_position_and_depth() {
        use super::Command::*;

        let commands = vec![
            Forward(5),
            Down(5),
            Forward(8),
            Up(3),
            Down(8),
            Forward(2),
        ];
        let (pos, depth) = determine_position_and_depth(&commands);

        assert_eq!(pos, 15);
        assert_eq!(depth, 10);
        assert_eq!(pos * depth, 150);
    }

    #[test]
    fn test_determine_position_and_depth_with_aim() {
        use super::Command::*;

        let commands = vec![
            Forward(5),
            Down(5),
            Forward(8),
            Up(3),
            Down(8),
            Forward(2),
        ];
        let (pos, depth) = determine_position_and_depth_with_aim(&commands);

        assert_eq!(pos, 15);
        assert_eq!(depth, 60);
        assert_eq!(pos * depth, 900);
    }
}
