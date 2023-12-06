use std::cmp::max;

fn main(){
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn draw_counts(draw: &str) -> (i64, i64, i64) {
    let split: Vec<&str> = draw.split(" ").collect();
    let num = split.get(0).unwrap().parse::<i64>().unwrap();
    let color = *split.get(1).unwrap();
    match (num, color) {
        (c, "red") => (c, 0, 0),
        (c, "green") => (0, c, 0),
        (c, "blue") => (0, 0, c),
        _ => panic!("invalid color draw {:?}", draw)
    }
}

fn color_counts(hand: &str) -> (i64, i64, i64) {
    hand.split(',').map(|draw| draw_counts(draw.trim())).fold((0, 0, 0), max_set)
}

fn max_set(acc: (i64, i64, i64), x: (i64, i64, i64)) -> (i64, i64, i64) {
    let (ar, ag, ab) = acc;
    let (r, g, b) = x;
    (max(ar, r), max(ag, g), max(ab, b))
}

fn game_power(line: &str) -> i64 {
    let hands = *line.split(':').collect::<Vec<&str>>().get(1).unwrap();
    let (mr, mg, mb) = hands.split(';').map(color_counts).fold((0, 0, 0), max_set);
    mr * mg * mb
}

fn part2(input: &str) -> i64{
    input.lines().map(|line| game_power(line.trim())).sum()
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
        let result = part2(input);
        assert_eq!(2286, result);
    }
}