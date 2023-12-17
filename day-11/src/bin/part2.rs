fn main(){
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Space {
    Galaxy,
    Empty
}

fn step_distance(start: (isize, isize), end: (isize, isize)) -> isize {
    (start.0 - end.0).abs() + (start.1 - end.1).abs()
}

fn part2(input: &str) -> isize{
    let graph = input.lines()
                     .map(|line| line.chars().map(|c| {
                        match c {
                            '#' => Space::Galaxy,
                            '.' => Space::Empty,
                            _ => unreachable!()
                        }
                     }).collect::<Vec<_>>())
                     .collect::<Vec<_>>();

    let mut galaxies: Vec<(isize, isize)> = graph.iter().enumerate().flat_map(|(r, line)| 
        line.iter().enumerate().filter_map(move |(c, v)| {
            match v {
                Space::Galaxy => Some((r as isize, c as isize)),
                Space::Empty => None
            }
        })
    ).collect();
    
    let galaxy_count = galaxies.len();
    let increment = 999_999;
    
    //Iterate over the rows; whenever we find an empty row, increment subsequent galaxy row coords
    graph.iter().enumerate().fold(0, |skew, (row, line)| {
        if line.iter().all(|x| *x == Space::Empty){
            galaxies = galaxies.iter().map(|(r, c)| {
                if *r > row as isize + skew  {
                    (*r + increment, *c)
                } else {
                    (*r, *c)
                }
            }).collect();
            skew + increment
        } else {
            skew
        }
    });
    //Iterate over the cols; whenever we find an empty col, increment subsequent galaxy col coords
    (0..graph[0].len()) // transpose 
        .map(|i| graph.iter().map(|line| line[i]).collect::<Vec<Space>>())
        .enumerate().fold(0, |skew, (col, line): (usize, Vec<Space>)|{
            if line.iter().all(|x| *x == Space::Empty){
                galaxies = galaxies.iter().map(|(r, c)| {
                if *c > col as isize + skew {
                    (*r, *c + increment)
                } else {
                    (*r, *c)
                }
                }).collect();
                skew + increment
            } else {
                skew
            }
        });

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
        let result = part2(input);
        assert_eq!(1030, result);
    }
}