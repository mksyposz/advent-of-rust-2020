use itertools::{Itertools, MinMaxResult};
use std::io::{self, BufRead};

const PREAMBLE_SIZE: usize = 25;

fn main() {
    let numbers = read_numbers(io::stdin().lock());
    let target = part1(&numbers);
    println!("Day 9, part 1: {}", target);
    println!("Day 9, part 2: {}", part2(target, &numbers));
}

fn part1(numbers: &[i32]) -> i32 {
    *numbers
        .iter()
        .skip(PREAMBLE_SIZE)
        .enumerate()
        .find(|(i, n)| !is_sum_of_previous(**n, &numbers[*i..*i + PREAMBLE_SIZE]))
        .expect("Solution not found!")
        .1
}

fn part2(target: i32, numbers: &[i32]) -> i32 {
    let result = numbers
        .iter()
        .enumerate()
        .filter_map(|(i, _)| find_exact_sum_of_previous(target, &numbers[..i]))
        .next()
        .expect("Solution not found!");
    result.0 + result.1
}

fn find_exact_sum_of_previous(n: i32, numbers: &[i32]) -> Option<(i32, i32)> {
    let mut sum = 0;
    let mut v: Vec<i32> = Vec::new();
    for num in numbers.iter().rev() {
        sum += num;
        v.push(*num);

        if sum > n {
            return None;
        }

        if sum == n {
            if let MinMaxResult::MinMax(min, max) = v.iter().minmax() {
                return Some((*min, *max));
            } else {
                panic!("fiasco!");
            }
        }
    }

    None
}

fn is_sum_of_previous(n: i32, numbers: &[i32]) -> bool {
    let sorted_numbers = numbers.iter().sorted().collect::<Vec<_>>();

    numbers
        .iter()
        .any(|num| sorted_numbers.binary_search(&&(n - num)).is_ok())
}

fn read_numbers<R: BufRead>(reader: R) -> Vec<i32> {
    reader
        .lines()
        .filter_map(Result::ok)
        .filter_map(|i| i.parse::<i32>().ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_solution() {
        let numbers = read_numbers(BufReader::new(File::open("inputs/day9/1.txt").unwrap()));
        let part1_solution = part1(&numbers);
        assert_eq!(part1_solution, 177777905);
        assert_eq!(part2(part1_solution, &numbers), 23463012);
    }
}
