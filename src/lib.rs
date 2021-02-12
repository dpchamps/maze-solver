use serde::{Deserialize, Serialize};
use std::fs;
pub mod maze;

#[derive(Serialize, Deserialize, Debug)]
pub struct MazeIndex {
    pub row: i32,
    pub col: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Maze {
    pub start: MazeIndex,
    pub end: MazeIndex,
    pub rowsAndColumns: Vec<Vec<i32>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Output {
    pub results: Vec<MazeIndex>,
}

impl Output {
    pub fn from_index((row, col): &(i32, i32)) -> MazeIndex {
        MazeIndex {
            row: *row,
            col: *col,
        }
    }
}

pub fn load_maze(resource: &str) -> Maze {
    let path = format!("resources/{}", resource);
    if let Ok(file) = fs::read_to_string(path) {
        serde_json::from_str(&file).unwrap()
    } else {
        panic!(format!("Could not find resource {}", resource));
    }
}

pub fn output_result(output: &Output, resource: &str) {
    let path = format!("resources/{}", resource);
    fs::write(path, serde_json::to_string(output).unwrap()).unwrap();
}
