use std::collections::HashMap;

pub fn next_valid_password(num: i64) -> i64 {
    let (_,
        has_two_same_adjacent_digits,
        has_digits_with_non_decreasing_value) = is_valid(num);

    match (has_digits_with_non_decreasing_value, has_two_same_adjacent_digits) {
        (true, false) => {
            return num + 10;
        }
        (false, _) => {
            return repair_non_decreasing_order_of_values(num);
        }
        (true, true) => { num }
    }
}

pub fn repair_non_decreasing_order_of_values(num: i64) -> i64 {
    let mut digits: Vec<u32> = num.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();

    let mut previous_digit = digits[0];
    let mut start_repairing = false;
    let updated_digits = digits.iter_mut().map(|digit| {
        if start_repairing {
            return previous_digit;
        } else if previous_digit > *digit {
            start_repairing = true;
            return previous_digit;
        }
        previous_digit = *digit;
        return *digit;
    });

    updated_digits.fold(String::from(""), |agg: String, x| -> String {
        format!("{}{}", agg, x.to_string())
    }).parse::<i64>().unwrap()
}

pub fn has_exactly_two_same_adjacent_digits (num : i64) -> bool {
    let mut digits: Vec<u32> = num.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
    let count_map: HashMap<u32, u32> = digits.iter().fold(HashMap::new(), |mut map:  HashMap<u32, u32>, x| -> HashMap<u32, u32> {
        if map.get(x).is_some() {
            map.insert(*x, map.get(x).unwrap() + 1);
        } else {
            map.insert(*x, 1);
        }
        map
    });

    count_map.values().find(|x| **x == 2 as u32).is_some()
}

pub fn is_valid(num: i64) -> (bool, bool, bool) {
    let mut last_digit: i8 = (num % 10) as i8;
    let mut remaining_digits: i64 = num / 10;

    let mut number_of_digits = 1;
    let mut has_two_same_adjacent_digits = false;
    let mut has_digits_with_non_decreasing_value = true;

    while remaining_digits > 0 {
        let next_digit: i8 = (remaining_digits % 10) as i8;
        if next_digit == last_digit {
            has_two_same_adjacent_digits = true;
        }

        has_digits_with_non_decreasing_value =
            has_digits_with_non_decreasing_value
                && last_digit >= next_digit;

        number_of_digits = number_of_digits + 1;
        last_digit = next_digit;
        remaining_digits = remaining_digits / 10;
    }

    (number_of_digits == 6, has_two_same_adjacent_digits, has_digits_with_non_decreasing_value)
}

pub fn main() {
    let mut stop = false;
    let mut starting_range = 357253;
    let mut all_possible_passwords: Vec<i64> = Vec::new();
    while !stop {
        let next_valid = next_valid_password(starting_range);
        if next_valid <= 892942 {
            starting_range = next_valid + 1;
            all_possible_passwords.push(next_valid);
        } else {
            stop = true;
        }
    }
    println!("All possible {} passwords {}", all_possible_passwords.len(), all_possible_passwords.iter()
        .map(|x| x.to_string()).collect::<Vec<String>>().join(","));

    all_possible_passwords = all_possible_passwords.iter()
        .filter(|x| has_exactly_two_same_adjacent_digits(**x)).cloned().collect();
    println!("Possible passwords with additional validation ({}) : {}", all_possible_passwords.len(), all_possible_passwords.iter()
        .map(|x| x.to_string()).collect::<Vec<String>>().join(","));
}

#[cfg(test)]
mod tests {
    use crate::{is_valid, repair_non_decreasing_order_of_values, next_valid_password, has_exactly_two_same_adjacent_digits};

    #[test]
    fn checks_is_valid() {
        assert_eq!(is_valid(111111), (true, true, true));
        assert_eq!(is_valid(223450), (true, true, false));
        assert_eq!(is_valid(123789), (true, false, true));
        assert_eq!(is_valid(12378), (false, false, true));
    }

    #[test]
    fn should_repair_increasing_order_of_values() {
        assert_eq!(repair_non_decreasing_order_of_values(25933), 25999);
        assert_eq!(repair_non_decreasing_order_of_values(25999), 25999);
        assert_eq!(repair_non_decreasing_order_of_values(10000), 11111);
        assert_eq!(repair_non_decreasing_order_of_values(11121), 11122);
    }

    #[test]
    fn get_next_valid_password() {
        assert_eq!(next_valid_password(357253), 357777);
        assert_eq!(next_valid_password(223450), 223455);
        assert_eq!(next_valid_password(123789), 123799);
        assert_eq!(next_valid_password(123781), 123788);
    }

    #[test]
    fn check_for_exactly_two_same_adjacent_digits() {
        assert_eq!(has_exactly_two_same_adjacent_digits(112233), true);
        assert_eq!(has_exactly_two_same_adjacent_digits(123444), false);
        assert_eq!(has_exactly_two_same_adjacent_digits(111122), true);
    }
}