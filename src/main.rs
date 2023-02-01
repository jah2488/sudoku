use rand::prelude::*;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};

#[derive(Clone, Eq, PartialEq, Hash)]
struct Point {
    x: u8,
    y: u8,
}

impl Point {
    fn new(x: u8, y: u8) -> Point {
        return Point { x, y };
    }
}

impl Debug for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[derive(Clone)]
struct Cell {
    x: u8,
    y: u8,
    value: u8,
    neighbors: HashSet<Point>,
}

impl Cell {
    fn is_valid(&self, graph: &Graph) -> bool {
        let mut valid = true;
        for point in &self.neighbors {
            if graph.at(point.x, point.y).value == self.value {
                valid = false;
                break;
            }
        }
        return valid;
    }
}

impl Debug for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "Cell {{ x: {}, y: {}, value: {}, neighbors: {:?} }}",
            self.x, self.y, self.value, self.neighbors
        )
    }
}

#[derive(Clone)]
struct Graph {
    cells: Vec<Cell>,
}

impl Graph {
    fn new() -> Graph {
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

    fn at(&self, x: u8, y: u8) -> Cell {
        let c = self.cells.iter().find(|c| c.x == x && c.y == y);
        if let Some(c) = c {
            return c.clone();
        }
        return Cell {
            x,
            y,
            value: 0,
            neighbors: HashSet::new(),
        };
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
            (1..=3, 1..=3) => neighbors.extend(Graph::box_set(1, 3).into_iter().map(|p| p.clone())),
            (3..=6, 0..=3) => neighbors.extend(Graph::box_set(1, 3).into_iter().map(|p| p.clone())),
            (6..=9, 0..=3) => neighbors.extend(Graph::box_set(1, 3).into_iter().map(|p| p.clone())),

            (0..=3, 3..=6) => neighbors.extend(Graph::box_set(1, 3).into_iter().map(|p| p.clone())),
            (3..=6, 3..=6) => neighbors.extend(Graph::box_set(1, 3).into_iter().map(|p| p.clone())),
            (6..=9, 3..=6) => neighbors.extend(Graph::box_set(1, 3).into_iter().map(|p| p.clone())),

            (0..=3, 6..=9) => neighbors.extend(Graph::box_set(1, 3).into_iter().map(|p| p.clone())),
            (3..=6, 6..=9) => neighbors.extend(Graph::box_set(1, 3).into_iter().map(|p| p.clone())),
            (6..=9, 6..=9) => neighbors.extend(Graph::box_set(1, 3).into_iter().map(|p| p.clone())),
            _ => {
                eprintln!("Incorrect X,Y range supplied!");
            }
        }
        return neighbors;
    }

    fn box_set(min: u8, max: u8) -> HashSet<Point> {
        let mut points: HashSet<Point> = HashSet::new();
        (min..=max).for_each(|x| {
            (min..=max).for_each(|y| {
                points.insert(Point::new(x, y));
            });
        });
        println!("{:?}", points);
        return points;
    }

    pub fn generate(&mut self) {
        let graph_copy = self.clone();
        let mut rng = rand::thread_rng();
        let mut nums: Vec<u8> = (1..9).collect();
        nums.shuffle(&mut rng);

        let mut i = 0;
        while i < 81 {
            nums.shuffle(&mut rng);
            let n = nums.get(1).unwrap().clone();
            let mut c = self.cells.get_mut(i).unwrap();
            c.value = n;
            if c.is_valid(&graph_copy) {
                i += 1;
            } else {
                c.value = 0;
            }
        }
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

fn main() {
    println!("Welcome to Sudoku!");
    let mut g = Graph::new();
    g.generate();
    println!("{:?}", g);
    println!("Cells in graph: {}", Graph::new().cells.len());
    println!("{:?}", g.cells.get(1).unwrap());
}
