use std::{array, collections::HashMap, hash::RandomState};

use advent_of_code_macros::match_module;

match_module!("day[0-9]+", fn(String) -> String);

fn empty(_input: String) -> String {
    todo!("this challenge is not ready")
}

pub fn get_solutions() -> [&'static fn(String) -> String; 31] {
    let mp: HashMap<&str, &fn(String) -> String, RandomState> =
        HashMap::from_iter(EXPORTS.iter().map(|(k, v)| (*k, v)));
    array::from_fn::<_, 31, _>(|i| {
        mp.get(format!("day{}", i+1).as_str())
            .unwrap_or(&&(empty as fn(String) -> String))
            .clone()
    })
}
