fn main() {
    let _input = std::fs::read_to_string("packages/aoc_2025_4/input.txt")
        .expect("Failed to read input file");
    let result = solve_1(&_input);
    println!("day4 part 1: {}", result);
}

fn solve_1(_input: &str) -> isize {
    let sum = 0;
    sum
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
        let expected = 13;
        assert_eq!(expected, actual)
    }
}
