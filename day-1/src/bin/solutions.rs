fn main() {
    let input = include_str!("./input1.txt");
    println!("Solution to part1: {}", part1(input));
    println!("eolution to part2: {}", part2(input));
}

fn part1(input: &str) -> String {

    let output = input.lines()
                           .map(filter_first_and_last)
                           .sum::<u32>();
    output.to_string()
}

fn part2(input: &str) -> u32 {

    input.replace("one", "o1e")
         .replace("two", "t2o")
         .replace("three", "t3e")
         .replace("four", "f4r")
         .replace("five", "f5e")
         .replace("six", "s6x")
         .replace("seven", "s7n")
         .replace("eight", "e8t")
         .replace("nine", "n9e")
         .lines()
         .map(filter_first_and_last)
         .sum()
}

pub fn filter_first_and_last(string: &str) -> u32 {

    let mut digits = string.chars().filter(|c| c.is_ascii_digit());
    let first = digits.next().unwrap();
    let last = digits.last();

    match last {
        Some(digit) => format!("{first}{digit}"),
        None => format!("{first}{first}")
    }.parse::<u32>().unwrap()
}

