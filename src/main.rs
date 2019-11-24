
use std::time::{SystemTime};
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

mod sudoku;
mod algorithms;

fn main() -> io::Result<()> {

    let time = SystemTime::now();
    let mut solved = 0_u32;
    let mut total = 0_u32;

    for i in 0..4 {
        let file = format!("sudokus/level{}.txt", i + 1);
        let fo = File::open(file)?;
        let reader = BufReader::new(fo);

        for line in reader.lines() {
            let unwrapped = line.unwrap();
            let string = unwrapped.as_str();
            
            let puzzle = sudoku::from_string(string);
            let mut solvable = sudoku::Sudoku::new(puzzle);

            let result : bool = algorithms::solve(&mut solvable);
            let formatted = solvable.to_string();

            total += 1;

            if result {
                solved += 1;
            }

            println!("{} {}", if result {'y'} else {'n'}, formatted);
        }
    }

    match time.elapsed() {
        Ok(elapsed) => {
            println!("");
            println!("Solved {}/{} sudokus in {} secs!", solved, total, elapsed.as_secs_f32());
        },
        Err(e) => {
            println!("Oops, something went wrong!");
        }
    }


    return Ok(());
}