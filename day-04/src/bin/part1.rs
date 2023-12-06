use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

struct Card {
    winning_nums: HashSet<i32>,
    my_nums: Vec<i32>
}

impl Card {
    fn new(line: &str) -> Card {
        let mut nums = line.trim().split(": ").nth(1).expect("invalid line")
                                             .split(" | ");
        let win_str = nums.next().expect("missing winning nums").trim();
        let num_str = nums.next().expect("missing my nums").trim();
        Card { 
            winning_nums: win_str.split(' ').filter(|x| x.len() > 0)
                                 .map(|x| x.parse().unwrap()).collect(), 
            my_nums: num_str.split(' ').filter(|x| x.len() > 0)
                            .map(|x| x.parse().unwrap()).collect() 
        }
    }

    fn winning_nums_count(self) -> u32 {
        self.my_nums.iter().filter(|x| self.winning_nums.contains(*x))
                                       .collect::<Vec<&i32>>().len() as u32
    }

    fn point_value(self) -> i32 {
        match self.winning_nums_count() {
            0 => 0,
            c => 2_i32.pow(c - 1)
        }
    }
}


fn part1(input: &str) -> i32 {
    input.lines().map(Card::new).map(|c| c.point_value()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11");
        assert_eq!(result, 13);
    }

}


