use std::fs::File;
use std::io::{Read, BufReader, BufRead};
use std::io;

fn read_input(path: &str) -> io::Result<Vec<usize>> {
    let file = File::open(path)?;
    
    let buf = BufReader::new(file);

    // let mut data = vec![];
    // file.read_to_end(&mut data)?;

    Ok(buf.lines()
    .map(|line| -> usize {
        line.expect("Invalid line").parse::<usize>().expect("Invalid number")
    })
    .collect::<Vec<usize>>())

}

fn count_depht_increase(input: &Vec<usize>) -> usize {
    let mut prev = &input[0];
    let mut increase_count = 0;
    for val in input.iter().skip(1) {
        if val > prev {
            increase_count += 1;
        }
        prev = val;
    }

    increase_count
}

fn main() {

    let input = read_input("input.txt").unwrap();

    let count = count_depht_increase(&input);
    println!("Increases count: {}", count);
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_depth_increases() {
        let input = vec![1, 2, 1, 4, 5, 3];
        let count = count_depht_increase(&input);

        assert_eq!(count, 3)
    }
}
