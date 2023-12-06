fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn extract_number(line: &str) -> i64 {
    let mut first: i64 = -1;
    let mut second: i64 = -1;
    for c in line.chars() {
        if c.is_numeric() {
            if first < 0 {
                first = c.to_digit(10).unwrap().into(); 
                second = first;
            } else {
                second = c.to_digit(10).unwrap().into();
            }
        }
    }
    first * 10 + second
}

fn part1(input: &str) -> i64 {
    input.lines().map(extract_number).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1("1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet");
        assert_eq!(result, 142);
    }

}


