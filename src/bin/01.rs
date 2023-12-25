advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut sum = 0;
    for line in lines {
        let nums = line.chars().fold((None, None), |acc, c| {
            let Some(c) = c.to_digit(10) else {
                return acc;
            };
            match acc {
                (None, _) => (Some(c), Some(c)),
                (Some(a), _) => (Some(a), Some(c)),
            }
        });
        let num = nums.0? * 10 + nums.1?;
        sum += num;
    }
    Some(sum)
}

fn parse_digit(s: &str) -> Option<u32> {
    if let Some(d) = s.chars().next() {
        if let Some(d) = d.to_digit(10) {
            return Some(d);
        }
    }
    for len in 1..=s.len() {
        let sub = &s[..len];
        let num = match sub {
            "one" => Some(1),
            "two" => Some(2),
            "three" => Some(3),
            "four" => Some(4),
            "five" => Some(5),
            "six" => Some(6),
            "seven" => Some(7),
            "eight" => Some(8),
            "nine" => Some(9),
            _ => None,
        };
        if num.is_some() {
            return num;
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut sum = 0;
    for line in lines {
        let first_digit = {
            let mut res = None;
            for start in 0..line.len() {
                let sub = &line[start..];
                if let Some(d) = parse_digit(sub) {
                    res = Some(d);
                    break;
                }
            }
            res
        };
        let last_digit = {
            let mut res = None;
            for start in (0..line.len()).rev() {
                let sub = &line[start..];
                if let Some(d) = parse_digit(sub) {
                    res = Some(d);
                    break;
                }
            }
            res
        };
        sum += first_digit? * 10 + last_digit?;
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
