fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn contains_digit(word: &str) -> Option<i64> {
    let digits = [
        ("one", 1), ("two", 2), ("three", 3),
        ("four", 4), ("five", 5), ("six", 6),
        ("seven", 7), ("eight", 8), ("nine", 9)];
    
    for (w,d) in digits {
        if word.contains(w) {
            return Some(d);
        }
    }

    None
}

fn first_digit(line: &str, contains: fn(&str) -> Option<i64>) -> i64 {
    for (i,c) in line.char_indices(){
        if let Some(digit) = contains(&line[0..i+1]) {
            return digit;
        }
        if c.is_numeric() {
           return c.to_digit(10).unwrap().into();
        }
    }

    0
}

fn reverse(word: &str) -> String {
    word.chars().rev().collect::<String>()
}

fn extract_number(line: &str) -> i64 {
    let first = first_digit(line, contains_digit);
    let second = first_digit(&reverse(line), |word| contains_digit(&reverse(word)));

    first * 10 + second
}

fn part2(input: &str) -> i64 {
    input.lines().map(|line| extract_number(line.trim())).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let result = part2(
            "two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen");
        assert_eq!(result, 281);
    }

}


