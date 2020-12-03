use std::str::Lines;
use intcode::IntCode;
use regex::Regex;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::process::exit;
use math::round::ceil;
use num_traits::real::Real;

static TRACE: bool = true;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Chemical {
    name: String,
    quantity: i64,
}

impl Chemical {
    fn from(string: &str) -> Chemical {
        let matches = Regex::new(r"(\d+) ([A-Z]+)")
            .unwrap()
            .captures(string)
            .unwrap();

        let quantity = matches.get(1)
            .unwrap()
            .as_str()
            .trim()
            .parse::<i64>()
            .unwrap();

        let name = matches.get(2)
            .unwrap()
            .as_str()
            .to_string();

        Chemical { name, quantity }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Reaction {
    inputs: Vec<Chemical>,
    output: Chemical,
}

fn read_input(contents: &str) -> Vec<Reaction> {
    contents.lines()
        .into_iter()
        .map(|line| {
            let split = line.split("=>")
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            let inputs = split[0].split(",")
                .map(|s| s.to_string())
                .map(|s| Chemical::from(&s))
                .collect::<Vec<Chemical>>();
            let output = Chemical::from(&split[1]);
            Reaction { inputs, output }
        }).collect::<Vec<Reaction>>()
}

fn get_ore_requirement(
    chemical_name: String,
    units_required: i64,
    reactions: &Vec<Reaction>,
    mut left_over: &mut HashMap<String, i64>
) -> i64 {

    let reaction = &reactions.iter()
        .find(|r| r.output.name.eq(&chemical_name))
        .unwrap();

    let existing_leftover = left_over.get(chemical_name.as_str());
    let mut units_required_updated = units_required;

    if existing_leftover.is_some() {
        let leftover_value = existing_leftover.unwrap();
        units_required_updated = units_required - leftover_value;

        if units_required_updated <= 0 {
            left_over.insert(chemical_name, leftover_value - units_required);
            return 0;
        }
    }

    let multiplier = (units_required_updated as f32 / reaction.output.quantity as f32).ceil() as i64;
    left_over.insert(chemical_name, (reaction.output.quantity * multiplier) - units_required_updated);

    let ores_required = reaction.inputs.iter()
        .map(|input| {
            if input.name == String::from("ORE") {
                input.quantity * multiplier
            } else {
                return get_ore_requirement(
                    String::from(&input.name),
                    input.quantity * multiplier,
                    &reactions,
                    &mut left_over
                );
            }
        }).sum();

    ores_required
}

fn produce_from_trillion_units_of_ore(reactions: &Vec<Reaction>) -> i64 {

    let available_ore = 1000000000000;
    let mut left = 0;
    let mut right = available_ore;
    let mut mid = available_ore / 2;
    let mut visited_points: HashSet<i64> = HashSet::new();

    loop {
        let ore_used = get_ore_requirement(String::from("FUEL"), mid, &reactions, &mut HashMap::new());
        if ore_used < available_ore && visited_points.contains(&mid) {
            return mid;
        }
        visited_points.insert(mid);

        if ore_used < available_ore {
            //go right
            left = mid;
            let new_mid = ((right - left) / 2) as f64;
            mid += new_mid.ceil() as i64
        } else {
            //go left
            right = mid;
            let new_mid = ((right - left) / 2) as f64;
            mid -= new_mid.ceil() as i64
        }
    }

    unreachable!()
}


fn produce_from_trillion_units_of_ore_alternate(reactions: &Vec<Reaction>) -> i64 {
    let leftover = &mut HashMap::new();
    let mut stepwise_ores_used_until_complete_cycle = vec![];
    let one_trillion = 1000000000000;
    let mut cycles_complete_with_no_leftovers = false;
    let mut total_ores_used = 0;
    let mut total_fuel_produced = 0;
    loop {
        let ores_used_this_step = get_ore_requirement(String::from("FUEL"), 1, &reactions, leftover);

        if total_ores_used + ores_used_this_step > one_trillion { break }
        total_ores_used += ores_used_this_step;
        total_fuel_produced += 1;

        if leftover.values().find(|&x| *x != 0).is_none()
            && !cycles_complete_with_no_leftovers {
            let multiplier = one_trillion / total_ores_used;
            total_ores_used *= multiplier;
            total_fuel_produced *= multiplier;
            cycles_complete_with_no_leftovers = true;
        }

        if !cycles_complete_with_no_leftovers {
            stepwise_ores_used_until_complete_cycle.push(total_ores_used);
        }
    }

    total_fuel_produced
}

pub fn main() {
    let contents = include_str!("../../data/fourteen.data");
    let reactions = read_input(contents);
    let leftover = &mut HashMap::new();
    println!("Ore requirement for 1 unit of fuel: {:?}", get_ore_requirement(String::from("FUEL"), 1, &reactions, leftover));
    println!("Fuel generated with 1 trillion units of ore: {:?}", produce_from_trillion_units_of_ore(&reactions));
}

#[cfg(test)]
mod tests {
    use crate::{Reaction, Chemical, read_input, get_ore_requirement, produce_from_trillion_units_of_ore, produce_from_trillion_units_of_ore_alternate};
    use std::collections::HashMap;

    #[test]
    fn gets_requirement_for_producing() {
        assert_eq!(setup_and_get_fuel_requirement(
            "9 ORE => 2 A
            8 ORE => 3 B
            7 ORE => 5 C
            3 A, 4 B => 1 AB
            5 B, 7 C => 1 BC
            4 C, 1 A => 1 CA
            2 AB, 3 BC, 4 CA => 1 FUEL"), 165);

        assert_eq!(setup_and_get_fuel_requirement(
            "157 ORE => 5 NZVS
            165 ORE => 6 DCFZ
            44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
            12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
            179 ORE => 7 PSHF
            177 ORE => 5 HKGWZ
            7 DCFZ, 7 PSHF => 2 XJWVT
            165 ORE => 2 GPVTF
            3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"), 13312);

        assert_eq!(setup_and_get_fuel_requirement(
            "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
            17 NVRVD, 3 JNWZP => 8 VPVL
            53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
            22 VJHF, 37 MNCFX => 5 FWMGM
            139 ORE => 4 NVRVD
            144 ORE => 7 JNWZP
            5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
            5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
            145 ORE => 6 MNCFX
            1 NVRVD => 8 CXFTF
            1 VJHF, 6 MNCFX => 4 RFSQX
            176 ORE => 6 VJHF"), 180697);

        assert_eq!(setup_and_get_fuel_requirement(
            "171 ORE => 8 CNZTR
            7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
            114 ORE => 4 BHXH
            14 VRPVC => 6 BMBT
            6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
            6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
            15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
            13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
            5 BMBT => 4 WPTQ
            189 ORE => 9 KTJDG
            1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
            12 VRPVC, 27 CNZTR => 2 XDBXC
            15 KTJDG, 12 BHXH => 5 XCVML
            3 BHXH, 2 VRPVC => 7 MZWV
            121 ORE => 7 VRPVC
            7 XCVML => 6 RJRHP
            5 BHXH, 4 VRPVC => 5 LTCX"), 2210736);
    }

    #[test]
    fn gets_fuel_produced_from_trillion_ore () {

        //ALTERNATE SOLUTION
        assert_eq!(setup_and_get_fuel_production_from_trillion_ores_alternate(
            "157 ORE => 5 NZVS
            165 ORE => 6 DCFZ
            44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
            12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
            179 ORE => 7 PSHF
            177 ORE => 5 HKGWZ
            7 DCFZ, 7 PSHF => 2 XJWVT
            165 ORE => 2 GPVTF
            3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"), 82892753);

        assert_eq!(setup_and_get_fuel_production_from_trillion_ores(
            "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
            17 NVRVD, 3 JNWZP => 8 VPVL
            53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
            22 VJHF, 37 MNCFX => 5 FWMGM
            139 ORE => 4 NVRVD
            144 ORE => 7 JNWZP
            5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
            5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
            145 ORE => 6 MNCFX
            1 NVRVD => 8 CXFTF
            1 VJHF, 6 MNCFX => 4 RFSQX
            176 ORE => 6 VJHF"), 5586022);

        assert_eq!(setup_and_get_fuel_production_from_trillion_ores(
            "171 ORE => 8 CNZTR
            7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
            114 ORE => 4 BHXH
            14 VRPVC => 6 BMBT
            6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
            6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
            15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
            13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
            5 BMBT => 4 WPTQ
            189 ORE => 9 KTJDG
            1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
            12 VRPVC, 27 CNZTR => 2 XDBXC
            15 KTJDG, 12 BHXH => 5 XCVML
            3 BHXH, 2 VRPVC => 7 MZWV
            121 ORE => 7 VRPVC
            7 XCVML => 6 RJRHP
            5 BHXH, 4 VRPVC => 5 LTCX"), 460664);
    }

    fn setup_and_get_fuel_requirement(contents: &str) -> i64 {
        let reactions = read_input(contents);
        get_ore_requirement(String::from("FUEL"), 1, &reactions, &mut HashMap::new())
    }

    fn setup_and_get_fuel_production_from_trillion_ores(contents: &str) -> i64 {
        let reactions = read_input(contents);
        produce_from_trillion_units_of_ore(&reactions)
    }

    fn setup_and_get_fuel_production_from_trillion_ores_alternate(contents: &str) -> i64 {
        let reactions = read_input(contents);
        produce_from_trillion_units_of_ore_alternate(&reactions)
    }

}