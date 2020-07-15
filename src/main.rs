#![allow(dead_code)]
use std::error::Error;
use std::fmt;
use std::io;
use std::vec::Vec;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Input sudoku. Spaces for empty cells. Press enter to go to next line.");

    let mut input = String::new();
    for _ in 0..9 {
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;

        if line.len() != 10 {
            //todo better?
            panic!("line was {} chars, but we need 9", line.len())
        }
        input.push_str(&line);
    }

    let sudoku = parse_input(input)?;
    sudoku.print();
    Ok(())
}

struct Square<T>([[T; 3]; 3]);

type Sudoku = Square<CellBox>;
type CellBox = Square<Cell>;

enum Cell {
    Known(u8),
    UnKnown(Vec<u8>),
}

impl<T> Square<T> {
    fn make(initial: fn() -> T) -> Square<T> {
        Square::<T>([
            [initial(), initial(), initial()],
            [initial(), initial(), initial()],
            [initial(), initial(), initial()],
        ])
    }
}

impl Sudoku {
    fn new() -> Sudoku {
        Sudoku::make(|| CellBox::new())
    }

    fn print(&self) {
        println!("-------------");
        print!("|")
    }
}

impl CellBox {
    fn new() -> CellBox {
        CellBox::make(|| Cell::unknown())
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

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cell::Known(x) => write!(f, "{}", x),
            Cell::UnKnown(_) => write!(f, " "),
        }
    }
}

fn parse_input(s: String) -> Result<Sudoku, &'static str> {
    let mut sudoku = Sudoku::new();
    for i in 0..3 {

    };
    Ok(sudoku)
}
