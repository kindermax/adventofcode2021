const NUMBERS: &'static str = include_str!("../input_numbers.txt");
const BOARDS: &'static str = include_str!("../input_boards.txt");
const BOARDS_TEST: &'static str = include_str!("../input_boards_test.txt");


fn read_numbers(raw_data: &str) -> Vec<usize> {
    raw_data.split(',').map(|s| s.parse::<usize>().expect("invalid number")).collect()
}

fn read_boards(raw_data: &str, size: usize) -> Vec<Board> {
    raw_data.split_terminator("\n\n")
    .map(|line| {
        let mut board = Board::new(size);
        line.split_whitespace().for_each(|raw_num| {
            let num = raw_num.parse::<usize>().expect("invalid board number");
            board.add(num);
        });
        board
    })
    .collect()
}

fn play_game(boards: &Vec<Board>, numbers: &Vec<usize>) -> Option<(Board, usize)> {
    let mut my_boards: Vec<Board> = boards.into_iter().map(|b| b.clone()).collect();

    for num in numbers.iter() {
        for b in my_boards.iter_mut() {
            b.mark(num);
            if b.is_bingo() {
                return Some((b.clone(), num.clone()));
            }
        }
    }

    None
}

fn play_game_win_last(boards: &Vec<Board>, numbers: &Vec<usize>) -> Option<(Board, usize)> {
    let mut my_boards: Vec<Board> = boards.into_iter().map(|b| b.clone()).collect();

    let mut last_wining_board: Option<Board> = None;
    let mut wining_num: usize = 0;
    for num in numbers.iter() {
        for b in my_boards.iter_mut() {
            if !b.is_bingo() { 
                b.mark(num);
            } else { 
                continue;
            }

            if b.is_bingo() {
                last_wining_board = Some(b.clone());
                wining_num = num.clone();
            }
        }
    }

    match last_wining_board {
        Some(b) => Some((b, wining_num)),
        None => None
    }

}


#[derive(Debug, PartialEq, Eq, Clone)]
enum Cell {
    Marked(usize),
    Unmarked(usize),
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Board {
    board: Vec<Vec<Cell>>,
    size: usize
}

impl Board {
    fn new(size: usize) -> Board {
        let mut board = vec![];
        for _ in 0..size {
            board.push(vec![]);
        }
        Board { board, size }
    }

    fn add(&mut self, number: usize) {
        for line in self.board.iter_mut() {
            if line.len() < self.size {
                line.push(Cell::Unmarked(number));
                break;
            }
        }
    }

    fn mark(&mut self, number: &usize) {
        for line in self.board.iter_mut() {
            let pos = line.iter().position(|c| match c {
                Cell::Unmarked(val) => val == number,
                Cell::Marked(_) => false,
            });

            match pos {
                Some(pos) => {
                    line[pos] = Cell::Marked(number.clone());
                    break;
                },
                None => continue
            }
        }
    }

    fn is_bingo(&mut self) -> bool {
        for line in self.board.iter_mut() {
            let is_bingo = line.iter().all(|cell| match cell {
                Cell::Marked(_) => true,
                Cell::Unmarked(_) => false,
            });
            if is_bingo  {
                return true;
            }
        }

        for idx in 0..self.size {
            let mut marked_count = 0;
            for line in self.board.iter() {
                if matches!(line[idx], Cell::Marked(_)) { 
                    marked_count += 1;
                }
            }
            if marked_count == self.size {
                return true;
            }
        }

        false
    }

    fn count_unmarked(&self) -> usize {
        self.board.iter()
        .flatten()
        .filter_map(|cell| match cell {
            Cell::Marked(_) => None,
            Cell::Unmarked(v) => Some(v),
        })
        .sum()
    }
}


fn main() {
    let numbers = read_numbers(NUMBERS);
    let boards = read_boards(BOARDS, 5);
    let (board, number) = play_game(&boards, &numbers).unwrap();
    println!("Sum {}, number {}, result {}", board.count_unmarked(), number, board.count_unmarked() * number);


    let (board, number) = play_game_win_last(&boards, &numbers).unwrap();
    println!("Sum {}, number {}, result {}", board.count_unmarked(), number, board.count_unmarked() * number);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_numbers() {
        let numbers = read_numbers("1,2,3");
        assert_eq!(numbers, vec![1, 2, 3]);
    }

    #[test]
    fn test_read_boards() {
        let boards = read_boards(
            r"1 2
              3 4

              5 6
              7 8
            ", 2
        );
        let mut b1 = Board::new(2);
        b1.add(1);
        b1.add(2);
        b1.add(3);
        b1.add(4);
        assert_eq!(boards[0], b1);

        let mut b2 = Board::new(2);
        b2.add(5);
        b2.add(6);
        b2.add(7);
        b2.add(8);
        assert_eq!(boards[1], b2);
    }

    #[test]
    fn test_play_game_example() {
        let numbers = vec![7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1];

        let boards = read_boards(
            BOARDS_TEST,
            5
        );
        // line
        let (board, number) = play_game(&boards, &numbers).unwrap();
        assert_eq!(board.count_unmarked(), 188);
        assert_eq!(number, 24);
    }

    #[test]
    fn test_play_game_win_last_example() {
        let numbers = vec![7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1];

        let boards = read_boards(
            BOARDS_TEST,
            5
        );
        // line
        let (board, number) = play_game_win_last(&boards, &numbers).unwrap();
        assert_eq!(board.count_unmarked(), 148);
        assert_eq!(number, 13);
    }

}