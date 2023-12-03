use regex::Regex;

fn main() {
    let input1 = include_str!("./input1.txt");
    let games = parse_games(input1);
    println!("Solution to part1: {}", part1(&games, &MAX_HAND));
    println!("Solution to part2: {}", part2(&games))
}

struct Game {
    id: i32,
    rounds: Vec<ElfHand>
}

struct ElfHand {
    red: i32,
    green: i32,
    blue: i32
}

const MAX_HAND: ElfHand = ElfHand {
    red: 12,
    green: 13,
    blue: 14
};

fn part1(games: &Vec<Game>, max_hand: &ElfHand) -> i32{

    games.iter()
         .filter(|game| is_valid_game(game, max_hand))
         .map(|game| game.id)
         .sum()
}

fn is_valid_game(game: &Game, max_hand: &ElfHand) -> bool {

    game.rounds.iter()
               .all(|hand| 
                    hand.red <= max_hand.red && hand.green <= max_hand.green && hand.blue <= max_hand.blue)
} 

fn part2(games: &Vec<Game>) -> i32 {

    let mut total = 0;
    for game in games {

        total += get_power(&game.rounds)
    }

    total
}

fn get_power(hands: &Vec<ElfHand>) -> i32{

    let mut max_hand = ElfHand {red: 0, green: 0, blue: 0};

    for hand in hands {

        if hand.red > max_hand.red {
            max_hand.red = hand.red;
        }
        if hand.green > max_hand.green {
            max_hand.green = hand.green;
        }
        if hand.blue > max_hand.blue {
            max_hand.blue = hand.blue;
        }
    }

    max_hand.red * max_hand.green * max_hand.blue
}

fn parse_games(input: &str) -> Vec<Game> {

    let regex = Regex::new(r"^Game (\d+): (.+)$").unwrap();
    input.lines()
         .map(|line| {
                let captures = regex.captures(line).unwrap();
                let game_id = captures[1].parse::<i32>().unwrap();
                let hands: Vec<ElfHand> = captures[2].split("; ")
                                                     .map(parse_hand)
                                                     .collect();
                Game {
                    id: game_id,
                    rounds: hands
                }
         }).collect()
}

fn parse_hand(hand: &str) -> ElfHand {

    let mut elf_hand = ElfHand {
        red: 0,
        green: 0,
        blue: 0
    };

    for cubes in hand.split(", ") {

        let partitions: Vec<&str> = cubes.split(" ").collect();
        let num = partitions[0].parse::<i32>().unwrap();
        let color = partitions[1];

        match color {
            "red" => elf_hand.red = num,
            "green" => elf_hand.green = num,
            "blue" => elf_hand.blue = num,
            _ => panic!("unknown colour")
        }
    }

    elf_hand
}


