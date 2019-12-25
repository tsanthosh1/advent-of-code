use std::str::Lines;
use intcode::IntCode;
use std::collections::HashMap;
use std::io::{stdout, Write};
use itertools::Itertools;
use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};
use crate::PositionStatus::{Wall, NotVisited, Visited, LocationOfOxygenSystem, Home};
use queues::*;
use std::hash::Hash;
use std::f32::MAX;

static NORTH: i64 = 1;
static SOUTH: i64 = 2;
static WEST: i64 = 3;
static EAST: i64 = 4;

type Position = (i64, i64);

#[derive(Copy, Clone, PartialEq)]
enum PositionStatus {
    Wall,
    Home,
    Visited,
    LocationOfOxygenSystem,
    NotVisited,
}

struct MazeBlock {
    position: Position,
    status: PositionStatus,
    last_visited_from: Option<Position>,
    last_visited_from_direction: Option<i64>,
}

impl MazeBlock {
    fn from(position: Position,
            status: PositionStatus,
            last_visited_from: Option<Position>,
            last_visited_from_direction: Option<i64>) -> MazeBlock {
        MazeBlock {
            position,
            status,
            last_visited_from,
            last_visited_from_direction,
        }
    }
}

pub fn main() {
    let contents = include_str!("../../data/fifteen.data");
    let maze = grid(contents);

    let start = (0, 0);
    let end = get_oxygen_cylinder_location(&maze);
    println!("Shortest path {}", bfs_shortest_path(&maze, start, end));
}

fn bfs_shortest_path(maze: &HashMap<(i64, i64), MazeBlock>, start: (i64, i64), end: (i64, i64)) -> i64 {
    let mut queue: Queue<Position> = queue![];
    let mut visited_vertices: HashMap<Position, bool> = HashMap::new();
    let mut distance_from_source: HashMap<Position, i64> = HashMap::new();

    let adjacency_list = create_adjacency_list(&maze);

    adjacency_list.keys()
        .for_each(|key| {
            visited_vertices.insert(*key, false);
            distance_from_source.insert(*key, MAX as i64);
        });

    visited_vertices.insert(start, true);
    distance_from_source.insert(start, 0);
    queue.add(start);

    while queue.peek().is_ok() {
        let next_vertex = queue.remove().ok().unwrap();
        let neighbours = adjacency_list.get(&next_vertex).unwrap();
        for neighbour in neighbours {
            if !visited_vertices.get(&neighbour).unwrap() {
                visited_vertices.insert(*neighbour, true);
                let distance = distance_from_source.get(&next_vertex).unwrap() + 1;
                distance_from_source.insert(
                    *neighbour,
                    distance);
                queue.add(*neighbour);
                if *neighbour == end {
                    return distance;
                }
            }
        }
    };
    unreachable!()
}

fn create_adjacency_list(
    maze: &&HashMap<(i64, i64), MazeBlock>
) -> HashMap<(i64, i64), Vec<(i64, i64)>> {
    maze.keys()
        .filter(|x| is_visitable_position(maze.get(&x).unwrap().status))
        .map(|x| {
            let possible_movements = get_possible_movements(*x);
            let neighbours = possible_movements.iter()
                .filter(|&&(position, _)|
                    is_visitable_position(maze.get(&position).unwrap().status))
                .map(|&x| x.0)
                .collect_vec();
            (*x, neighbours)
        }).into_iter().collect::<HashMap<Position, Vec<Position>>>()
}

fn is_visitable_position(x: PositionStatus) -> bool {
    vec![Visited, Home, LocationOfOxygenSystem].contains(&x)
}

fn get_oxygen_cylinder_location(
    maze: &HashMap<(i64, i64), MazeBlock>
) -> (i64, i64) {
    *maze.keys()
        .find(|x| maze.get(x).unwrap().status == LocationOfOxygenSystem)
        .unwrap()
}

fn grid(contents: &str) -> HashMap<Position, MazeBlock> {
    let mut intcode = IntCode::initialize(contents, None, true);
    let mut maze: HashMap<Position, MazeBlock> = HashMap::new();
    let mut current_position = (0, 0);
    maze.insert(current_position, MazeBlock::from(current_position, Home, None, None));

    loop {
        let next_position_option = get_next_position(current_position, &mut maze);
        if next_position_option.is_none() { break; }

        let (next_position, direction, going_backwards) =
            next_position_option.unwrap();

        intcode.set_input(direction);
        intcode.execute();

        let output = intcode.take_output()[0];
        match output {
            0 => { maze.insert(next_position, MazeBlock::from(next_position, Wall, None, None)); }
            1 => {
                if !going_backwards {
                    maze.insert(next_position, MazeBlock::from(
                        next_position,
                        Visited,
                        Some(current_position),
                        Some(direction)
                    ));
                }
                current_position = next_position;
            }
            2 => {
                if !going_backwards {
                    maze.insert(next_position, MazeBlock::from(
                        next_position,
                        LocationOfOxygenSystem,
                        Some(current_position),
                        Some(direction)
                    ));
                }
                current_position = next_position;
            }
            _ => {}
        }
    }
    print_track(&maze);
    maze
}

fn print_track(maze: &HashMap<Position, MazeBlock>) {
    let (min_x, max_x) = maze.keys()
        .map(|x| x.0)
        .minmax().into_option().unwrap();

    let (min_y, max_y) = maze.keys()
        .map(|x| x.1)
        .minmax().into_option().unwrap();


    for i in min_y..max_y {
        print!("|");
        for j in min_x..max_x {
            let default_value = MazeBlock::from((0, 0), NotVisited, None, None);
            let maze_block =
                *maze.get(&(j, i)).get_or_insert(&default_value);

            match maze_block.status {
                Visited => print!("."),
                Home => print!("X"),
                Wall => print!("█"),
                LocationOfOxygenSystem => print!("o"),
                NotVisited => print!("+")
            }
        }
        println!("|");
    }
}

fn get_next_position(
    current_position: Position,
    mut maze: &mut HashMap<Position, MazeBlock>,
) -> Option<(Position, i64, bool)> {
    let mut current_position = current_position;
    loop {
        let possible_movements = get_possible_movements(current_position);

        let next_position_and_direction = possible_movements.iter()
            .find(|&(position, _)| !is_visited(*position, maze));

        //go forward
        if next_position_and_direction.is_some() {
            let (position, direction) = *next_position_and_direction.unwrap();
            return Some((position, direction, false));
        }

        //hit wall. Go backward
        let maze_block =
            maze.get(&current_position).unwrap().clone();

        if maze_block.last_visited_from.is_none() { return None; }

        return Some((maze_block.last_visited_from.unwrap(),
                     get_opposition_direction(maze_block.last_visited_from_direction.unwrap()),
                     true));
    }
    unreachable!()
}

fn get_possible_movements(current_position: (i64, i64)) -> [((i64, i64), i64); 4] {
    [((current_position.0, current_position.1 - 1), NORTH),
        ((current_position.0 + 1, current_position.1), EAST),
        ((current_position.0, current_position.1 + 1), SOUTH),
        ((current_position.0 - 1, current_position.1), WEST)]
}

fn get_opposition_direction(current_direction: i64) -> i64 {
    match current_direction {
        1 => SOUTH,
        2 => NORTH,
        3 => EAST,
        4 => WEST,
        _ => panic!("Invalid direction")
    }
}

fn is_visited(
    position: (i64, i64),
    maze: &HashMap<(i64, i64), MazeBlock>,
) -> bool {
    let default_value = MazeBlock::from(position, NotVisited, None, None);
    let mut maze_block_wrapped = maze.get(&position);
    let maze_block = maze_block_wrapped.get_or_insert(&default_value);
    maze_block.status != NotVisited
}


