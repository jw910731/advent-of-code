pub fn day1(input: String)->String{
    let numbers = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    input.lines().map(|_line|{
        let mut line = "".to_string();
        _line.clone_into(&mut line);
        let occurrence: Vec<_> = numbers
            .iter()
            .enumerate()
            .flat_map(|(i, x)|
                line
                    .match_indices(x)
                    .map(|(index, _)| (index, i))
                    .collect::<Vec<_>>())
            .collect();
        let mut digits = line
            .chars()
            .enumerate()
            .map(|(idx, c)| {
                let occur = occurrence
                    .iter()
                    .find(|(i, _)| *i == idx);
                if let Some((_, n)) = occur {
                    n.to_string().chars().next().unwrap()
                } else {
                    c
                }
            })
            .filter(|c| c.is_ascii_digit());
        let first: i32 = digits.next().unwrap().to_string().parse().unwrap();
        let last: i32 = digits.last().map(|c| c.to_string().parse().unwrap()).unwrap_or_else(||first);
        first*10+last
    }).sum::<i32>().to_string()
}
