use std::str::Lines;
use std::collections::HashMap;
use std::fmt;
use log::debug;
use std::cmp::max;
use num::abs;
use num_traits::Float;
use std::f32::consts::PI;
use num_traits::real::Real;
use std::f32::MAX;
use itertools::Itertools;
use std::cmp::Ordering::Equal;
use math::round;

static DEBUG: bool = true;

type Asteroid = (i32, i32);

#[derive(Debug, Copy, Clone)]
struct AsteroidVisibility {
    visible_asteroid: Asteroid,
    angle_of_view: f32,
    distance: i32,
}

fn get_asteroids(lines: Lines) -> Vec<Asteroid> {
    lines.enumerate()
        .flat_map(|(line_index, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, x)| x == '#')
                .map(move |(char_index, _)| (char_index as i32, line_index as i32))
        }).collect()
}

fn manhattan_distance((x1, y1): Asteroid, (x2, y2): Asteroid) -> i32 {
    ((x1 - x2).abs() + (y1 - y2).abs())
}

fn get_angle((x1, y1): Asteroid, (x2, y2): Asteroid) -> f32 {
    let dy = y2 - y1;
    let dx = x2 - x1;
    (Float::atan2(dy as f32, dx as f32) * 180.0) / PI
}

fn get_visible_asteroids(asteroids: &Vec<Asteroid>, asteroid: Asteroid) -> Vec<AsteroidVisibility> {
    let grouped = &asteroids.iter()
        .filter(|x| **x != asteroid)
        .map(|x| -> AsteroidVisibility {
            AsteroidVisibility {
                visible_asteroid: *x,
                angle_of_view: get_angle(asteroid, *x),
                distance: manhattan_distance(asteroid, *x),
            }
        })
        .sorted_by(|&a, &b|
            a.angle_of_view.partial_cmp(&b.angle_of_view).unwrap_or(Equal))
        .group_by(|x| (&x).angle_of_view);

    let mut visible_asteroids: Vec<AsteroidVisibility> = Vec::new();
    for (abc, group) in grouped.into_iter() {
        visible_asteroids.push(
            group.into_iter()
                .map(|asteroid_visibility| (asteroid_visibility, asteroid_visibility.distance))
                .min_by_key(|pair| pair.1)
                .map(|pair| pair.0)
                .unwrap());
    }
    visible_asteroids
}

fn get_asteroid_monitor(
    asteroid_mapping: &HashMap<Asteroid, Vec<AsteroidVisibility>>)
    -> (&Asteroid, &Vec<AsteroidVisibility>) {
    asteroid_mapping.into_iter()
        .max_by_key(|&x| x.1.len())
        .unwrap()
}

fn map_asteroid_positions(asteroids: &Vec<Asteroid>) -> HashMap<Asteroid, Vec<AsteroidVisibility>> {
    let asteroid_positions_enumerated =
        asteroids
            .into_iter()
            .map(|x| (x, get_visible_asteroids(&asteroids, *x)))
            .fold(HashMap::new(), |mut agg, x| {
                agg.insert(*x.0, x.1);
                agg
            });

    asteroid_positions_enumerated
}

fn destroy_visible_asteroids(asteroids: &mut Vec<Asteroid>) -> Vec<AsteroidVisibility>{
    let asteroid_visibility_mapping =
        map_asteroid_positions(&asteroids);
    let (monitor_asteroid, mut visible_asteroids) =
        get_asteroid_monitor(&asteroid_visibility_mapping);
    let mut destroyed_asteroids: Vec<AsteroidVisibility> = Vec::new();

    loop {
        let visible_asteroids = get_visible_asteroids(asteroids, *monitor_asteroid);
        if visible_asteroids.is_empty() { break }

        let grouped = &visible_asteroids
            .iter()

            .map(|x| (x, unsafe { get_clockwise_quadrant(&&x) }))
            // sort by angle
            .sorted_by(|&a, &b|
                a.0.angle_of_view.partial_cmp(&b.0.angle_of_view).unwrap_or(Equal))
            // sort by quadrant
            .sorted_by(|&a, &b|
                a.1.partial_cmp(&b.1).unwrap_or(Equal))
            .group_by(|&x| x.1);

        for (key, group) in grouped {
            destroyed_asteroids
                .extend(group.map(|x| *x.0)
                .collect::<Vec<AsteroidVisibility>>())
        }

        visible_asteroids.iter().for_each(|x|{
            let index = asteroids.iter()
                .position(|asteroid| *asteroid == (*x).visible_asteroid)
                .unwrap();
            asteroids.remove(index);
        });
    }
    destroyed_asteroids
}

fn get_clockwise_quadrant(x: &&&AsteroidVisibility) -> i32 {
    match round::ceil((x.angle_of_view * 0.01111) as f64, 0) {
        0.0 => 0,
        1.0 => 1,
        2.0 => 2,
        -1.0 => 3,
        _ => unreachable!()
    }
}

pub fn main() {
    let contents = include_str!("../../data/ten.data");
    let lines = contents.lines();
    let mut asteroids = get_asteroids(lines);
    let destroyed_asteroids = destroy_visible_asteroids(&mut asteroids);
    println!("200th asteroid: {:?}", destroyed_asteroids[199].visible_asteroid)
}

#[cfg(test)]
mod tests {
    use crate::{get_angle, get_asteroids, get_visible_asteroids, map_asteroid_positions, get_asteroid_monitor, destroy_visible_asteroids};
    use std::str::Lines;

    #[test]
    fn test_angle() {
        assert_eq!(get_angle((1, 0), (1, 1)), 90.0);
        assert_eq!(get_angle((1, 0), (1, -1)), -90.0);
        assert_eq!(get_angle((11, 14), (11, 13)), -90.0);
        assert_eq!(get_angle((11, 13), (11, 14)), 90.0);
        assert_eq!(get_angle((11, 13), (-11, 13)), 179.99998);
        assert_eq!(get_angle((11, 13), (11, -13)), -90.0);
    }

    #[test]
    fn test_destroyed_order() {
        let mut asteroids = get_asteroids(".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##".lines());
        let asteroid_mapping = map_asteroid_positions(&asteroids);
        let monitor = get_asteroid_monitor(&asteroid_mapping);
        let destroyed_asteroids = destroy_visible_asteroids(&mut asteroids);

        assert_eq!(destroyed_asteroids[0].visible_asteroid, (11, 12));
        assert_eq!(destroyed_asteroids[1].visible_asteroid, (12, 1));
        assert_eq!(destroyed_asteroids[2].visible_asteroid, (12, 2));
        assert_eq!(destroyed_asteroids[9].visible_asteroid, (12, 8));
        assert_eq!(destroyed_asteroids[19].visible_asteroid, (16, 0));
        assert_eq!(destroyed_asteroids[49].visible_asteroid, (16, 9));
        assert_eq!(destroyed_asteroids[99].visible_asteroid, (10, 16));
        assert_eq!(destroyed_asteroids[198].visible_asteroid, (9, 6));
        assert_eq!(destroyed_asteroids[199].visible_asteroid, (8, 2));
        assert_eq!(destroyed_asteroids[200].visible_asteroid, (10, 9));
        assert_eq!(destroyed_asteroids[298].visible_asteroid, (11, 1));
    }

    #[test]
    fn returns_max_number_of_asteroid_visibility() {
        assert_eq!(get_max_visibility(".#..#\n.....\n#####\n....#\n...##".lines()),
                   8);

        assert_eq!(get_max_visibility("......#.#.\n#..#.#....\n..#######.\n.#.#.###..\n.#..#.....\n..#....#.#\n#..#....#.\n.##.#..###\n##...#..#.\n.#....####".lines()),
                   33);

        assert_eq!(get_max_visibility("#.#...#.#.\n.###....#.\n.#....#...\n##.#.#.#.#\n....#.#.#.\n.##..###.#\n..#...##..\n..##....##\n......#...\n.####.###.".lines()),
                   35);

        assert_eq!(get_max_visibility(".#..#..###\n####.###.#\n....###.#.\n..###.##.#\n##.##.#.#.\n....###..#\n..#.#..#.#\n#..#.#.###\n.##...##.#\n.....#.#..".lines()),
                   41);

        assert_eq!(get_max_visibility(".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##".lines()),
                   210);
    }

    fn get_max_visibility(input: Lines) -> usize {
        let asteroids = get_asteroids(input);
        let asteroid_mapping = map_asteroid_positions(&asteroids);
        let (asteroid, visible_asteroids) = get_asteroid_monitor(&asteroid_mapping);
        visible_asteroids.len()
    }
}