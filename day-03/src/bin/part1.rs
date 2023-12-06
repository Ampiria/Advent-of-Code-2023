use std::cmp::min;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn symbol_adjacent(grid: &Vec<String>, r: usize, c: usize) -> bool {
    let rmin = if r == 0 {0} else {r - 1};
    let cmin = if c == 0 {0} else {c - 1};
    let rmax = min(grid.len() , r + 2);
    let cmax = min(grid.get(0).unwrap().len() , c + 2);
    for i in rmin..rmax {
        for j in cmin..cmax {
            let c = grid.get(i).unwrap().chars().nth(j).unwrap();
            if c != '.' && !c.is_numeric() {
                return  true;
            }
        }
    }
    return false;
} 

fn part1(input: &str) -> i32 {
    let grid = input.lines().map(|line| line.trim().chars().collect::<String>()).collect::<Vec<String>>();
    let mut part_sum = 0;
    for i in 0..grid.len() {
        let line = grid.get(i).unwrap();
        let mut j = 0;
        while j < line.len(){
            let mut c = line.chars().nth(j).unwrap();
            let mut num: String = "".to_string();
            let mut adjacent = false;
            while c.is_numeric() && j < line.len(){
                num.push(c);
                adjacent = adjacent || symbol_adjacent(&grid, i, j);
                j += 1;
                if j < line.len() {
                    c = line.chars().nth(j).unwrap();
                }
            }
            if adjacent {
                part_sum += num.parse::<i32>().expect("Should be a integer string");
            } else {
                j += 1;
            }
        }
    }
    part_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1("467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..");
        assert_eq!(result, 4361);
    }

}


