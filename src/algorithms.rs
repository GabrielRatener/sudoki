
use std::collections::{VecDeque};

use crate::sudoku;
use crate::sudoku::{Sudoku};

type CellCollection = [sudoku::Index; sudoku::SIZE as usize];

fn box_indexes(n : u32) -> CellCollection {
    let x0 = sudoku::ORDER * (n % sudoku::ORDER);
    let y0 = sudoku::ORDER * (n / sudoku::ORDER);
    let mut indexes : CellCollection = [0; sudoku::SIZE as usize];
    let mut i : usize = 0;

    for y in y0..(y0 + sudoku::ORDER) {
        for x in x0..(x0 + sudoku::ORDER) {
            let index = sudoku::index(&sudoku::Point::new(x, y));

            indexes[i] = index as usize;
            
            i += 1;
        }
    }

    return indexes;
}

fn row_indexes(y : u32) -> CellCollection {
    let mut indexes : CellCollection = [0; sudoku::SIZE as usize];

    for i in 0..sudoku::SIZE {
        indexes[i as usize] = sudoku::index(&sudoku::Point::new(i, y));
    }

    return indexes;
}

fn col_indexes(x : u32) -> CellCollection {
    let mut indexes : CellCollection = [0; sudoku::SIZE as usize];

    for i in 0..sudoku::SIZE {
        indexes[i as usize] = sudoku::index(&sudoku::Point::new(x, i));
    }

    return indexes;
}

fn clean(puzzle : & mut Sudoku, indexes : CellCollection) {
    let mut index_queue : VecDeque<sudoku::Index> = VecDeque::new();
    let mut value_queue : VecDeque<sudoku::Value> = VecDeque::new();

    for index in indexes.iter() {
        let value = puzzle.view(*index);

        if value == 0 {
            index_queue.push_front(*index);
        } else {
            value_queue.push_front(value);
        }
    }

    for index in index_queue.iter() {
        for value in value_queue.iter() {
            puzzle.eliminate(*index, *value);
        }
    }
}

fn fill(puzzle : & mut Sudoku, indexes : CellCollection) {
    let mut target : sudoku::Index = 0;
    let mut set = sudoku::candidate_set();
    let mut empty_count = 0_u32;

    for index in indexes.iter() {
        let value = puzzle.view(*index);

        if value > 0 {
            set.remove(&value);
        } else {
            if empty_count > 0 {
                return;
            } else {
                empty_count += 1;

                target = *index;
            }
        }
    }

    if empty_count == 1 {
        for val in set.iter() {

            puzzle.fill(target, *val);
        }
    }
}

fn value_fill(puzzle : &mut Sudoku, indexes : CellCollection) {

    for value in 1..=sudoku::SIZE {
        let mut occurences = 0_u32;
        let mut target : sudoku::Index = 0;

        for index in indexes.iter() {
            if puzzle.is_candidate(*index, value) {
                occurences += 1;

                if occurences > 1 {
                    break;
                } else {
                    target = *index;
                }
            }
        }

        if occurences == 1 {
            puzzle.fill(target, value);
        }
    }
}

fn do_fills(puzzle : &mut Sudoku) {

    for i in 0..sudoku::SIZE {

        fill(puzzle, box_indexes(i));
        fill(puzzle, row_indexes(i));
        fill(puzzle, col_indexes(i));
    }
}

fn do_value_fills(puzzle : &mut Sudoku) {

    for i in 0..sudoku::SIZE {

        value_fill(puzzle, box_indexes(i));
        value_fill(puzzle, row_indexes(i));
        value_fill(puzzle, col_indexes(i));
    }
}

fn do_clean(puzzle : &mut Sudoku) {

    for i in 0..sudoku::SIZE {

        clean(puzzle, box_indexes(i));
        clean(puzzle, row_indexes(i));
        clean(puzzle, col_indexes(i));
    }
}

pub fn solve(puzzle : &mut Sudoku) -> bool {
    loop {
        let progress = puzzle.progress();

        do_clean(puzzle);

        do_fills(puzzle);
        do_value_fills(puzzle);

        if puzzle.complete() {
            return true;
        }

        if puzzle.progress() == progress {
            return false;
        }
    }
}
