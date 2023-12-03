use regex::Regex;

fn main() {
    let input = include_str!("./input1.txt");
    let schematic = parse_schematic(input);
    println!("Solution to part2: {}", part2(&schematic));
}

#[derive(Debug)]
struct Schematic {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>
}

#[derive(Debug)]
struct Number {
    value: u32,
    row: usize,
    start: usize,
    end: usize
}

#[derive(Debug)]
struct Symbol {
    character: char,
    row: usize,
    col: usize
}

fn part2(schematic: &Schematic) -> u32 {

    let mut total = 0;

    for symbol in &schematic.symbols {

        if symbol.character == '*' {

            let matches = get_gear_nums(&symbol, &schematic.numbers);
            if matches.len() == 2 {

                total += matches[0] * matches[1];
            }
        }
    }

    total
}


fn get_gear_nums(symbol: &Symbol, numbers: &Vec<Number>) -> Vec<u32> {

    let mut matches = vec![];
    for num in numbers {

        let row_match = symbol.row == num.row || symbol.row + 1 == num.row || symbol.row - 1 == num.row;
        let mut col_match = false;

        let start = match num.start {
            0 => num.start,
            _ => num.start - 1
        };
        let end = num.end + 1;

        if row_match {
        
            for i in start..=end {

                if i == symbol.col {
                    col_match = true;
                }
            }
        }

        if row_match && col_match {
            matches.push(num.value)
        }
    }
    matches
}

fn parse_schematic(input: &str) -> Schematic {

    let mut schematic = Schematic {numbers: vec![], symbols: vec![]};
    let regex = Regex::new(r"[0-9]+|[^0-9.]").unwrap();

    for (y, line) in input.lines().enumerate() {

        for m in regex.find_iter(line) {

            match m.as_str().parse::<u32>() {
                Ok(number) => {

                    schematic.numbers.push(Number { value: number, row: y, start: m.start(), end: m.end() - 1 });
                },
                Err(_) => {

                    schematic.symbols.push(Symbol { 
                        character: m.as_str().chars().next().unwrap(), 
                        row: y, 
                        col: m.start() 
                    });
                }
            }
        }

    }
    schematic
}
