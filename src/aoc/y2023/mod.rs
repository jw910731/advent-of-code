mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day26;
mod day27;
mod day28;
mod day29;
mod day30;
mod day31;

pub fn get_solutions() -> [&'static fn(String) -> String; 31] {
    [
        &(day1::day1 as fn(String) -> String),
        &(day2::day2 as fn(String) -> String),
        &(day3::day3 as fn(String) -> String),
        &(day4::day4 as fn(String) -> String),
        &(day5::day5 as fn(String) -> String),
        &(day6::day6 as fn(String) -> String),
        &(day7::day7 as fn(String) -> String),
        &(day8::day8 as fn(String) -> String),
        &(day9::day9 as fn(String) -> String),
        &(day10::day10 as fn(String) -> String),
        &(day11::day11 as fn(String) -> String),
        &(day12::day12 as fn(String) -> String),
        &(day13::day13 as fn(String) -> String),
        &(day14::day14 as fn(String) -> String),
        &(day15::day15 as fn(String) -> String),
        &(day16::day16 as fn(String) -> String),
        &(day17::day17 as fn(String) -> String),
        &(day18::day18 as fn(String) -> String),
        &(day19::day19 as fn(String) -> String),
        &(day20::day20 as fn(String) -> String),
        &(day21::day21 as fn(String) -> String),
        &(day22::day22 as fn(String) -> String),
        &(day23::day23 as fn(String) -> String),
        &(day24::day24 as fn(String) -> String),
        &(day25::day25 as fn(String) -> String),
        &(day26::day26 as fn(String) -> String),
        &(day27::day27 as fn(String) -> String),
        &(day28::day28 as fn(String) -> String),
        &(day29::day29 as fn(String) -> String),
        &(day30::day30 as fn(String) -> String),
        &(day31::day31 as fn(String) -> String),
    ]
}
