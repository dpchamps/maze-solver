use maze_solver::{load_maze, output_result, Maze, MazeIndex, Output};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::ops::Deref;

#[derive(Clone, Debug, Eq, PartialEq)]
struct MazeCell {
    h_cost: i32,
    g_cost: i32,
    index: (i32, i32),
    parent: Box<Option<MazeCell>>,
}

impl MazeCell {
    pub fn from_index(index: (i32, i32)) -> Self {
        MazeCell {
            h_cost: 0,
            g_cost: 0,
            index,
            parent: Box::new(None),
        }
    }

    pub fn get_path(&self) -> Vec<(i32, i32)> {
        if let Some(parent_cell) = self.parent.deref() {
            return parent_cell
                .get_path()
                .into_iter()
                .chain(vec![self.index])
                .collect();
        }

        vec![self.index]
    }

    pub fn set_h_cost(&mut self, (ax, ay): (i32, i32), (bx, by): (i32, i32)) {
        // manhattan dist works fine for cardinal directions
        self.h_cost = (bx - ax).abs() + (by - ay).abs()
    }

    pub fn cost(&self) -> i32 {
        self.h_cost + self.g_cost
    }
}

impl Ord for MazeCell {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost().cmp(&self.cost())
    }
}

impl PartialOrd for MazeCell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const CARDINAL_DIRECTIONS: [(i32, i32, i32); 4] = [(1, -1, 0), (2, 0, 1), (4, 1, 0), (8, 0, -1)];

fn index_to_tuple(idx: &MazeIndex) -> (i32, i32) {
    (idx.row, idx.col)
}

fn is_in_bounds((y, x): (i32, i32), (height, width): (i32, i32)) -> bool {
    x >= 0 && y >= 0 && x < width && y < height
}

fn get_neighbors((y, x): (usize, usize), maze: &[Vec<i32>]) -> Vec<MazeCell> {
    let current = maze[y][x];

    CARDINAL_DIRECTIONS
        .iter()
        .filter(|&&(bitmask, _, _)| (current & bitmask) != 0)
        .map(|&(_, next_y, next_x)| MazeCell::from_index((y as i32 + next_y, x as i32 + next_x)))
        .filter(|cell| is_in_bounds(cell.index, (maze.len() as i32, maze[0].len() as i32)))
        .collect()
}

fn get_path(maze: &Maze) -> Option<Vec<(i32, i32)>> {
    let mut open = BinaryHeap::new();
    let mut closed: HashSet<(i32, i32)> = HashSet::new();
    let mut marked: HashSet<(i32, i32)> = HashSet::new();
    let destination = index_to_tuple(&maze.end);

    let mut start_cell = MazeCell::from_index(index_to_tuple(&maze.start));

    start_cell.set_h_cost(start_cell.index, destination);
    open.push(start_cell);

    while !open.is_empty() {
        let next = open.pop()?;

        closed.insert(next.index);

        if next.index == destination {
            return Some(next.get_path());
        }

        get_neighbors(
            (next.index.0 as usize, next.index.1 as usize),
            &maze.rowsAndColumns,
        )
        .iter_mut()
        .for_each(|neighbor| {
            if closed.contains(&neighbor.index) || marked.contains(&neighbor.index) {
                return;
            }

            neighbor.g_cost = 1 + next.g_cost;
            neighbor.set_h_cost(neighbor.index, destination);
            neighbor.parent = Box::new(Some(next.clone()));

            open.push((*neighbor).clone());
            marked.insert(neighbor.index);
        })
    }

    None
}

fn main() {
    let maze = load_maze("input.json");
    let path = get_path(&maze)
        .unwrap()
        .iter()
        .map(|x| Output::from_index(x))
        .collect();
    let result = Output { results: path };

    output_result(&result, "a-star.json");
}
