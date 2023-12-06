fn main(){
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn valid_draw(draw: &str) -> i64 {
    let split: Vec<&str> = draw.split(" ").collect();
    let num = split.get(0).unwrap().parse::<i64>().unwrap();
    let color = *split.get(1).unwrap();
    match (num, color) {
        (c, "red") => if c <= 12 {1} else {0},
        (c, "green") => if c <= 13 {1} else {0},
        (c, "blue") => if c <= 14 {1} else {0},
        _ => panic!("invalid color draw {:?}", draw)
    }
}

fn hand_possible(hand: &str) -> i64 {
    hand.split(',').map(|draw| valid_draw(draw.trim())).fold(1, |acc, x| acc * x)
}

fn game_possible(line: &str) -> i64 {
    let id_split: Vec<&str> = line.split(':').collect();
    let id = match id_split.get(0).unwrap().split(" ").collect::<Vec<&str>>().get(1) {
        Some(num) => num.parse::<i64>().unwrap(),
        _ => panic!("Game id invalid {:?}", line)
    };
    let hands_valid = id_split.get(1).unwrap().split(';').map(hand_possible).fold(1, |acc, x| acc * x);
    id * hands_valid
}

fn part1(input: &str) -> i64{
    input.lines().map(|line| game_possible(line.trim())).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = part1(input);
        assert_eq!(8, result);
    }
}