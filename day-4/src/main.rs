use std::collections::{HashSet, HashMap};

fn main() {
    let input = include_str!("./input.txt");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}

#[derive(Debug)]
struct Card {
    winning_numbers: HashSet<u32>,
    my_numbers: HashSet<u32>
}

fn part1(input: &str) -> u32 {

    input.lines()
         .map(|line| {

            let card = parse_scratchcard(line);
            let matches = card.my_numbers.intersection(&card.winning_numbers).count() as u32; 
            
            if matches != 0 {
                2u32.pow(matches - 1)
            } else {
                matches
            }
         })
         .sum()
}

fn part2(input: &str) -> u32 {

    let cards = parse_scratchcards(input);
    let mut copies = (0..cards.len()).map(|i| (i, 1))
                                     .collect::<HashMap<usize, u32>>();

    for (i, card) in cards.iter().enumerate() {

        let mut card_id = i; 
        let n = copies[&i];

        for win in &card.winning_numbers {

            if card.my_numbers.contains(win) {
                card_id += 1;
                if let Some(count) = copies.get_mut(&card_id) {
                    *count += n;
                }
            }
        }
    }
    copies.values().sum()

}

fn parse_scratchcards(input: &str) -> Vec<Card> {

    input.lines() 
         .map(parse_scratchcard)
         .collect()
}

fn parse_scratchcard(line: &str) -> Card {

    let (_, numbers) = line.split_once(':').unwrap();
    let (winning, my) = numbers.split_once('|').unwrap();
    Card {
        winning_numbers: parse_numbers(winning),
        my_numbers: parse_numbers(my)
    }
}

fn parse_numbers(line: &str) -> HashSet<u32> {
    
    line.split_ascii_whitespace()
        .map(|num| num.parse::<u32>().unwrap())
        .collect::<HashSet<u32>>()
}

