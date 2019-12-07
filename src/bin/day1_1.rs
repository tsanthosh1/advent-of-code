use std::str::Lines;

pub fn get_fuel_requirement(lines: Lines) -> i32{
    let mut total_requirement = 0;
    for module_mass in lines {
        let one_third_of_mass = (module_mass.parse::<i32>().unwrap() / 3) as f32;
        total_requirement = (one_third_of_mass.floor() as i32 - 2) + total_requirement;
    }
    return total_requirement;
}

pub fn main() {

    let contents = include_str!("../../data/one.data");
    let module_masses = contents.lines();

    println!("Total fuel required: {}", get_fuel_requirement(module_masses))
}


#[cfg(test)]
mod tests {
    use crate::{get_fuel_requirement};

    #[test]
    fn it_works() {
        assert_eq!(get_fuel_requirement("12".lines()), 2);
        assert_eq!(get_fuel_requirement("14".lines()), 2);
        assert_eq!(get_fuel_requirement("1969".lines()), 654);
        assert_eq!(get_fuel_requirement("100756".lines()), 33583);
    }
}