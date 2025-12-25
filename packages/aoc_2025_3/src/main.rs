fn main() {
    let _input = std::fs::read_to_string("input.txt").expect("Failed to read input file");
    let result = solve_1(&_input);
    println!("day3 part 1: {}", result);
}

fn solve_1(input: &str) -> isize {
    let digits = 2;
    let mut sum: isize = 0;
    input.lines().for_each(|line| {
        sum += find_max_number(line, digits).parse::<isize>().unwrap();
    });
    sum
}

fn find_max_number(input: &str, digits: usize) -> String {
    if digits == 0 {
        return String::new();
    }
    let mut max = 0;
    let mut index = 0;
    for (i, c) in input.char_indices() {
        if i > input.len() - digits {
            break;
        }
        let digit = c.to_digit(10).unwrap();
        if max < c.to_digit(10).unwrap() {
            max = digit;
            index = i;
        }
    }
    format!(
        "{}{}",
        max,
        find_max_number(&input[index + 1..], digits - 1)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn load_example() -> String {
        std::fs::read_to_string("example.txt").expect("Failed to read example file")
    }

    #[test]
    fn part1_works() {
        let example = load_example();
        let actual = solve_1(&example);
        let expected = 357;
        assert_eq!(expected, actual)
    }
}
