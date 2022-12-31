#![feature(test)]
extern crate test;

use std::collections::HashSet;

const INPUTS: [&str; 2] = [include_str!("./sample.txt"), include_str!("./input.txt")];

fn parse(input: &'static str) -> Vec<char> {
    input.trim().chars().collect()
}

fn solution(input: Vec<char>) -> usize {
    let mut set = HashSet::new();
    let (mut sx, mut sy) = (0, 0);

    for c in input {
        match c {
            'v' => sx += 1,
            '>' => sy += 1,
            '^' => sx -= 1,
            '<' => sy -= 1,

            _ => (),
        }

        set.insert((sx, sy));
    }

    set.len()
}

fn main() {
    for input in INPUTS.iter() {
        let output = parse(input);
        let output = solution(output);
        println!("{output}");
    }
}

#[bench]
fn solution_bench(b: &mut test::Bencher) {
    b.iter(|| {
        let input = parse(INPUTS[1]);
        let result = solution(input);
        test::black_box(result);
    })
}
