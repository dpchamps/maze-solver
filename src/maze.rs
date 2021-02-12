use crate::MazeIndex;
use std::cmp::Ordering;
use std::ops::Deref;

const CARDINAL_DIRECTIONS: [(i32, i32, i32); 4] = [(1, -1, 0), (2, 0, 1), (4, 1, 0), (8, 0, -1)];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MazeCell {
    pub h_cost: i32,
    pub g_cost: i32,
    pub index: (i32, i32),
    pub parent: Box<Option<MazeCell>>,
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

    pub fn distance_fn((ax, ay): (i32, i32), (bx, by): (i32, i32)) -> i32 {
        (bx - ax).abs() + (by - ay).abs()
    }

    pub fn set_h_cost(&mut self, a: (i32, i32), b: (i32, i32)) {
        // manhattan dist works fine for cardinal directions
        self.h_cost = MazeCell::distance_fn(a, b);
    }

    pub fn cost(&self) -> i32 {
        self.h_cost + self.g_cost
    }

    pub fn as_usize_index(&self) -> (usize, usize) {
        (self.index.0 as usize, self.index.1 as usize)
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

pub fn index_to_tuple(idx: &MazeIndex) -> (i32, i32) {
    (idx.row, idx.col)
}

pub fn is_in_bounds((y, x): (i32, i32), (height, width): (i32, i32)) -> bool {
    x >= 0 && y >= 0 && x < width && y < height
}

pub fn get_neighbors((y, x): (usize, usize), maze: &[Vec<i32>]) -> Vec<MazeCell> {
    let current = maze[y][x];

    CARDINAL_DIRECTIONS
        .iter()
        .filter(|&&(bitmask, _, _)| (current & bitmask) != 0)
        .map(|&(_, next_y, next_x)| MazeCell::from_index((y as i32 + next_y, x as i32 + next_x)))
        .filter(|cell| is_in_bounds(cell.index, (maze.len() as i32, maze[0].len() as i32)))
        .collect()
}

pub fn is_passable() -> bool {

}
