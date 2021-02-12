use maze_solver::maze::{get_neighbors, index_to_tuple, is_in_bounds, MazeCell};
use maze_solver::{load_maze, output_result, Maze, Output};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

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
