use std::str::Lines;
use std::collections::HashMap;
use std::fmt;
use log::debug;
use std::cmp::max;

static DEBUG: bool = true;

#[derive(Copy, Clone, Eq, Hash, Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn create(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

pub struct AsteroidPositionVisibilityMapping {
    mapping: HashMap<Point, Vec<Point>>
}

impl AsteroidPositionVisibilityMapping {
    fn add_visibility_mapping(&mut self, from: Point, visible_positions: Vec<Point>) {
        visible_positions.iter().for_each(|visible_position| {
            self.add_visibility_mapping_p2p(from, *visible_position);
            self.add_visibility_mapping_p2p(*visible_position, from);
        });
    }

    fn add_visibility_mapping_p2p(&mut self, point1: Point, point2: Point) {
        if self.mapping.get(&point1).is_none() {
            self.mapping.insert(point1, vec![point2]);
        }

        let already_visible_points = self.mapping.get_mut(&point1).unwrap();
        let point2_already_mapped_to_point1 =
            already_visible_points.iter().find(|x| x.eq(&&point2));
        if point2_already_mapped_to_point1.is_none() {
            already_visible_points.push(point2);
        }
    }

    fn get_asteroid_for_monitoring(&self) -> i32 {
        self.mapping.keys().fold(0, |max_number_of_visible_asteroids, asteroid_position| {
            max(max_number_of_visible_asteroids, self.mapping.get(asteroid_position).unwrap().len() as i32)
        })
    }

    fn print(&self) {
        self.mapping.keys().for_each(|asteroid| {
            let visible_asteroids = self.mapping.get(asteroid).unwrap();
            let visible_asteroids_formatted = to_string(visible_asteroids);
            println!("{} ({})-> {:?}", asteroid, visible_asteroids.len(), visible_asteroids_formatted);
        })
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

pub fn are_three_points_collinear(point1: Point, point2: Point, point3: Point) -> bool {
//    Area = 0.5 * [x1(y2 - y3) + x2(y3 - y1) + x3(y1 - y2)]

    ((point1.x * (point2.y - point3.y))
        + (point2.x * (point3.y - point1.y))
        + (point3.x * (point1.y - point2.y))) as f32 == 0.0
}

pub fn main() {
    let contents = include_str!("../../data/ten.data");
    let lines = contents.lines();
    let all_asteroid_positions = read_asteroid_positions(lines);
    let visible_asteroid_mapping = get_asteroid_mapping(all_asteroid_positions);
    println!("You can monitor {} asteroids", visible_asteroid_mapping.get_asteroid_for_monitoring())
}

fn get_asteroid_mapping(all_asteroid_positions: Vec<Point>) -> AsteroidPositionVisibilityMapping {

    let mut visible_asteroid_mapping: AsteroidPositionVisibilityMapping =
        AsteroidPositionVisibilityMapping { mapping: HashMap::new() };

    let asteroid_positions_enumerated = all_asteroid_positions
        .clone()
        .into_iter()
        .enumerate();

    asteroid_positions_enumerated.for_each(|(index, asteroid_position)| {
        let upcoming_asteroid_positions: Vec<Point> = all_asteroid_positions[index + 1..].to_vec();
        let mut visible_asteroid_positions: Vec<Point> = Vec::new();

        upcoming_asteroid_positions.iter().for_each(|upcoming_asteroid| {
            if visible_asteroid_positions.is_empty() {
                visible_asteroid_positions.push(*upcoming_asteroid);
            } else {
                let asteroids_not_hiding_upcoming_asteroid =
                    get_asteroid_positions_not_hiding_upcoming_asteroid(
                        asteroid_position,
                        &mut visible_asteroid_positions,
                        upcoming_asteroid
                    );
                if asteroids_not_hiding_upcoming_asteroid.len() == visible_asteroid_positions.len() {
                    visible_asteroid_positions.push(*upcoming_asteroid);
                }
            }
        });
        visible_asteroid_mapping.add_visibility_mapping(asteroid_position, visible_asteroid_positions);
    });
    visible_asteroid_mapping.print();
    visible_asteroid_mapping
}

fn get_asteroid_positions_not_hiding_upcoming_asteroid(
    asteroid_position: Point,
    mut visible_asteroid_positions: &mut Vec<Point>,
    upcoming_asteroid: &Point
) -> Vec<Point> {

    let mut asteroids_not_hiding_upcoming_asteroid: Vec<Point> = Vec::new();
    visible_asteroid_positions.iter()
        .for_each(|visible_asteroid_position| {
        if !are_three_points_collinear(asteroid_position, *upcoming_asteroid, *visible_asteroid_position) {
            asteroids_not_hiding_upcoming_asteroid.push(*upcoming_asteroid);
        }
    });
    asteroids_not_hiding_upcoming_asteroid
}

fn to_string(visible_asteroids: &Vec<Point>) -> String {
    visible_asteroids
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn read_asteroid_positions(lines: Lines) -> Vec<Point> {
    lines.enumerate().fold(Vec::new(), |mut asteroid_positions: Vec<Point>, (line_index, line)| {
        let asteroid_positions_this_line =
            line.chars().enumerate().fold(Vec::new(), |mut asteroid_positions_per_line_aggregate, (char_index, char_value)| {
                if char_value == '#' {
                    asteroid_positions_per_line_aggregate.push(Point::create(char_index as i32, line_index as i32))
                }
                asteroid_positions_per_line_aggregate
            });
        asteroid_positions.extend(asteroid_positions_this_line);
        asteroid_positions
    })
}


#[cfg(test)]
mod tests {
    use crate::{are_three_points_collinear, Point, read_asteroid_positions, get_asteroid_mapping, AsteroidPositionVisibilityMapping};
    use std::str::Lines;

    #[test]
    fn test_collinear() {
        assert_eq!(are_three_points_collinear(point(0, 0), point(1, 1), point(2, 2)), true);
        assert_eq!(are_three_points_collinear(point(0, 0), point(1, 4), point(2, 2)), false);
        assert_eq!(are_three_points_collinear(point(1, 0), point(2, 2), point(3, 4)), true);
        assert_eq!(are_three_points_collinear(point(1, 0), point(4, 3), point(3, 2)), false);
    }

    #[test]
    fn returns_max_number_of_asteroid_visibility() {
        let visible_asteroid_mapping = setup(".#..#\n.....\n#####\n....#\n...##".lines());
        assert_eq!(visible_asteroid_mapping.get_asteroid_for_monitoring(), 8);

        let visible_asteroid_mapping = setup("......#.#.\n#..#.#....\n..#######.\n.#.#.###..\n.#..#.....\n..#....#.#\n#..#....#.\n.##.#..###\n##...#..#.\n.#....####".lines());
        assert_eq!(visible_asteroid_mapping.get_asteroid_for_monitoring(), 33);

        let visible_asteroid_mapping = setup("#.#...#.#.\n.###....#.\n.#....#...\n##.#.#.#.#\n....#.#.#.\n.##..###.#\n..#...##..\n..##....##\n......#...\n.####.###.".lines());
        assert_eq!(visible_asteroid_mapping.get_asteroid_for_monitoring(), 35);

        let visible_asteroid_mapping = setup(".#..#..###\n####.###.#\n....###.#.\n..###.##.#\n##.##.#.#.\n....###..#\n..#.#..#.#\n#..#.#.###\n.##...##.#\n.....#.#..".lines());
        assert_eq!(visible_asteroid_mapping.get_asteroid_for_monitoring(), 41);

        let visible_asteroid_mapping = setup(".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##".lines());
        assert_eq!(visible_asteroid_mapping.get_asteroid_for_monitoring(), 210);
    }

    fn setup(input: Lines) -> AsteroidPositionVisibilityMapping {
        let all_asteroid_positions = read_asteroid_positions(input);
        let visible_asteroid_mapping = get_asteroid_mapping(all_asteroid_positions);
        visible_asteroid_mapping
    }

    fn point(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}