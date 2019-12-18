use itertools::Itertools;
use permutator::Permutation;
use std::borrow::{Borrow, BorrowMut};
use num_traits::abs;
use num_traits::real::Real;
use regex::Regex;

type Position = (i64, i64, i64);
type Velocity = (i64, i64, i64);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Moon {
    position: Position,
    velocity: Velocity,
}

pub fn main() {
    let contents = include_str!("../../data/twelve.data");
    let mut moons: Vec<Moon> = parse_input(contents);
//    apply_gravity_and_velocity(&mut moons, 1000);
//    println!("Total energy required {}", get_energy(&moons));

    println!("Total steps required for complete revolution is {}", find_complete_revolution_time(&mut moons))
}

fn parse_input(contents: &str) -> Vec<Moon> {
    let re = Regex::new(r"(-?\d+)").unwrap();
    contents.lines()
        .fold(Vec::new(), |mut moons_aggregatez, line| {
            let numbers = re.captures_iter(line)
                .fold(Vec::new(), |mut number_aggregate, capture| {
                    number_aggregate.push(capture[0].parse::<i64>().unwrap());
                    number_aggregate
                });
            moons_aggregatez.push(Moon {
                position: (numbers[0], numbers[1], numbers[2]),
                velocity: (0, 0, 0),
            });
            moons_aggregatez
        })
}

pub fn apply_gravity(moons: &mut Vec<Moon>) {
    for i in 0..moons.len() {
        moons[i].velocity = moons.iter()
            .fold(moons[i].velocity, |aggregate_difference: Velocity, y| -> Velocity {
                (aggregate_difference.0 + compare(moons[i].position.0, y.position.0),
                 aggregate_difference.1 + compare(moons[i].position.1, y.position.1),
                 aggregate_difference.2 + compare(moons[i].position.2, y.position.2))
            });
    }
}

pub fn apply_velocity(moons: &mut Vec<Moon>) {
    for i in 0..moons.len() {
        moons[i].position =
            (moons[i].position.0 + moons[i].velocity.0,
             moons[i].position.1 + moons[i].velocity.1,
             moons[i].position.2 + moons[i].velocity.2);
    }
}

pub fn apply_gravity_and_velocity(mut moons: &mut Vec<Moon>, times: i64) {
    for i in 0..times {
        apply_gravity(&mut moons);
        apply_velocity(&mut moons);
    }

    pub fn find_complete_revolution_time(mut moons: &mut Vec<Moon>) -> i64 {
        let mut steps_taken_for_x: Option<i64> = None;
        let mut steps_taken_for_y: Option<i64> = None;
        let mut steps_taken_for_z: Option<i64> = None;
        let mut steps_count = 0;
        let initial_moon_position = moons.clone();
        let initial_x_position = get_x_positions(&moons).clone();
        let initial_y_position = get_y_positions(&moons).clone();
        let initial_z_position = get_z_positions(&moons).clone();
        let initial_velocity = vec![0, 0, 0, 0];
        loop {
            apply_gravity(&mut moons);
            apply_velocity(&mut moons);

            steps_count += 1;
            if steps_taken_for_x.is_some() && steps_taken_for_y.is_some() && steps_taken_for_z.is_some() {
                break;
            }
            if steps_taken_for_x.is_none() {
                if initial_x_position == get_x_positions(&moons) &&
                    get_x_velocities(&moons) == initial_velocity {
                    steps_taken_for_x = Some(steps_count);
                }
            }

            if steps_taken_for_y.is_none() {
                if initial_y_position == get_y_positions(&moons) &&
                    get_y_velocities(&moons) == initial_velocity {
                    steps_taken_for_y = Some(steps_count);
                }
            }

            if steps_taken_for_z.is_none() {
                if initial_z_position == get_z_positions(&moons) &&
                    get_z_velocities(&moons) == initial_velocity {
                    steps_taken_for_z = Some(steps_count);
                }
            }
        }

        num::integer::lcm(
            num::integer::lcm(
                steps_taken_for_x.unwrap(),
                steps_taken_for_y.unwrap(),
            ),
            steps_taken_for_z.unwrap(),
        )
    }

    fn get_x_positions(moons: &Vec<Moon>) -> Vec<i64> {
        moons.iter()
            .map(|moon| moon.position.0).collect::<Vec<i64>>()
    }

    fn get_y_positions(moons: &Vec<Moon>) -> Vec<i64> {
        moons.iter()
            .map(|moon| moon.position.1).collect::<Vec<i64>>()
    }

    fn get_z_positions(moons: &Vec<Moon>) -> Vec<i64> {
        moons.iter()
            .map(|moon| moon.position.2).collect::<Vec<i64>>()
    }

    fn get_x_velocities(moons: &Vec<Moon>) -> Vec<i64> {
        moons.iter()
            .map(|moon| moon.velocity.0).collect::<Vec<i64>>()
    }

    fn get_y_velocities(moons: &Vec<Moon>) -> Vec<i64> {
        moons.iter()
            .map(|moon| moon.velocity.1).collect::<Vec<i64>>()
    }

    fn get_z_velocities(moons: &Vec<Moon>) -> Vec<i64> {
        moons.iter()
            .map(|moon| moon.velocity.2).collect::<Vec<i64>>()
    }

    pub fn compare(a: i64, b: i64) -> i64 {
        if a < b { 1 } else if b < a { -1 } else { 0 }
    }

    pub fn get_energy(moons: &Vec<Moon>) -> i64 {
        moons.iter()
            .map(|x| (abs(x.position.0) + abs(x.position.1) + abs(x.position.2)) *
                (abs(x.velocity.0) + abs(x.velocity.1) + abs(x.velocity.2)))
            .sum()
    }

    #[cfg(test)]
    mod tests {
        use crate::{apply_gravity, Moon, apply_velocity, apply_gravity_and_velocity, get_energy, find_complete_revolution_time};

        fn equals(expected: &Vec<Moon>, actual: &Vec<Moon>) -> bool {
            expected.eq(actual)
        }

        #[test]
        fn calculates_velocity_and_gravity() {
            let mut moons = vec![
                Moon { position: (-1, 0, 2), velocity: (0, 0, 0) },
                Moon { position: (2, -10, -7), velocity: (0, 0, 0) },
                Moon { position: (4, -8, 8), velocity: (0, 0, 0) },
                Moon { position: (3, 5, -1), velocity: (0, 0, 0) }];
            apply_gravity_and_velocity(&mut moons, 1);

            assert!(equals(&moons, &vec![
                Moon { position: (2, -1, 1), velocity: (3, -1, -1) },
                Moon { position: (3, -7, -4), velocity: (1, 3, 3) },
                Moon { position: (1, -7, 5), velocity: (-3, 1, -3) },
                Moon { position: (2, 2, 0), velocity: (-1, -3, 1) }]));

            apply_gravity_and_velocity(&mut moons, 1);
            assert!(equals(&moons, &vec![
                Moon { position: (5, -3, -1), velocity: (3, -2, -2) },
                Moon { position: (1, -2, 2), velocity: (-2, 5, 6) },
                Moon { position: (1, -4, -1), velocity: (0, 3, -6) },
                Moon { position: (1, -4, 2), velocity: (-1, -6, 2) }]), true);

            apply_gravity_and_velocity(&mut moons, 1);
            assert!(equals(&moons, &vec![
                Moon { position: (5, -6, -1), velocity: (0, -3, 0) },
                Moon { position: (0, 0, 6), velocity: (-1, 2, 4) },
                Moon { position: (2, 1, -5), velocity: (1, 5, -4) },
                Moon { position: (1, -8, 2), velocity: (0, -4, 0) }]), true);

            apply_gravity_and_velocity(&mut moons, 1);
            assert!(equals(&moons, &vec![
                Moon { position: (2, -8, 0), velocity: (-3, -2, 1) },
                Moon { position: (2, 1, 7), velocity: (2, 1, 1) },
                Moon { position: (2, 3, -6), velocity: (0, 2, -1) },
                Moon { position: (2, -9, 1), velocity: (1, -1, -1) }]), true);

            apply_gravity_and_velocity(&mut moons, 1);
            assert!(equals(&moons, &vec![
                Moon { position: (-1, -9, 2), velocity: (-3, -1, 2) },
                Moon { position: (4, 1, 5), velocity: (2, 0, -2) },
                Moon { position: (2, 2, -4), velocity: (0, -1, 2) },
                Moon { position: (3, -7, -1), velocity: (1, 2, -2) }]), true);

            apply_gravity_and_velocity(&mut moons, 1);
            assert!(equals(&moons, &vec![
                Moon { position: (-1, -7, 3), velocity: (0, 2, 1) },
                Moon { position: (3, 0, 0), velocity: (-1, -1, -5) },
                Moon { position: (3, -2, 1), velocity: (1, -4, 5) },
                Moon { position: (3, -4, -2), velocity: (0, 3, -1) }]), true);

            apply_gravity_and_velocity(&mut moons, 1);
            assert!(equals(&moons, &vec![
                Moon { position: (2, -2, 1), velocity: (3, 5, -2) },
                Moon { position: (1, -4, -4), velocity: (-2, -4, -4) },
                Moon { position: (3, -7, 5), velocity: (0, -5, 4) },
                Moon { position: (2, 0, 0), velocity: (-1, 4, 2) }]), true);

            apply_gravity_and_velocity(&mut moons, 1);
            assert!(equals(&moons, &vec![
                Moon { position: (5, 2, -2), velocity: (3, 4, -3) },
                Moon { position: (2, -7, -5), velocity: (1, -3, -1) },
                Moon { position: (0, -9, 6), velocity: (-3, -2, 1) },
                Moon { position: (1, 1, 3), velocity: (-1, 1, 3) }]), true);

            apply_gravity_and_velocity(&mut moons, 1);
            assert!(equals(&moons, &vec![
                Moon { position: (5, 3, -4), velocity: (0, 1, -2) },
                Moon { position: (2, -9, -3), velocity: (0, -2, 2) },
                Moon { position: (0, -8, 4), velocity: (0, 1, -2) },
                Moon { position: (1, 1, 5), velocity: (0, 0, 2) }]), true);

            apply_gravity_and_velocity(&mut moons, 1);
            assert!(equals(&moons, &vec![
                Moon { position: (2, 1, -3), velocity: (-3, -2, 1) },
                Moon { position: (1, -8, 0), velocity: (-1, 1, 3) },
                Moon { position: (3, -6, 1), velocity: (3, 2, -3) },
                Moon { position: (2, 0, 4), velocity: (1, -1, -1) }]), true);

            assert_eq!(get_energy(&moons), 179)
        }

        #[test]
        fn calculates_velocity_and_gravity_with_large_number_of_iterations() {
            let mut moons = vec![
                Moon { position: (-8, -10, 0), velocity: (0, 0, 0) },
                Moon { position: (5, 5, 10), velocity: (0, 0, 0) },
                Moon { position: (2, -7, 3), velocity: (0, 0, 0) },
                Moon { position: (9, -8, -3), velocity: (0, 0, 0) }];

            apply_gravity_and_velocity(&mut moons, 10);
            assert!(equals(&moons, &vec![
                Moon { position: (-9, -10, 1), velocity: (-2, -2, -1) },
                Moon { position: (4, 10, 9), velocity: (-3, 7, -2) },
                Moon { position: (8, -10, -3), velocity: (5, -1, -2) },
                Moon { position: (5, -10, 3), velocity: (0, -4, 5) }],
            ));

            apply_gravity_and_velocity(&mut moons, 10);
            assert!(equals(&moons, &vec![
                Moon { position: (-10, 3, -4), velocity: (-5, 2, 0) },
                Moon { position: (5, -25, 6), velocity: (1, 1, -4) },
                Moon { position: (13, 1, 1), velocity: (5, -2, 2) },
                Moon { position: (0, 1, 7), velocity: (-1, -1, 2) }],
            ));

            apply_gravity_and_velocity(&mut moons, 10);
            assert!(equals(&moons, &vec![
                Moon { position: (15, -6, -9), velocity: (-5, 4, 0) },
                Moon { position: (-4, -11, 3), velocity: (-3, -10, 0) },
                Moon { position: (0, -1, 11), velocity: (7, 4, 3) },
                Moon { position: (-3, -2, 5), velocity: (1, 2, -3) }],
            ));

            apply_gravity_and_velocity(&mut moons, 10);
            assert!(equals(&moons, &vec![
                Moon { position: (14, -12, -4), velocity: (11, 3, 0) },
                Moon { position: (-1, 18, 8), velocity: (-5, 2, 3) },
                Moon { position: (-5, -14, 8), velocity: (1, -2, 0) },
                Moon { position: (0, -12, -2), velocity: (-7, -3, -3) }],
            ));

            apply_gravity_and_velocity(&mut moons, 10);
            assert!(equals(&moons, &vec![
                Moon { position: (-23, 4, 1), velocity: (-7, -1, 2) },
                Moon { position: (20, -31, 13), velocity: (5, 3, 4) },
                Moon { position: (-4, 6, 1), velocity: (-1, 1, -3) },
                Moon { position: (15, 1, -5), velocity: (3, -3, -3) }],
            ));

            apply_gravity_and_velocity(&mut moons, 10);
            assert!(equals(&moons, &vec![
                Moon { position: (36, -10, 6), velocity: (5, 0, 3) },
                Moon { position: (-18, 10, 9), velocity: (-3, -7, 5) },
                Moon { position: (8, -12, -3), velocity: (-2, 1, -7) },
                Moon { position: (-18, -8, -2), velocity: (0, 6, -1) }],
            ));

            apply_gravity_and_velocity(&mut moons, 10);
            assert!(equals(&moons, &vec![
                Moon { position: (-33, -6, 5), velocity: (-5, -4, 7) },
                Moon { position: (13, -9, 2), velocity: (-2, 11, 3) },
                Moon { position: (11, -8, 2), velocity: (8, -6, -7) },
                Moon { position: (17, 3, 1), velocity: (-1, -1, -3) }],
            ));

            apply_gravity_and_velocity(&mut moons, 10);
            assert!(equals(&moons, &vec![
                Moon { position: (30, -8, 3), velocity: (3, 3, 0) },
                Moon { position: (-2, -4, 0), velocity: (4, -13, 2) },
                Moon { position: (-18, -7, 15), velocity: (-8, 2, -2) },
                Moon { position: (-2, -1, -8), velocity: (1, 8, 0) }],
            ));

            apply_gravity_and_velocity(&mut moons, 10);
            assert!(equals(&moons, &vec![
                Moon { position: (-25, -1, 4), velocity: (1, -3, 4) },
                Moon { position: (2, -9, 0), velocity: (-3, 13, -1) },
                Moon { position: (32, -8, 14), velocity: (5, -4, 6) },
                Moon { position: (-1, -2, -8), velocity: (-3, -6, -9) }],
            ));

            apply_gravity_and_velocity(&mut moons, 10);
            assert!(equals(&moons, &vec![
                Moon { position: (8, -12, -9), velocity: (-7, 3, 0) },
                Moon { position: (13, 16, -3), velocity: (3, -11, -5) },
                Moon { position: (-29, -11, -1), velocity: (-3, 7, 4) },
                Moon { position: (16, -13, 23), velocity: (7, 1, 1) }],
            ));
            assert_eq!(get_energy(&moons), 1940)
        }

        #[test]
        fn calculates_velocity_and_gravity2() {
            let mut moons = vec![
                Moon { position: (-1, 0, 2), velocity: (0, 0, 0) },
                Moon { position: (2, -10, -7), velocity: (0, 0, 0) },
                Moon { position: (4, -8, 8), velocity: (0, 0, 0) },
                Moon { position: (3, 5, -1), velocity: (0, 0, 0) }];
            println!("------------------- Round 0");
            println!("{:?}", &moons[0]);
            println!("{:?}", &moons[1]);
            println!("{:?}", &moons[2]);
            println!("{:?}", &moons[3]);
            find_complete_revolution_time(&mut moons);
        }
    }
}