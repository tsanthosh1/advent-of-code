use std::str::Lines;

pub fn get_fuel_requirement(module_masses: Lines) -> i32{
    let mut total_requirement = 0;
    for module_mass in module_masses {
        let parsed_module_mass = module_mass.parse::<i32>().unwrap();
        total_requirement = fuel_requirement_for_mass(parsed_module_mass) + total_requirement;
    }
    return total_requirement;
}

fn fuel_requirement_for_mass(module_mass: i32) -> i32{
    let one_third_of_mass = (module_mass / 3) as f32;
    let fuel_required = one_third_of_mass.floor() as i32 - 2;
    if fuel_required > 0 {
        return fuel_requirement_for_mass(fuel_required) + fuel_required
    }
    return 0;
}

pub fn main() {

    let contents = include_str!("../../data/one-part-two.data");
    let module_masses = contents.lines();

    println!("Total fuel required: {}", get_fuel_requirement(module_masses))
}



#[cfg(test)]
mod tests {
    use crate::{get_fuel_requirement};

    #[test]
    fn it_works() {
        assert_eq!(get_fuel_requirement("14".lines()), 2);
        assert_eq!(get_fuel_requirement("1969".lines()), 966);
        assert_eq!(get_fuel_requirement("100756".lines()), 50346);
    }
}