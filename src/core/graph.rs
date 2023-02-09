use rand::prelude::*;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};

use crate::core::cell::{Cell, Point};
use crate::core::value::{from_val, options, to_val, Value};

#[derive(Clone)]
pub struct Graph {
    pub cells: Vec<Cell>,
}

impl Graph {
    pub fn new() -> Graph {
        let mut g = Graph { cells: Vec::new() };
        (1..=9).for_each(|x| {
            (1..=9).for_each(|y| {
                let c = Cell {
                    x,
                    y,
                    value: 0,
                    neighbors: HashSet::new(),
                };
                g.cells.push(c);
            });
        });

        let g_copy = g.clone();
        g.cells.iter_mut().for_each(|cell: &mut Cell| {
            cell.neighbors = g_copy.neighbors_for(cell);
        });
        return g;
    }

    pub fn at(&self, x: u8, y: u8) -> Option<&Cell> {
        let cell = self.cells.iter().find(|c| c.x == x && c.y == y);
        return cell;
    }

    pub fn index(&mut self, index: u8) -> Option<&mut Cell> {
        let x = (index - 1) % 9 + 1;
        let y = (index - 1) / 9 + 1;

        let cell = self.cells.iter_mut().find(|c| c.x == x && c.y == y);
        return cell;
    }

    fn neighbors_for(&self, cell: &Cell) -> HashSet<Point> {
        let mut neighbors: HashSet<Point> = HashSet::new();
        let x = cell.x;
        let y = cell.y;

        // Row
        (1..=9).for_each(|i| {
            if i != y {
                neighbors.insert(Point { x, y: i });
            }
            if i != x {
                neighbors.insert(Point { x: i, y });
            }
        });

        // y is the row (vertical)
        // x is the column (horizontal)
        match (x, y) {
            (1..=3, 1..=3) => {
                neighbors.extend(Graph::box_set(1, 3, 1, 3).into_iter().map(|p| p.clone()))
            }

            (3..=6, 1..=3) => {
                neighbors.extend(Graph::box_set(4, 6, 1, 3).into_iter().map(|p| p.clone()))
            }

            (6..=9, 1..=3) => {
                neighbors.extend(Graph::box_set(7, 9, 1, 3).into_iter().map(|p| p.clone()))
            }

            #[allow(overlapping_range_endpoints)]
            (1..=3, 3..=6) => {
                neighbors.extend(Graph::box_set(1, 3, 4, 6).into_iter().map(|p| p.clone()))
            }

            #[allow(overlapping_range_endpoints)]
            (3..=6, 3..=6) => {
                neighbors.extend(Graph::box_set(4, 6, 4, 6).into_iter().map(|p| p.clone()))
            }

            #[allow(overlapping_range_endpoints)]
            (6..=9, 3..=6) => {
                neighbors.extend(Graph::box_set(7, 9, 4, 6).into_iter().map(|p| p.clone()))
            }

            #[allow(overlapping_range_endpoints)]
            (1..=3, 6..=9) => {
                neighbors.extend(Graph::box_set(1, 3, 7, 9).into_iter().map(|p| p.clone()))
            }

            #[allow(overlapping_range_endpoints)]
            (3..=6, 6..=9) => {
                neighbors.extend(Graph::box_set(4, 6, 7, 9).into_iter().map(|p| p.clone()))
            }

            #[allow(overlapping_range_endpoints)]
            (6..=9, 6..=9) => {
                neighbors.extend(Graph::box_set(7, 9, 7, 9).into_iter().map(|p| p.clone()))
            }
            _ => {
                eprintln!("Incorrect X,Y range supplied!");
            }
        }
        return neighbors;
    }

    fn box_set(min_x: u8, max_x: u8, min_y: u8, max_y: u8) -> HashSet<Point> {
        let mut points: HashSet<Point> = HashSet::new();
        (min_x..=max_x).for_each(|x| {
            (min_y..=max_y).for_each(|y| {
                points.insert(Point::new(x, y));
            });
        });
        return points;
    }

    pub fn generate(&mut self) {
        let max_depth = 1_000_000;
        let mut depth = 0;
        let mut rng = rand::thread_rng();
        let mut graph: Graph;
        let mut i = 0;
        let mut invalid_moves: HashMap<usize, Vec<Value>> = HashMap::new();

        while i < 81 {
            graph = self.clone();
            let vals: HashSet<Value> = HashSet::new();

            // At the start of each loop, we need to reset the values of all cells after the current one
            let mut xi = i;
            while xi < 81 {
                let mut cell = self.cells.get_mut(xi).unwrap();
                cell.value = 0;
                xi += 1;
                match invalid_moves.get(&xi) {
                    Some(_) => {
                        invalid_moves.insert(xi, vec![]);
                    }
                    None => {}
                }
            }

            // Grab all possible valid moves for the current cell, and shuffle them, then pick the first one
            let mut cell = self.cells.get_mut(i).unwrap();
            let set = graph.possible_values(cell, vals);
            let mut v: Vec<Value> = Vec::new();
            for n in set {
                match invalid_moves.get(&(i)) {
                    Some(moves) => {
                        if !moves.contains(&n) {
                            v.push(n);
                        }
                    }
                    None => {
                        v.push(n);
                        invalid_moves.insert(i, vec![]);
                    }
                }
            }
            v.shuffle(&mut rng);
            let choice = v.get(0).cloned();

            match choice {
                Some(num) => {
                    cell.value = from_val(num);
                }
                None => {
                    let last_cell = self.cells.get(i - 1).unwrap();
                    // If we have no valid moves, we need to backtrack, and
                    // add the current cell's value to the list of invalid moves for the previous cell
                    match invalid_moves.get(&(i - 1)) {
                        Some(moves) => {
                            let mut moves = moves.clone();
                            if i > 1 {
                                match to_val(last_cell.value) {
                                    Value::Unknown => {}
                                    n => {
                                        moves.push(n);
                                    }
                                }
                                invalid_moves.insert(i - 1, moves);
                            }
                        }
                        None => match to_val(last_cell.value) {
                            Value::Unknown => {}
                            n => {
                                invalid_moves.insert(i - 1, vec![n]);
                            }
                        },
                    }

                    i -= 1;

                    depth += 1;

                    if depth >= max_depth {
                        break;
                    } else {
                        continue;
                    }
                }
            }

            if cell.is_valid(&graph) {
                i += 1;
                //let ten_millis = Duration::from_millis(20);
                //thread::sleep(ten_millis);
                //print!("{}[2J", 27 as char);
                //println!("{}=>{:?}\n", i, invalid_moves.get(&(i)));
                //println!("{:?}", self);
                //invalid_moves.clone().into_iter().for_each(|(k, v)| {
                //    println!("{}: {:?}", k, v);
                //});
            } else {
                cell.value = 0;
                i = 1;
            }
        }
    }

    fn possible_values(&mut self, cell: &Cell, mut vals: HashSet<Value>) -> HashSet<Value> {
        for n in &cell.neighbors {
            vals.insert(to_val(self.at(n.x, n.y).unwrap().value));
        }
        return options().difference(&vals).cloned().collect();
    }

    pub fn invalid_cells(&self) -> Vec<Cell> {
        let mut invalid: Vec<Cell> = Vec::new();
        for cell in &self.cells {
            if cell.value == 0 {
                continue;
            }
            if !cell.is_valid(self) {
                invalid.push(cell.clone());
            }
        }
        return invalid;
    }

    pub fn make_puzzle(remaining_clues: u8) -> Graph {
        let mut graph = Graph::new();
        graph.generate();
        let mut rng = rand::thread_rng();
        let mut i = 81;
        while i > remaining_clues {
            let idx = rng.gen_range(0..81);
            let cell = graph.cells.get_mut(idx).unwrap();
            if cell.value != 0 {
                cell.value = 0;
                i -= 1;
            }
        }
        return graph;
    }
}

//□  ■  ▲  ▼  ◆  ◇  ●  ○  ★  ☆
fn double_row_top() -> String {
    return String::from("╔═══╤═══╤═══╦═══╤═══╤═══╦═══╤═══╤═══╗");
}

fn double_row_bottom() -> String {
    return String::from("╚═══╧═══╧═══╩═══╧═══╧═══╩═══╧═══╧═══╝");
}

fn double_row_middle() -> String {
    return String::from("╠═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╣");
}

fn single_row_middle() -> String {
    return String::from("╟───┼───┼───╫───┼───┼───╫───┼───┼───╢");
}

impl Debug for Graph {
    //    "\033[1m#{self}\033[0m" // bold
    //    "\033[#{direction == :up ? 1 : 2}J" // clear screen

    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut s = String::new();
        s.push_str(&format!("{}", double_row_top()));
        for i in 0..9 {
            if i == 3 || i == 6 {
                s.push_str(&format!("\n{}\n║", double_row_middle()));
            } else {
                if i > 0 {
                    s.push_str(&format!("\n{}\n║", single_row_middle()));
                } else {
                    s.push_str(&format!("\n║"));
                }
            }
            for j in 0..9 {
                let c = self.cells.iter().find(|c| c.x - 1 == i && c.y - 1 == j);
                if let Some(c) = c {
                    if j == 2 || j == 5 || j == 8 {
                        s.push_str(&format!(" {} ║", c.value));
                    } else {
                        s.push_str(&format!(" {} │", c.value));
                    }
                }
            }
            s.push_str(&format!(""));
        }
        s.push_str(&format!("\n{}\n", double_row_bottom()));
        write!(f, "{}", s)
    }
}
