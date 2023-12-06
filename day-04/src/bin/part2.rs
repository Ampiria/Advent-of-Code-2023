use std::collections::{HashSet, BTreeMap};

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

#[derive(Debug)]
struct Card {
    winning_nums: HashSet<i32>,
    my_nums: Vec<i32>,
    copy_count: u32,
}

impl Card {
    fn new(line: &str) -> Card {
        let mut nums = line.split(" | ");
        let win_str = nums.next().expect("missing winning nums");
        let num_str = nums.next().expect("missing my nums");
        Card { 
            winning_nums: win_str.split_whitespace().filter(|x| x.len() > 0)
                                 .map(|x| x.parse().unwrap()).collect(), 
            my_nums: num_str.split_whitespace().filter(|x| x.len() > 0)
                            .map(|x| x.parse().unwrap()).collect(),
            copy_count: 1,
        }
    }

    fn count_winning_nums(&self) -> u32{
        self.my_nums.iter().filter(|x| self.winning_nums.contains(*x))
                           .collect::<Vec<&i32>>().len() as u32
    }
}


fn part2(input: &str) -> u32 {
    let mut cards = input.lines().map(|line| {
        let mut id_split = line.trim().split(": ");
        let id = id_split.next().expect("invalid game id").split_whitespace().nth(1)
                              .expect("invalid id").parse::<u32>().expect("id not int");
        let line = id_split.next().expect("invalid line");
        (id, Card::new(line))
    } ).collect::<BTreeMap<u32, Card>>();

    let update_vec = cards.iter().map(|(id, c)| {
        c.count_winning_nums();
        (*id, c.count_winning_nums())
    }).collect::<Vec<(u32,u32)>>();

    update_vec.iter().map(|(id, wc)| {
        let start = *id + 1;
        let end = start + wc;
        let cc = cards.get(id).unwrap().copy_count;
        cards.iter_mut().filter(|(nid, _)| (start..end).contains(*nid))
                        .for_each(|(_, c)| c.copy_count += cc);
        cc
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let result = part2("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11");
        assert_eq!(result, 30);
    }

}


