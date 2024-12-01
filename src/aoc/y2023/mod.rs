mod day1;
mod day2;
mod day3;
mod day4;

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
