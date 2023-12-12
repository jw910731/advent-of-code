use itertools::Itertools;

pub fn day4(input: String) -> String {
    let pt: Vec<_> = input
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
            owned_numbers
                .iter()
                .filter(|x| winning_numbers.contains(x))
                .collect_vec()
                .len() as i32
        })
        .collect();
    let mut result = Vec::default();
    for idx in 0..pt.len() {
        let next_value = (0..idx)
            .rev()
            .zip(pt.iter())
            .map(|(i, p)| (i as i32) < *p)
            .map(|r| r as i32)
            .zip(result.iter())
            .map(|(a, b)| a * (*b))
            .sum::<i32>();
        result.push(next_value + 1);
    }
    result.iter().sum::<i32>().to_string()
}
