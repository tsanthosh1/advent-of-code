use std::fmt;
use std::f64::MAX;
use std::cmp::min;

#[derive(Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Copy, Clone)]
struct Line {
    point1: Point,
    point2: Point,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.point1.to_string(), self.point2.to_string())
    }
}

pub fn main() {
    let input = include_str!("../../data/three.data");
    let paths: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    let wire1_path = &paths[0];
    let wire2_path = &paths[1];
    let nearest_distance=
        get_nearest_intersection(wire1_path, wire2_path);

    println!("Nearest distance {}", nearest_distance)
}

fn get_nearest_intersection(wire1_path: &str, wire2_path: &str) -> i64 {
    let wire1_lines = path_string_to_lines(wire1_path);
    let wire2_lines = path_string_to_lines(wire2_path);

    println!("Wire 1 path {}", wire1_lines.iter()
        .map(|x| -> String { x.to_string() })
        .collect::<Vec<String>>().join(".."));
    println!("Wire 2 path {}", wire2_lines.iter()
        .map(|x| -> String { x.to_string() })
        .collect::<Vec<String>>().join(".."));

    let mut intersection_points =
        get_intersection_points_of_lines(&wire1_lines, &wire2_lines);

    let first_origin_position = intersection_points.iter()
        .position(|item| item.x == 0 && item.y == 0);
    if first_origin_position.is_some() {
        intersection_points.remove(first_origin_position.unwrap());
    }

    println!("All intersection points {}", intersection_points.iter()
        .map(|x| -> String { x.to_string() })
        .collect::<Vec<String>>().join(".."));

    let mut nearest_distance: i64 = MAX as i64;
    intersection_points.iter().for_each(|item| {
        let distance = distance_between_two_points(&Point { x: 0, y: 0 }, &item);
        nearest_distance = min(distance, nearest_distance);
    });
    nearest_distance
}

fn distance_between_two_points(point1: &Point, point2: &Point) -> i64 {
    return (point1.x - point2.x).abs() + (point1.y - point2.y).abs()
}

fn get_intersection_points_of_lines(wire1_lines: &Vec<Line>,
                                    wire2_lines: &Vec<Line>) -> Vec<Point> {
    let mut intersection_points: Vec<Point> = Vec::new();

    wire1_lines.iter().for_each(|wire1_line| {
        wire2_lines.iter().for_each(|wire2_line| {
            let (is_intersecting, intersection_point) =
                get_intersection(wire1_line, wire2_line);
            if is_intersecting { intersection_points.push(intersection_point.unwrap()) }
        });
    });

    return intersection_points;
}

fn get_intersection(line1: &Line, line2: &Line) -> (bool, Option<Point>) {
    let x1: i64 = line1.point1.x;
    let y1: i64 = line1.point1.y;
    let x2: i64 = line1.point2.x;
    let y2: i64 = line1.point2.y;

    let x3: i64 = line2.point1.x;
    let y3: i64 = line2.point1.y;
    let x4: i64 = line2.point2.x;
    let y4: i64 = line2.point2.y;

    let denominator = ((x2 - x1) * (y4 - y3)) - ((x4 - x3) * (y2 - y1));
    if denominator == 0 {
        return (false, None);
    } else {
        let x_numerator = (((x2 * y1) - (x1 * y2)) * (x4 - x3)) - (((x4 * y3) - (x3 * y4)) * (x2 - x1));
        let y_numerator = (((x2 * y1) - (x1 * y2)) * (y4 - y3)) - (((x4 * y3) - (x3 * y4)) * (y2 - y1));
        let x_intersection: i64 = x_numerator / denominator;
        let y_intersection: i64 = y_numerator / denominator;

        let intersection = Point { x: x_intersection as i64, y: y_intersection as i64 };
        let is_intersection_inside_the_points =
            (distance_between_two_points(&intersection, &line1.point1) +
                distance_between_two_points(&intersection, &line1.point2) ==
                distance_between_two_points(&line1.point1, &line1.point2)) &&
                (distance_between_two_points(&intersection, &line2.point1) +
                    distance_between_two_points(&intersection, &line2.point2) ==
                    distance_between_two_points(&line2.point1, &line2.point2));
        return if is_intersection_inside_the_points {
            (true, Some(intersection))
        } else {
            return (false, None)
        };
    }
}

fn path_string_to_lines(wire_path: &str) -> Vec<Line> {
    let instructions: Vec<String> = wire_path.split(',').map(|s| s.to_string()).collect();
    let mut previous_point: Point = Point { x: 0, y: 0 };

    let lines: Vec<Line> = instructions.into_iter().map(|instruction| {
        let direction: &str = &instruction[..1];
        let units: &i64 = &instruction[1..].parse::<i64>().unwrap();
        let line;
        let next_point: Point;
        match direction {
            "R" => {
                next_point = Point { x: previous_point.x + *units, y: previous_point.y };
                line = Line { point1: previous_point, point2: next_point };
            }
            "L" => {
                next_point = Point { x: previous_point.x - *units, y: previous_point.y };
                line = Line { point1: previous_point, point2: next_point };
            }
            "U" => {
                next_point = Point { x: previous_point.x, y: previous_point.y + *units };
                line = Line { point1: previous_point, point2: next_point };
            }
            "D" => {
                next_point = Point { x: previous_point.x, y: previous_point.y - *units };
                line = Line { point1: previous_point, point2: next_point };
            }
            _ => {
                panic!("Invalid direction {}", direction);
            }
        }
        previous_point = next_point;
        return line;
    }).collect();

    return lines;
}

#[cfg(test)]
mod tests {
    use crate::{path_string_to_lines, get_intersection, Line, Point, get_nearest_intersection};

    #[test]
    fn parses_path_strings_to_lines() {
        let output: String = path_string_to_lines("R75,D30,L83,U83")
            .into_iter()
            .map(|x| -> String { x.to_string() })
            .collect::<Vec<String>>().join("..");
        assert_eq!(output, "(0,0),(75,0)..(75,0),(75,-30)..(75,-30),(-8,-30)..(-8,-30),(-8,53)");
    }

    #[test]
    fn get_intersection_works() {
        let (is_intersecting, intersection) =
            get_intersection(&Line { point1: Point { x: 1, y: 1 }, point2: Point { x: 4, y: 1 } },
                             &Line { point1: Point { x: 2, y: 3 }, point2: Point { x: 2, y: -1 } });
        assert_eq!(is_intersecting, true);
        assert_eq!(intersection.unwrap().to_string(), "(2,1)");

        let (is_intersecting, intersection) =
            get_intersection(&Line { point1: Point { x: 3, y: 5 }, point2: Point { x: 3, y: 2 } },
                             &Line { point1: Point { x: 2, y: 3 }, point2: Point { x: 6, y: 3 } });
        assert_eq!(is_intersecting, true);
        assert_eq!(intersection.unwrap().to_string(), "(3,3)");

        let (is_intersecting, intersection) =
            get_intersection(&Line { point1: Point { x: 0, y: 0 }, point2: Point { x: 8, y: 0 } },
                             &Line { point1: Point { x: 6, y: 7 }, point2: Point { x: 6, y: 3 } });
        assert_eq!(is_intersecting, false);
    }

    #[test]
    fn parallel_lines_should_not_intersect() {
        let (is_intersecting, intersection) =
            get_intersection(&Line { point1: Point { x: 1, y: 1 }, point2: Point { x: 4, y: 1 } },
                             &Line { point1: Point { x: 1, y: 2 }, point2: Point { x: 4, y: 2 } });
        assert_eq!(is_intersecting, false);
    }

    #[test]
    fn should_return_false_if_intersection_happen_outside_the_points() {
        let (is_intersecting, intersection) =
            get_intersection(&Line { point1: Point { x: 1, y: 1 }, point2: Point { x: 3, y: 1 } },
                             &Line { point1: Point { x: 3, y: 3 }, point2: Point { x: 4, y: 2 } });
        assert_eq!(is_intersecting, false);
    }

    #[test]
    fn gets_shortest_distance() {

        assert_eq!(get_nearest_intersection("R75,D30,R83,U83,L12,D49,R71,U7,L72",
                                            "U62,R66,U55,R34,D71,R55,D58,R83"), 159);
        assert_eq!(get_nearest_intersection("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                                            "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"), 135);
        assert_eq!(get_nearest_intersection("R8,U5,L5,D3", "U7,R6,D4,L4"), 6);
    }
}