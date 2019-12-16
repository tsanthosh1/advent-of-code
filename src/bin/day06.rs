use std::str::Lines;
use std::collections::HashMap;
use std::f32::MAX;
use std::cmp::min;


pub fn get_all_orbits(orbits: &HashMap<String, String>) -> i64 {
    orbits.keys().fold(0, |agg, x| {
        let distance = distance_from_center_of_mass(orbits, String::from(x));
        agg + distance
    })
}

pub fn distance_between_two_object(orbits: &HashMap<String, String>,
                                   space_object1: String,
                                   space_object2: String, ) -> i64 {
    let mut path1: Vec<String> = path_towards_com(orbits, space_object1.as_str());
    let mut path2: Vec<String> = path_towards_com(orbits, space_object2.as_str());
    path1.reverse();
    path2.reverse();

    let mut joining_index = std::usize::MAX;
    for (a, b) in path1.iter().zip(path2.iter()) {
        if a != b {
            joining_index = path2.iter().position(|x| x == b).unwrap();
            break;
        }
    }
    joining_index =
        if joining_index == std::usize::MAX as usize { min(path1.len(), path2.len()) }
        else { joining_index };

    let number_of_orbital_transfer_in_path1 = path1.len() - 1;
    let number_of_orbital_transfer_in_path2 = path2.len() - 1;
    let common_orbital_transfer = joining_index - 1;

    ((number_of_orbital_transfer_in_path1 + number_of_orbital_transfer_in_path2) -
        ((common_orbital_transfer) * 2) - 2) as i64
}

pub fn distance_from_center_of_mass(orbits: &HashMap<String, String>, space_object: String) -> i64 {
    let mut center = orbits.get(space_object.as_str());
    let mut total_distance = 1;
    if center.is_none() {
        if space_object != String::from("COM") {
            panic!("Probably orbit map is wrong. Read {}", space_object)
        } else {
            return 0;
        }
    }

    while center.unwrap() != "COM" {
        total_distance += 1;
        center = orbits.get(center.unwrap());
    }
    total_distance
}

pub fn path_towards_com(orbits: &HashMap<String, String>, space_object: &str) -> Vec<String> {
    let mut path: Vec<String> = Vec::new();
    let mut center = orbits.get(space_object);
    path.push(String::from(space_object));
    if center.is_none() {
        if space_object != String::from("COM") {
            panic!("Probably orbit map is wrong. Read {}", space_object)
        } else {
            return path;
        }
    }

    while center.unwrap() != "COM" {
        path.push(String::from(center.unwrap()));
        center = orbits.get(center.unwrap());
    }
    path.push(String::from(center.unwrap()));
    path
}

pub fn read(maps: Lines) -> HashMap<String, String> {
    let mut all_space_objects: HashMap<String, String> = HashMap::new();

    for map in maps {
        let split: Vec<String> = map.split(')').map(|s| s.to_string()).collect();
        let center = &split[0];
        let orbited_by = &split[1];

        let orbited_by_entry = all_space_objects.get(orbited_by);
        if orbited_by_entry.is_none() {
            all_space_objects.insert(String::from(orbited_by), String::from(center));
        }
    }

    return all_space_objects;
}

pub fn main() {
    let contents = include_str!("../../data/six.data");
    let orbits = read(contents.lines());

    println!("Total number of orbits: {}", get_all_orbits(&orbits));

    println!("Distance between the objects SAN and YOU are orbiting: {}",
             distance_between_two_object(&orbits, String::from("SAN"), String::from("YOU")))
}


#[cfg(test)]
mod tests {
    use crate::{read, distance_from_center_of_mass, get_all_orbits, path_towards_com, distance_between_two_object};

    #[test]
    fn calculates_distance() {
        let map = read("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L".lines());
        let b_distance = distance_from_center_of_mass(&map, String::from("B"));
        let l_distance = distance_from_center_of_mass(&map, String::from("L"));
        let d_distance = distance_from_center_of_mass(&map, String::from("D"));
        assert_eq!(b_distance, 1);
        assert_eq!(l_distance, 7);
        assert_eq!(d_distance, 3);
    }

    #[test]
    fn derives_path() {
        let map = read("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L".lines());
        let b_path: Vec<String> = path_towards_com(&map, String::from("B").as_str());
        let l_path: Vec<String> = path_towards_com(&map, String::from("L").as_str());
        let d_path: Vec<String> = path_towards_com(&map, String::from("D").as_str());
        assert_eq!(b_path.join("-"), "B-COM");
        assert_eq!(l_path.join("-"), "L-K-J-E-D-C-B-COM");
        assert_eq!(d_path.join("-"), "D-C-B-COM");
    }

    #[test]
    fn calculates_total_orbits() {
        let map = read("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L".lines());
        assert_eq!(get_all_orbits(&map), 42);
    }

    #[test]
    fn calculates_distance_between_to_objects() {
        let map = read("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L".lines());
        assert_eq!(distance_between_two_object(&map,
                                               String::from("K"),
                                               String::from("I")), 2);

        assert_eq!(distance_between_two_object(&map,
                                               String::from("H"),
                                               String::from("I")), 3);

        assert_eq!(distance_between_two_object(&map,
                                               String::from("L"),
                                               String::from("B")), 4);
    }
}