
use std::collections::{HashSet, HashMap, LinkedList};

pub const ORDER : u32 = 3;
pub const SIZE : u32 = ORDER * ORDER;
pub const VOLUME : u32 = SIZE * SIZE;

pub type Index = usize;
pub type Value = u32;

pub enum ProgressType {
    FILL,
    ELIMINATE
}

pub struct Progress {
    kind: ProgressType,
    index: Index,
    value: Value,
}

pub struct SudokuPuzzle {
    values: [Value; VOLUME as usize]
}

pub struct Sudoku {
    puzzle: SudokuPuzzle,
    candidates: HashMap<Index, HashSet<Value>>,
    empty: u32,
    stack: LinkedList<Progress>,
}

pub struct Point {
    x: u32,
    y: u32,
}

impl Point {
    pub fn new(x : u32, y : u32) -> Self {
        return Self {
            x: x,
            y: y
        }
    }
}

pub fn index(point : &Point) -> Index {
    return (point.y * SIZE + point.x) as Index;
}

pub fn point(i : Index) -> (u32, u32) {
    return ((i as u32) % SIZE, (i as u32) / SIZE);
}

pub fn from_string(string : &str) -> SudokuPuzzle {
    let mut values : [Value; VOLUME as usize] = [0; VOLUME as usize];
    
    for (i, c) in string.chars().enumerate() {
        let value = c.to_digit(10);

        values[i] = value.unwrap();
    }

    return SudokuPuzzle {
        values: values
    }
}

impl Sudoku {

    pub fn new(puzzle : SudokuPuzzle) -> Self {
        let mut empty_count = 0;
        let mut candidates : HashMap<Index, HashSet<Value>> = HashMap::new();

        for i in 0..puzzle.values.len() {
            if puzzle.values[i as usize] == 0 {
                let mut set : HashSet<Value> = HashSet::with_capacity(9);

                for n in 1..=9 {
                    set.insert(n);
                }

                empty_count += 1;

                candidates.insert(i as Index, set);
            }
        }

        return Self {
            puzzle: puzzle,
            candidates: candidates,
            empty: empty_count,
            stack: LinkedList::new()
        }
    }

    pub fn view(&self, i : Index) -> Value {
        if self.puzzle.values[i] > 0 {
            return self.puzzle.values[i];
        } else {

            match self.candidates.get(&i) {
                None => 0,
                Some(candidates) => {
                    // println!("len {}", candidates.len());

                    if candidates.len() == 1 {

                        match candidates.iter().next() {
                            None => {
                                return 0;
                            },
                            Some(candidate) => {
                                return *candidate;
                            }
                        }
                    } else {
                        return 0;
                    }
                }
            }
        }
    }

    pub fn is_candidate(&self, i : Index, val : Value) -> bool {
        if self.puzzle.values[i] > 0 {
            return self.puzzle.values[i] == val;
        } else {
            match self.candidates.get(&i) {
                None => {
                    return false;
                },
                Some(possible) => {
                    return possible.contains(&val);
                }
            }
        }
    }

    pub fn eliminate(&mut self, i : Index, val : Value) {
        if self.is_candidate(i, val) {
            match self.candidates.get_mut(&i) {
                None => (),
                Some(possible) => {   
                    
                    possible.remove(&val);

                    if possible.len() == 1 {
                        self.empty -= 1;

                        self.stack.push_front(Progress {
                            kind: ProgressType::ELIMINATE,
                            index: i,
                            value: val
                        });
                    } else {
                        self.stack.push_front(Progress {
                            kind: ProgressType::FILL,
                            index: i,
                            value: val
                        });
                    }
                }
            }
        }
    }

    pub fn fill(&mut self, i : Index, val : Value) {

        if self.view(i) == 0 {
            self.empty -= 1;
    
            match self.candidates.get_mut(&i) {
                None => (),
                Some(possibilities) => {
                    possibilities.clear();
                    possibilities.insert(val);

                    self.stack.push_front(Progress {
                        kind: ProgressType::FILL,
                        index: i,
                        value: val
                    });
                }
            }
        }
    }

    pub fn to_string(&self) -> String {
        let mut string = String::new();

        for i in 0..VOLUME {
            let val : Value = self.view(i as Index);

            string.push_str(val.to_string().as_str());
        }

        return string;
    }

    pub fn complete(&self) -> bool {
        return self.empty == 0;
    }

    pub fn progress(&self) -> usize {
        return self.stack.len();
    }
}

pub fn candidate_set() -> HashSet<Value> {
    let mut set : HashSet<Value> = HashSet::new();

    for i in 0..SIZE {
        set.insert(i + 1);
    }

    return set;
}

pub fn format_string(source : &str) -> String {
    let mut string = String::new();

    for i in 0..SIZE {
        string.push_str(&source[((SIZE * i) as usize)..((SIZE * (i + 1)) as usize)]);
        string.push('\n');
    }

    return string;
}