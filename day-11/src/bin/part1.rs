use std::vec::IntoIter;
fn main(){
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Space {
    Galaxy,
    Empty
}

fn expand(line: Vec<Space>) -> IntoIter<Vec<Space>>{
    if line.iter().all(|x| *x == Space::Empty) {
        let copy = line.clone();
        vec![line, copy].into_iter()
    } else {
        vec![line].into_iter()
    }
}

fn step_distance(start: (isize, isize), end: (isize, isize)) -> isize {
    (start.0 - end.0).abs() + (start.1 - end.1).abs()
}

fn part1(input: &str) -> isize{
    let mut graph = input.lines()
                     .map(|line| line.chars().map(|c| {
                        match c {
                            '#' => Space::Galaxy,
                            '.' => Space::Empty,
                            _ => unreachable!()
                        }
                     }).collect::<Vec<_>>())
                     .collect::<Vec<_>>()
                     .into_iter()
                     .flat_map(expand).collect::<Vec<_>>();
    graph = (0..graph[0].len()) // transpose and expand the empty columns
        .map(|i| graph.iter().map(|line| line[i]).collect())
        .flat_map(expand)
        .collect::<Vec<Vec<Space>>>().into_iter()
        .collect();

    graph = (0..graph[0].len()) // transpose 
                .map(|i| graph.iter().map(|line| line[i]).collect())
                .collect();

    let galaxies: Vec<(isize, isize)> = graph.iter().enumerate().flat_map(|(r, line)| 
        line.iter().enumerate().filter_map(move |(c, v)| {
            match v {
                Space::Galaxy => Some((r as isize, c as isize)),
                Space::Empty => None
            }
        })
    ).collect();
    
    let galaxy_count = galaxies.len();

    (0..galaxy_count)
        .map(|i| {
            (i+1..galaxy_count)
                .map(|j| step_distance(galaxies[i], galaxies[j]))
                .sum::<isize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        let result = part1(input);
        assert_eq!(374, result);
    }
}