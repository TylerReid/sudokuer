#![allow(dead_code)]
use std::io;
use std::vec::Vec;

fn main() -> Result<(), std::io::Error> {
    println!("Input sudoku. Spaces for empty cells. Press enter to go to next line.");

    let mut sudoku = Sudoku::new();

    for _ in 0..3 {
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        trim_newline(&mut line);

        if line.len() != 9 {
            //todo better
            panic!("line was {} chars, but we need 9", line.len())
        }

        println!("{}", line);
    }

    Ok(())
}

struct Square<T> {
    top_left: T,
    top_center: T,
    top_right: T,

    middle_left: T,
    middle_center: T,
    middle_right: T,

    bottom_left: T,
    bottom_center: T,
    bottom_right: T,
}

type Sudoku = Square<Box>;
type Box = Square<Cell>;

enum Cell {
    Known(u8),
    UnKnown(Vec<u8>),
}

impl<T> Square<T> {
    fn make(initial: fn() -> T) -> Square<T> {
        Square::<T> {
            top_left: initial(),
            top_center: initial(),
            top_right: initial(),

            middle_left: initial(),
            middle_center: initial(),
            middle_right: initial(),

            bottom_left: initial(),
            bottom_center: initial(),
            bottom_right: initial(),
        }
    }
}

impl Sudoku {
    fn new() -> Sudoku {
        Sudoku::make(|| Box::new())
    }
}

impl Box {
    fn new() -> Box {
        Box::make(|| Cell::unknown())
    }
}

impl Cell {
    fn unknown() -> Cell {
        Cell::UnKnown(vec![1, 2, 3, 4, 5, 6, 7, 8, 9])
    }

    fn known(value: u8) -> Cell {
        Cell::Known(value)
    }

    fn remove(&mut self, value: u8) {
        if let Cell::UnKnown(unknown) = self {
            unknown.retain(|&x| x != value)
        }
    }

    fn has_value(&self, value: u8) -> bool {
        match self {
            Cell::Known(x) => *x == value,
            Cell::UnKnown(x) => x.contains(&value),
        }
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
