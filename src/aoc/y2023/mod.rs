use advent_of_code_macros::match_module;

match_module!("day[0-9]+", fn(String) -> String);

fn empty(_input: String) -> String {
    todo!("This challenge is not yet completed");
}

pub fn get_solutions() -> [&'static fn(String) -> String; 31] {
    [
        &(day1::day1 as fn(String) -> String),
        &(day2::day2 as fn(String) -> String),
        &(day3::day3 as fn(String) -> String),
        &(day4::day4 as fn(String) -> String),
        &(empty as fn(String) -> String),
        &(empty as fn(String) -> String),
        &(empty as fn(String) -> String),
        &(empty as fn(String) -> String),
        &(empty as fn(String) -> String),
        &(empty as fn(String) -> String),
        &(empty as fn(String) -> String),
        &(empty as fn(String) -> String),
        &(empty as fn(String) -> String),
        &(empty as fn(String) -> String),
        &(empty as fn(String) -> String),
        &(empty as fn(String) -> String),
        &(empty as fn(String) -> String),
        &(empty as fn(String) -> String),
        &(empty as fn(String) -> String),
        &(empty as fn(String) -> String),
        &(empty as fn(String) -> String),
        &(empty as fn(String) -> String),
        &(empty as fn(String) -> String),
        &(empty as fn(String) -> String),
        &(empty as fn(String) -> String),
        &(empty as fn(String) -> String),
        &(empty as fn(String) -> String),
        &(empty as fn(String) -> String),
        &(empty as fn(String) -> String),
        &(empty as fn(String) -> String),
        &(empty as fn(String) -> String),
    ]
}
