fn main(){
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn next_value(seq: Vec<i32>) -> i32 {
    if seq.iter().all(|x| *x == 0) {
        return 0;
    }
    let mut diffs = vec![seq.iter()
                   .cloned()
                   .enumerate()
                   .skip(1)
                   .map(|(i, n)| n - seq.get(i - 1).expect("always valid index"))
                   .collect::<Vec<i32>>()];


    while diffs.last().unwrap().iter().any(|x| *x != 0){
        let last = diffs.last().unwrap();
        diffs.push(last.iter()
        .cloned()
        .enumerate()
        .skip(1)
        .map(|(i, n)| n - last.get(i - 1).expect("always valid index"))
        .collect::<Vec<i32>>());
    }

    diffs.iter()
        .rev()
        .skip(1)
        .fold(0, |last_diff, s| {
            s.last().unwrap() + last_diff
        })
        + seq.last().unwrap()
}

fn parse_sequence(line: &str) -> Vec<i32>{
    line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect()
}

fn part1(input: &str) -> i32{
    input.lines().map(parse_sequence).map(next_value).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let result = part1(input);
        assert_eq!(114, result);
    }
}