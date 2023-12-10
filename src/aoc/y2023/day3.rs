use std::cmp::{max, min};
use itertools::Itertools;

pub fn day3(input: String) -> String{
    let map: Vec<&str> = input
        .lines()
        .collect();
    assert!(map.len() > 0, "Map row is 0");
    let is_symbol = |c: char| {
        // !c.is_digit(10) && c != '.'
        c == '*'
    };
    let is_in_bound = |ext_i: isize, inner_i: isize| {
        inner_i < map[0].len() as isize && inner_i >= 0 && ext_i < map.len() as isize && ext_i >= 0
    };
    let delta: [(isize, isize); 8] = [
        (1,  1), (1, 0),  (1, -1),
        (0,  1),          (0, -1),
        (-1, 1), (-1, 0), (-1, -1),
    ];


    let mut result: Vec<Vec<(usize, usize)>> = Vec::new();
    for vec in &map {
        let mut pairs: Vec<(usize, usize)> = Vec::new();
        let mut start_index: Option<usize> = None;

        for (i, ch) in vec.chars().enumerate() {
            if ch.is_digit(10) {
                if start_index.is_none() {
                    start_index = Some(i);
                }
            } else {
                if let Some(start) = start_index {
                    pairs.push((start, i - 1));
                    start_index = None;
                }
            }
        }

        if let Some(start) = start_index {
            pairs.push((start, vec.len() - 1));
        }

        result.push(pairs);
    }

    let ret = result
        .iter()
        .enumerate()
        .map(|(row_id, row)| {
            let row_map: Vec<Vec<_>> = map.iter().skip(max(row_id, 1) - 1).take(min(3, row_id + 2)).map(|s|s.chars().collect()).collect();
            let row_map_filter = row_map.clone();
            let row_map_map = map[row_id];
            let ret = row
                .iter()
                .filter_map(move |(start, end)| {
                    for ini in *start..*end + 1 {
                        let ini = ini as isize;
                        for (dx, dy) in delta {
                            let (nrid, nini) = (row_id as isize + dx, ini + dy);
                            if is_in_bound(nrid, nini) {
                                let (i, j) = ((dx + ((row_id > 0) as isize)) as usize, nini as usize);
                                if is_symbol((&row_map_filter)[i][j]) {
                                    return Some((*start, *end, nrid, nini, row_id));
                                }
                            }
                        }
                    }
                    return None;
                });
            ret
        })
        .flatten()
        .sorted_by(|(_, _, arid, acid, _), (_, _, brid, bcid, _)|
            (*arid, *acid).cmp(&(*brid, *bcid)))
        .tuple_windows::<(_, _)>()
        .filter(|(a, b)| a.2 == b.2 && a.3 == b.3)
        .collect::<Vec<_>>();
    let ret = ret.iter()
        .map(|(a, b)| ((a.0, a.1, a.4), (b.0, b.1, b.4)) )
        .map(|(a, b)| { // a&b : (start, end, row_id)
            let a = map[a.2][a.0..a.1 + 1].parse::<i64>().unwrap();
            let b = map[b.2][b.0..b.1 + 1].parse::<i64>().unwrap();
            a * b
        })
        .sum::<i64>()
        .to_string();
    ret
}
