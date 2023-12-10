use itertools::Itertools;

pub fn day4(input: String) -> String {
    input
        .lines()
        .map(|line| {
            let (winning_numbers, owned_numbers) = line
                .split(":")
                .last()
                .unwrap()
                .split("|")
                .map(|it| {
                    it.split_whitespace()
                        .map(str::parse::<i32>)
                        .map(Result::unwrap)
                        .collect::<Vec<_>>()
                })
                .collect_tuple()
                .unwrap();
            let winning_points = owned_numbers
                .iter()
                .filter(|x| winning_numbers.contains(x))
                .collect_vec()
                .len() as i32;
            if winning_points > 0 {
                1 << winning_points - 1
            } else {
                0
            }
        })
        .sum::<i32>()
        .to_string()
}
