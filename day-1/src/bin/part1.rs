fn main() {
    let input = include_str!("./input1.txt");
    let result = part1(input);
    println!("{result}");
}

fn part1(input: &str) -> String {

    let output = input.lines()
                           .map(filter_first_and_last)
                           .sum::<u32>();
    output.to_string()
}

fn filter_first_and_last(string: &str) -> u32 {

    let mut digits = string.chars().filter(|c| c.is_ascii_digit());
    let first = digits.next().unwrap();
    let last = digits.last();
    match last {
        Some(digit) => format!("{first}{digit}"),
        None => format!("{first}{first}")
    }.parse::<u32>().unwrap()
}

