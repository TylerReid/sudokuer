#![allow(dead_code)]
use std::error::Error;
use std::fmt;
use std::io;
use std::vec::Vec;
use array2d::Array2D;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Input sudoku. Spaces for empty cells. Press enter to go to next line.");

    let mut input = String::new();
    for _ in 0..9 {
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        trim_newline(&mut line);

        if line.len() != 9 {
            //todo better?
            panic!("line was {} chars, but we need 9", line.len())
        }
        input.push_str(&line);
    }

    let sudoku = parse_input(&input);
    println!("{:?}", sudoku.board);
    Ok(())
}

struct Sudoku {
    board: Array2D<Cell>,
}

#[derive(Debug, Clone)]
enum Cell {
    Known(u32),
    UnKnown(Vec<u32>),
}

impl Cell {
    fn unknown() -> Cell {
        Cell::UnKnown(vec![1, 2, 3, 4, 5, 6, 7, 8, 9])
    }

    fn known(value: u32) -> Cell {
        Cell::Known(value)
    }

    fn remove(&mut self, value: u32) {
        if let Cell::UnKnown(unknown) = self {
            unknown.retain(|&x| x != value)
        }
    }

    fn has_value(&self, value: u32) -> bool {
        match self {
            Cell::Known(x) => *x == value,
            Cell::UnKnown(x) => x.contains(&value),
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cell::Known(x) => write!(f, "{}", x),
            Cell::UnKnown(_) => write!(f, " "),
        }
    }
}

fn parse_input(s: &str) -> Sudoku {
    let mut cells = Vec::<Cell>::with_capacity(81);

    for c in s.chars() {
        let cell = match c.to_digit(10) {
            Some(x) => Cell::known(x),
            None => Cell::unknown(),
        };
        cells.push(cell);
    }

    Sudoku {
        board: Array2D::from_rows(&[cells]),
    }
}

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}
