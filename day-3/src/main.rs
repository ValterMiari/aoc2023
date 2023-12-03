use std::collections::{HashMap, HashSet};
use regex::Regex;

fn main() {
    let input = include_str!("./input1.txt");
    let schematic = parse_schematic(input);
    println!("Solution to part1: {}", part1(&schematic));
    //println!("Solution to part2: {}", part2());
}

#[derive(Debug)]
struct Schematic {
    numbers: Vec<Number>,
    symbols: HashMap<usize, HashSet<usize>>
}

#[derive(Debug)]
struct Number {
    value: u32,
    row: usize,
    start: usize,
    end: usize
}

fn part1(schematic: &Schematic) -> u32 {

    schematic.numbers
             .iter()
             .filter(|number| is_part_number(&number, &schematic.symbols))
             .map(|n| n.value)
             .sum()

}

fn parse_schematic(input: &str) -> Schematic {

    let hash_map: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut schematic = Schematic {numbers: vec![], symbols: hash_map};
    let regex = Regex::new(r"[0-9]+|[^0-9.]").unwrap();

    for (y, line) in input.lines().enumerate() {

        for m in regex.find_iter(line) {

            match m.as_str().parse::<u32>() {
                Ok(number) => {

                    schematic.numbers.push(Number { value: number, row: y, start: m.start(), end: m.end() - 1 });
                },
                Err(_) => {

                    let row_set = schematic.symbols.entry(y).or_insert(HashSet::new());
                    row_set.insert(m.range().start);
                }
            }
        }

    }
    schematic
}

fn is_part_number(num: &Number, symbols: &HashMap<usize, HashSet<usize>>) -> bool {

    let start: usize = match num.row {
        0 => 0,
        _ => num.row - 1 
    };
    let end = num.row + 1;

    for i in start..=end {

        let symbols_option = symbols.get(&i);

        if symbols_option.is_some() {
            if has_adjacent_symbol(num, symbols_option.unwrap(), i) {
                return true;
            }
        }
    } 

    return false;
}

fn has_adjacent_symbol(num: &Number, row_symbols: &HashSet<usize>, current_row: usize) -> bool {

    let behind = match num.start {
       0 => num.start,
       _ => num.start - 1 
    };
    let ahead = num.end + 1;

    match current_row.cmp(&num.row) {

        std::cmp::Ordering::Equal => {
            if behind == 0 {
                return row_symbols.contains(&ahead)
            } else {
                return row_symbols.contains(&behind) || row_symbols.contains(&ahead)
            }
        },
        _ => {
            for i in behind..=ahead {
                if row_symbols.contains(&i) {
                    return true; 
                }
            }
            return false;
        }
    }
}