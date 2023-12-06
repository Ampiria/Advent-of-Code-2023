use std::{cmp::min, collections::HashSet};

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn get_full_num(grid: &Vec<String>, r: usize, c: usize, 
                    visited: &mut HashSet<(usize, usize)>) -> Option<i32> {
    let mut num = "".to_string();
    for i in (0..c+1).rev(){
        if visited.contains(&(r,i)) {
            return None;
        }
        let d = grid.get(r).unwrap().chars().nth(i).unwrap();
        if d.is_numeric() {
            num.insert(0, d);
            visited.insert((r,i));
        } else {
            break;
        }
    }
    for i in c+1..grid.get(r).unwrap().len() {
        if visited.contains(&(r,i)) {
            return None;
        }
        let d = grid.get(r).unwrap().chars().nth(i).unwrap();
        if d.is_numeric() {
            num.push(d);
            visited.insert((r,i));
        } else {
            break;
        }
    }
    Some(num.parse::<i32>().unwrap())
}

fn gear_ratio(grid: &Vec<String>, r: usize, c: usize) -> i32 {
    let mut visited = HashSet::new();
    let mut adjacent_count = 0;
    let mut gear_ratio = 1;
    let rmin = if r == 0 {0} else {r - 1};
    let cmin = if c == 0 {0} else {c - 1};
    let rmax = min(grid.len() , r + 2);
    let cmax = min(grid.get(0).unwrap().len() , c + 2);
    
    for i in rmin..rmax {
        for j in cmin..cmax {
            let d = grid.get(i ).unwrap().chars().nth(j).unwrap();
            
            if d.is_numeric() {
                match get_full_num(grid, i , j , &mut visited) {
                    Some(x) => if adjacent_count > 1 {
                                        gear_ratio = 0;
                                    } else {
                                        gear_ratio = gear_ratio * x; 
                                        adjacent_count += 1;
                                    }
                    None => ()
                }
            }
        }
    }
    if adjacent_count != 2 {
        return 0;
    }
    gear_ratio
} 

fn part2(input: &str) -> i32 {
    let grid = input.lines().map(|line| line.trim().to_string())
                                .collect::<Vec<String>>();
    let mut sum = 0;
    for i in 0..grid.len() {
        let line = grid.get(i).unwrap();
        for j in 0..line.len() {
            sum += match grid.get(i).unwrap().chars().nth(j) {
                        Some('*') => gear_ratio(&grid, i , j ),
                        _ => 0
                    };
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let result = part2("467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..");
        assert_eq!(result, 467835);
    }

}


